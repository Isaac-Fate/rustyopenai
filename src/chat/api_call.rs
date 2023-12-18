use anyhow::{ Result, anyhow };
use futures::Stream;
use reqwest::Client;
use super::{
    super::OPENAI_API_KEY,
    OpenAIChatRequestBody,
    OpenAIChatCompletion,
    OpenAIChatCompletionChunk,
    OpenAIChatCompletionStream,
};

/// Call OpenAI chat API and return a complete chat response.
pub async fn get_complete_chat_response(
    client: &Client,
    request_body: &OpenAIChatRequestBody
) -> Result<OpenAIChatCompletion> {
    // Convert to a map
    let mut request_body = serde_json::to_value(request_body)?.as_object().unwrap().to_owned();

    // Set the key "stream" to false
    request_body.insert("stream".to_string(), serde_json::json!(false));

    // Call API to get chat response
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", OPENAI_API_KEY.as_str()))
        .json(&request_body)
        .send().await?;

    // Get the response content
    let response_content = response.text().await?;

    // Parse the response content
    // If the response is successful, parse the response content as OpenAIChatResponse
    // If the response is not successful, parse the response content as OpenAIError
    if let Ok(response) = serde_json::from_str::<OpenAIChatCompletion>(&response_content) {
        Ok(response)
    } else {
        Err(anyhow!(response_content))
    }
}

pub async fn get_streamed_chat_response(
    client: &Client,
    request_body: &OpenAIChatRequestBody
) -> Result<impl Stream<Item = OpenAIChatCompletionChunk>> {
    // Convert to a map
    let mut request_body = serde_json::to_value(request_body)?.as_object().unwrap().to_owned();

    // Set the key "stream" to true
    request_body.insert("stream".to_string(), serde_json::json!(true));

    // Call API to get chat response
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", OPENAI_API_KEY.as_str()))
        .json(&request_body)
        .send().await?;

    // Create ChatResponseStream from the response bytes stream
    Ok(OpenAIChatCompletionStream::new(response.bytes_stream()))
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use anyhow::Result;
    use futures::StreamExt;
    use reqwest::Client;
    use crate::chat::{ OpenAIChatMessage, OpenAIChatRole, OpenAIChatRequestBody };
    use super::{ get_complete_chat_response, get_streamed_chat_response };

    #[tokio::test]
    async fn test_get_complete_chat_response() -> Result<()> {
        // Call API to get chat response
        let response = get_complete_chat_response(
            &Client::builder().timeout(Duration::from_secs(60)).build()?,
            &OpenAIChatRequestBody::builder()
                .messages(
                    vec![OpenAIChatMessage {
                        role: OpenAIChatRole::User,
                        content: "What is Rust?".to_string(),
                    }]
                )
                .temperature(0.9)
                .build()
        ).await;

        println!("{:#?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_streamed_chat_response() -> Result<()> {
        // Initialize logger
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

        // Call API to get the streamed chat response
        let mut stream = get_streamed_chat_response(
            &Client::builder().timeout(Duration::from_secs(60)).build()?,
            &OpenAIChatRequestBody::builder()
                .messages(
                    vec![OpenAIChatMessage {
                        role: OpenAIChatRole::User,
                        content: "What is Rust?".to_string(),
                    }]
                )
                .temperature(0.0)
                .build()
        ).await?;

        while let Some(chunk) = stream.next().await {
            println!("{:#?}", chunk);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_stream_response() -> Result<()> {
        use super::OPENAI_API_KEY;
        use super::OpenAIChatRequestBody;
        use crate::chat::{ OpenAIChatMessage, OpenAIChatRole };
        use futures::StreamExt;

        let request_body = OpenAIChatRequestBody::builder()
            .messages(
                vec![OpenAIChatMessage {
                    role: OpenAIChatRole::User,
                    content: "What is Rust?".to_string(),
                }]
            )
            .temperature(0.0)
            .build();

        // Convert to a map
        let mut request_body = serde_json::to_value(request_body)?.as_object().unwrap().to_owned();

        // Set stream to true
        request_body.insert("stream".to_string(), serde_json::json!(true));

        let response = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()?
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", OPENAI_API_KEY.as_str()))
            .json(&request_body)
            .send().await?;

        let mut strean = response.bytes_stream();

        while let Some(chunk) = strean.next().await {
            println!("{:#?}", chunk);
        }

        Ok(())
    }
}
