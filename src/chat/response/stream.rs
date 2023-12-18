use anyhow::Result;
use tracing::error;
use futures::{ Stream, StreamExt };
use std::{ pin::Pin, task::{ Context, Poll } };
use bytes::Bytes;
use lazy_static::lazy_static;
use regex::Regex;
use super::OpenAIChatCompletionChunk;

lazy_static! {
    static ref STREAM_RESPONSE_CHUNK_RE: Regex = Regex::new(r"^data: \{.*\}\n\n").unwrap();
    static ref STREAM_RESPONSE_TERMINATION_CHUNK_RE: Regex =
        Regex::new(r"^data: \[DONE\]\n\n").unwrap();
}

fn extract_first_chunk(content: &str) -> Result<ExtractedChunkWithRemainingContent> {
    if let Some(mat) = STREAM_RESPONSE_CHUNK_RE.find(content) {
        // Matched string
        let matched_str = mat.as_str();

        // Remaining content
        let remaining_content = &content[mat.end()..];

        // Extract the JSON content
        let json_content = matched_str.strip_prefix("data: ").unwrap();

        // Parse the JSON content to OpenAIChatCompletionChunk
        let chunk = serde_json::from_str(json_content)?;

        // Return the extracted response and remaining content
        Ok(ExtractedChunkWithRemainingContent {
            chunk: Some(chunk),

            // If there is no remaining content, return None
            remaining_content: match remaining_content.len() {
                0 => None,
                _ => Some(remaining_content.to_string()),
            },
        })
    } else if let Some(_) = STREAM_RESPONSE_TERMINATION_CHUNK_RE.find(content) {
        // The current chunk is the termination chunk, "data: [DONE]\n\n", from OpenAI
        Ok(ExtractedChunkWithRemainingContent { chunk: None, remaining_content: None })
    } else {
        // There is no response in the json_content,
        // so return None for the response and the json_content as the remaining content
        Ok(ExtractedChunkWithRemainingContent {
            chunk: None,
            remaining_content: match content.len() {
                0 => None,
                _ => Some(content.to_string()),
            },
        })
    }
}

#[derive(Debug)]
pub struct ExtractedChunkWithRemainingContent {
    pub chunk: Option<OpenAIChatCompletionChunk>,
    pub remaining_content: Option<String>,
}

pub struct OpenAIChatCompletionStream<S> {
    response_bytes_stream: S,
    remaining_content: Option<String>,
}

impl<S> OpenAIChatCompletionStream<S> where S: Stream<Item = Result<Bytes, reqwest::Error>> + Unpin {
    pub fn new(response_bytes_stream: S) -> Self {
        Self {
            response_bytes_stream,
            remaining_content: None,
        }
    }
}

impl<S> Stream
    for OpenAIChatCompletionStream<S>
    where S: Stream<Item = Result<Bytes, reqwest::Error>> + Unpin
{
    type Item = OpenAIChatCompletionChunk;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            match self.response_bytes_stream.poll_next_unpin(cx) {
                Poll::Ready(Some(Ok(bytes))) => {
                    // Convert bytes to string
                    let mut content = std::str::from_utf8(&bytes).unwrap().to_string();

                    // Concatenate the remaining content if there is any
                    if let Some(remaining_content) = &self.remaining_content {
                        // Put the remaining content in front of the currently received content
                        content.insert_str(0, remaining_content);
                    }

                    // Extract the first response and remaining content
                    let response_with_remaining_content = extract_first_chunk(&content).unwrap();

                    // Get the response and remaining content
                    let chunk = response_with_remaining_content.chunk;
                    let remaining_content = response_with_remaining_content.remaining_content;

                    // Collect the remaining content if there is any
                    if let Some(remaining_content) = &remaining_content {
                        self.remaining_content = Some(remaining_content.to_owned());
                    } else {
                        self.remaining_content = None;
                    }

                    // Return Ready if there is one response
                    if let Some(chunk) = chunk {
                        return Poll::Ready(Some(chunk));
                    } else if remaining_content.is_none() {
                        // If both the chunk and the remaining content are None,
                        // then it means that the current chunk
                        // is the termination chunk, "data: [DONE]\n\n", from OpenAI
                        return Poll::Ready(None);
                    } else {
                        continue;
                        // return Poll::Pending;
                    }
                }
                Poll::Ready(Some(Err(e))) => {
                    error!("The response is incomplete - {}", e);
                    return Poll::Ready(None);
                }
                Poll::Ready(None) => {
                    // All the chunks from OpenAI have been received
                    // But chances are that there are still some remaining content
                    // that has not been parsed yet
                    // So we handle the remaining content here
                    if let Some(remaining_content) = &self.remaining_content {
                        // Extract the first response and remaining content
                        let response_with_remaining_content = extract_first_chunk(
                            &remaining_content
                        ).unwrap();

                        // Get the response and remaining content
                        let chunk = response_with_remaining_content.chunk;
                        let remaining_content = response_with_remaining_content.remaining_content;

                        // Collect the remaining content if there is any
                        if let Some(remaining_content) = &remaining_content {
                            self.remaining_content = Some(remaining_content.to_owned());
                        } else {
                            self.remaining_content = None;
                        }

                        // Return Ready if there is one response
                        if let Some(chunk) = chunk {
                            return Poll::Ready(Some(chunk));
                        } else if remaining_content.is_none() {
                            // If both the chunk and the remaining content are None,
                            // then it means that the current chunk
                            // is the termination chunk, "data: [DONE]\n\n", from OpenAI
                            return Poll::Ready(None);
                        } else {
                            continue;
                        }
                    }

                    // If there is no remaining content, then return None
                    return Poll::Ready(None);
                }
                Poll::Pending => {
                    return Poll::Pending;
                }
            }
        }
    }
}
