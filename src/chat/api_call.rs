use anyhow::{ Result, anyhow };
use futures::Stream;
use super::{
    super::OpenAIClient,
    ChatRequestBody,
    ChatCompletion,
    ChatCompletionChunk,
    ChatCompletionStream,
};

/// Call OpenAI chat API and return a complete chat response.
pub async fn get_complete_chat_response(
    client: &OpenAIClient,
    request_body: &ChatRequestBody
) -> Result<ChatCompletion> {
    // Convert to a map
    let mut request_body = serde_json::to_value(request_body)?.as_object().unwrap().to_owned();

    // Set the key "stream" to false
    request_body.insert("stream".to_string(), serde_json::json!(false));

    // Call API to get chat response
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .json(&request_body)
        .send().await?;

    // Get the response content
    let response_content = response.text().await?;

    // Parse the response content
    // If the response is successful, parse the response content as OpenAIChatResponse
    // If the response is not successful, parse the response content as OpenAIError
    if let Ok(response) = serde_json::from_str::<ChatCompletion>(&response_content) {
        Ok(response)
    } else {
        Err(anyhow!(response_content))
    }
}

pub async fn get_streamed_chat_response(
    client: &OpenAIClient,
    request_body: &ChatRequestBody
) -> Result<impl Stream<Item = ChatCompletionChunk>> {
    // Convert to a map
    let mut request_body = serde_json::to_value(request_body)?.as_object().unwrap().to_owned();

    // Set the key "stream" to true
    request_body.insert("stream".to_string(), serde_json::json!(true));

    // Call API to get chat response
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .json(&request_body)
        .send().await?;

    // Create ChatResponseStream from the response bytes stream
    Ok(ChatCompletionStream::new(response.bytes_stream()))
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use anyhow::Result;
    use futures::StreamExt;
    use crate::{ init_logger, chat::{ ChatMessage, ChatRole, ChatRequestBody }, OpenAIClient };
    use super::{ get_complete_chat_response, get_streamed_chat_response };

    #[tokio::test]
    async fn test_get_complete_chat_response() -> Result<()> {
        // Initialize logger
        init_logger();

        // Call API to get chat response
        let response = get_complete_chat_response(
            &OpenAIClient::builder().timeout(Duration::from_secs(60)).build()?,
            &ChatRequestBody::builder()
                .model("gpt-3.5-turbo")
                .messages(
                    vec![ChatMessage {
                        role: ChatRole::User,
                        content: "What is Rust?".to_string(),
                    }]
                )
                .temperature(0.9)
                .build()?
        ).await;

        println!("{:#?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_streamed_chat_response() -> Result<()> {
        // Initialize logger
        init_logger();

        // Call API to get the streamed chat response
        let mut stream = get_streamed_chat_response(
            &OpenAIClient::builder().timeout(Duration::from_secs(60)).build()?,
            &ChatRequestBody::builder()
                .model("gpt-3.5-turbo")
                .messages(
                    vec![ChatMessage {
                        role: ChatRole::User,
                        content: "What is Rust?".to_string(),
                    }]
                )
                .logprobs(true)
                .temperature(0.0)
                .build()?
        ).await?;

        while let Some(chunk) = stream.next().await {
            println!("{:#?}", chunk);
        }

        Ok(())
    }
}
