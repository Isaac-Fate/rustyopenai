use std::{ pin::Pin, task::{ Context, Poll } };
use futures::{ Stream, StreamExt };
use bytes::Bytes;
use lazy_static::lazy_static;
use regex::Regex;
use crate::{ OpenAIResult, OpenAIError, error::OpenAIErrorResponse };
use super::ChatCompletionChunk;

lazy_static! {
    static ref STREAM_RESPONSE_CHUNK_RE: Regex = Regex::new(r"^data: \{.*\}\n\n").unwrap();
    static ref STREAM_RESPONSE_TERMINATION_CHUNK_RE: Regex =
        Regex::new(r"^data: \[DONE\]\n\n").unwrap();
    static ref ERROR_RESPONSE_RE: Regex = Regex::new(
        r#"^\{\n  \"error\": \{\n.+\n.+\n.+\n.+\n.+\n\}\n"#
    ).unwrap();
}

pub struct ChatCompletionStream {
    /// The bytes stream from the response.
    bytes_stream: Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>>>>,

    /// The remaining content.
    remaining_content: Option<String>,
}

impl ChatCompletionStream {
    /// Create a new ChatCompletionStream from the response of the request.
    pub fn new(response: reqwest::Response) -> Self {
        Self {
            bytes_stream: Box::pin(response.bytes_stream()),
            remaining_content: None,
        }
    }
}

/// Extract the first chunk from the content,
/// and return the chunk and the remaining content.
fn extract_first_chunk<S: AsRef<str>>(
    content: S
) -> (Option<OpenAIResult<ChatCompletionChunk>>, Option<String>) {
    if let Some(chunk_match) = STREAM_RESPONSE_CHUNK_RE.find(content.as_ref()) {
        // If the response is a chunk, extract the chunk and the remaining content

        // Set remaining content
        let remaining_content = content.as_ref()[chunk_match.end()..].to_string();

        // Get the chunk content
        let chunk_content = chunk_match.as_str();

        // Strip the prefix "data: " and the suffix "\n\n"
        let chunk_content = chunk_content
            .strip_prefix("data: ")
            .unwrap()
            .strip_suffix("\n\n")
            .unwrap();

        // Parse the chunk content
        let chunk = serde_json::from_str(chunk_content).unwrap();

        (Some(Ok(chunk)), Some(remaining_content))
    } else if let Some(error_match) = ERROR_RESPONSE_RE.find(content.as_ref()) {
        // If the response is an error, return the error

        // Set remaining content
        let remaining_content = content.as_ref()[error_match.end()..].to_string();
        let remaining_content = match remaining_content.len() {
            0 => None,
            _ => Some(remaining_content),
        };

        // Get the error content
        let error_content = error_match.as_str();

        // Parse the error content
        let error_response = serde_json::from_str::<OpenAIErrorResponse>(error_content).unwrap();

        (Some(Err(OpenAIError::from(error_response))), remaining_content)
    } else if
        let Some(termination_chunk_match) = STREAM_RESPONSE_TERMINATION_CHUNK_RE.find(
            content.as_ref()
        )
    {
        // The response is the termination chunk

        // Set remaining content
        let remaining_content = content.as_ref()[termination_chunk_match.end()..].to_string();
        let remaining_content = match remaining_content.len() {
            0 => None,
            _ => Some(remaining_content),
        };

        (None, remaining_content)
    } else {
        // No chunk or error is found
        (None, Some(content.as_ref().to_string()))
    }
}

impl Stream for ChatCompletionStream {
    type Item = OpenAIResult<ChatCompletionChunk>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            // Poll the bytes stream
            match self.bytes_stream.poll_next_unpin(cx) {
                Poll::Ready(Some(Ok(bytes))) => {
                    // Convert the received bytes to a string
                    let content = String::from_utf8(bytes.to_vec()).unwrap();

                    // If there is remaining content, append the new content to it
                    let content = match &self.remaining_content {
                        Some(remaining_content) => { format!("{}{}", remaining_content, content) }
                        None => content,
                    };

                    // Extract the first chunk
                    let (chunk, remaining_content) = extract_first_chunk(content);

                    // Update the remaining content
                    self.remaining_content = remaining_content;

                    if let Some(chunk) = chunk {
                        return Poll::Ready(Some(chunk));
                    } else {
                        continue;
                    }
                }
                Poll::Ready(Some(Err(error))) => {
                    // Set the remaining content to None
                    self.remaining_content = None;

                    return Poll::Ready(Some(Err(OpenAIError::from(error))));
                }
                Poll::Ready(None) => {
                    // Continue extracting chunks if there is still some remaining content
                    if let Some(remaining_content) = &self.remaining_content {
                        // Extract the first chunk
                        let (chunk, remaining_content) = extract_first_chunk(remaining_content);

                        // Update the remaining content
                        self.remaining_content = remaining_content;

                        if let Some(chunk) = chunk {
                            return Poll::Ready(Some(chunk));
                        } else {
                            continue;
                        }
                    } else {
                        return Poll::Ready(None);
                    }
                }
                Poll::Pending => {
                    return Poll::Pending;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use futures::StreamExt;
    use crate::{
        OpenAIResult,
        OpenAIClient,
        chat::{ ChatRequestBody, ChatMessage, ChatRole },
        init_logger,
    };
    use super::ChatCompletionStream;

    #[tokio::test]
    async fn get_streamed_chat_response() -> OpenAIResult<()> {
        // Initialize logger
        init_logger();

        // Convert to a map
        let mut request_body = serde_json
            ::to_value(
                &ChatRequestBody::builder()
                    .model("gpt-3.5-turbo")
                    .messages(
                        vec![ChatMessage {
                            role: ChatRole::User,
                            content: r"What is Rust?".to_string(),
                        }]
                    )
                    .temperature(0.0)
                    .build()?
            )
            .unwrap()
            .as_object()
            .unwrap()
            .to_owned();

        // Set the key "stream" to false
        request_body.insert("stream".to_string(), serde_json::json!(true));

        // Create client
        let response = OpenAIClient::builder()
            .timeout(Duration::from_secs(10))
            .build()?
            .post("https://api.openai.com/v1/chat/completions")
            .json(&request_body)
            .send().await?;

        // Get the response bytes stream
        // let response_bytes_stream = resposne?.bytes_stream();
        let mut stream = ChatCompletionStream::new(response);

        // Print all the chunks
        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(chunk) => {
                    println!("{:#?}", chunk.choices.first().unwrap().delta.content);
                }
                Err(error) => {
                    println!("{:#?}", error);
                }
            }
        }

        Ok(())
    }
}
