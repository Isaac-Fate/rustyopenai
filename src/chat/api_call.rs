use futures::Stream;
use crate::{ OpenAIResult, OpenAIError, error::OpenAIErrorResponse, OpenAIClient };
use super::{ ChatRequestBody, ChatCompletion, ChatCompletionChunk, ChatCompletionStream };

/// Call OpenAI chat API and return a complete chat response.
pub async fn get_complete_chat_response(
    client: &OpenAIClient,
    request_body: &ChatRequestBody
) -> OpenAIResult<ChatCompletion> {
    // Convert to a map
    let mut request_body = serde_json
        ::to_value(request_body)
        .unwrap()
        .as_object()
        .unwrap()
        .to_owned();

    // Set the key "stream" to false
    request_body.insert("stream".to_string(), serde_json::json!(false));

    // Call API to get chat response
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .json(&request_body)
        .send().await;

    // If the response is an error, return the error
    let response = match response {
        Ok(response) => response,
        Err(error) => {
            // Construct an OpenAIError from the reqwest::Error
            return Err(OpenAIError::from(error));
        }
    };

    // Get the response content
    let response_content = response.text().await?;

    // Parse the response content
    // If the response is successful, parse the response content as OpenAIChatResponse
    // If the response is not successful, parse the response content as OpenAIError
    if let Ok(response) = serde_json::from_str::<ChatCompletion>(&response_content) {
        Ok(response)
    } else {
        let error_response = serde_json
            ::from_str::<OpenAIErrorResponse>(&response_content)
            .unwrap();
        if let Ok(openai_error) = OpenAIError::try_from(error_response) {
            Err(openai_error)
        } else {
            Err(OpenAIError::Other { message: response_content })
        }
    }
}

pub async fn get_streamed_chat_response(
    client: &OpenAIClient,
    request_body: &ChatRequestBody
) -> OpenAIResult<impl Stream<Item = OpenAIResult<ChatCompletionChunk>>> {
    // Convert to a map
    let mut request_body = serde_json
        ::to_value(request_body)
        .unwrap()
        .as_object()
        .unwrap()
        .to_owned();

    // Set the key "stream" to true
    request_body.insert("stream".to_string(), serde_json::json!(true));

    // Call API to get chat response
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .json(&request_body)
        .send().await?;

    // Create ChatResponseStream from the response bytes stream
    Ok(ChatCompletionStream::new(response))
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use futures::StreamExt;
    use crate::{
        init_logger,
        OpenAIResult,
        OpenAIError,
        chat::{ ChatMessage, ChatRole, ChatRequestBody },
        OpenAIClient,
    };
    use super::{ get_complete_chat_response, get_streamed_chat_response };

    #[tokio::test]
    async fn authentication_error() -> OpenAIResult<()> {
        // Initialize logger
        init_logger();

        // Call API to get chat response
        let response = get_complete_chat_response(
            &OpenAIClient::builder()
                .api_key("a-wrong-api-key")
                .timeout(Duration::from_secs(10))
                .build()?,
            &ChatRequestBody::builder()
                .model("gpt-3.5-turbo")
                .messages(
                    vec![ChatMessage {
                        role: ChatRole::User,
                        content: "What is Rust?".to_string(),
                    }]
                )
                .temperature(0.0)
                .build()?
        ).await;

        // Assert that the response is an error
        assert!(response.is_err());

        // Unwrap the error
        let error = response.unwrap_err();

        // Assert that the error is Authentication
        assert!(matches!(error, OpenAIError::Authentication { .. }));

        // Print the error
        println!("{}", error);

        Ok(())
    }

    #[tokio::test]
    async fn bad_request_error() -> Result<(), OpenAIError> {
        // Initialize logger
        init_logger();

        // Call API to get chat response
        let response = get_complete_chat_response(
            &OpenAIClient::builder().timeout(Duration::from_secs(10)).build()?,
            &ChatRequestBody::builder()
                .model("gpt-3.5-turbo")
                .messages(
                    vec![ChatMessage {
                        role: ChatRole::User,
                        content: "What is Rust?".to_string(),
                    }]
                )

                // Temperature must be between 0.0 and 1.0
                .temperature(-1.0)

                .build()?
        ).await;

        // Assert that the response is an error
        assert!(response.is_err());

        // Unwrap the error
        let error = response.unwrap_err();

        // Assert that the error is BadRequest
        assert!(matches!(error, OpenAIError::BadRequest { .. }));

        // Print the error
        println!("{}", error);

        Ok(())
    }

    #[tokio::test]
    async fn model_not_found_error() -> Result<(), OpenAIError> {
        // Initialize logger
        init_logger();

        // Call API to get chat response
        let response = get_complete_chat_response(
            &OpenAIClient::builder().timeout(Duration::from_secs(60)).build()?,
            &ChatRequestBody::builder()
                .model("gpt-dummy")
                .messages(
                    vec![ChatMessage {
                        role: ChatRole::User,
                        content: "What is Rust?".to_string(),
                    }]
                )
                .temperature(0.0)
                .build()?
        ).await;

        // Assert that the response is an error
        assert!(response.is_err());

        // Unwrap the error
        let error = response.unwrap_err();

        // Assert that the error is ModelNotFound
        assert!(matches!(error, OpenAIError::ModelNotFound { .. }));

        // Print the error
        println!("{}", error);

        Ok(())
    }

    #[tokio::test]
    async fn timed_out_error() -> Result<(), OpenAIError> {
        // Initialize logger
        init_logger();

        // Call API to get chat response
        let response = get_complete_chat_response(
            &OpenAIClient::builder().timeout(Duration::from_secs(1)).build()?,
            &ChatRequestBody::builder()
                .model("gpt-3.5-turbo")
                .messages(
                    vec![ChatMessage {
                        role: ChatRole::User,
                        content: "What is Rust?".to_string(),
                    }]
                )
                .temperature(0.0)
                .build()?
        ).await;

        // Assert that the response is an error
        assert!(response.is_err());

        // Unwrap the error
        let error = response.unwrap_err();

        // Assert that the error is TimedOut
        assert!(matches!(error, OpenAIError::TimedOut { .. }));

        // Print the error
        println!("{}", error);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_complete_chat_response() -> Result<(), OpenAIError> {
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
                .temperature(0.0)
                .build()?
        ).await;

        println!("response: {:#?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_streamed_chat_response() -> Result<(), OpenAIError> {
        // Initialize logger
        init_logger();

        // Call API to get the streamed chat response
        let mut stream = get_streamed_chat_response(
            &OpenAIClient::builder().timeout(Duration::from_secs(3)).build().unwrap(),
            &ChatRequestBody::builder()
                .model("gpt-3.5-turbo")
                .messages(
                    vec![ChatMessage {
                        role: ChatRole::User,
                        content: "What is Rust?".to_string(),
                    }]
                )
                .logprobs(false)
                .temperature(0.0)
                .build()
                .unwrap()
        ).await.unwrap();

        while let Some(chunk) = stream.next().await {
            println!("{:#?}", chunk);
        }

        Ok(())
    }
}
