use std::collections::HashMap;
use serde_json::json;
use crate::{ Result, Error, OpenAIClient, ChatApiError };
use super::super::{ endpoint::CHAT_COMPLETION_API_ENDPOINT, ChatRequestBody, ChatCompletionStream };

pub async fn create_chat_completion_stream(
    client: &OpenAIClient,
    request_body: &ChatRequestBody,
    include_usage: bool
) -> Result<ChatCompletionStream> {
    // We will first modify the request body so that
    // the fields `stream` and `stream_options` are set

    // Convert the request body to JSON value
    let request_body = match serde_json::to_value(&request_body) {
        Ok(request_body) => request_body,
        Err(error) => {
            return Err(Error::ChatApi(ChatApiError::ChatRequestBodyToJson { source: error }));
        }
    };

    // Convert the request body to a map so that it can be modified
    let mut request_body: HashMap<String, serde_json::Value> = match
        serde_json::from_value(request_body)
    {
        Ok(request_body) => request_body,
        Err(error) => {
            return Err(Error::ChatApi(ChatApiError::ChatRequestBodyJsonToMap { source: error }));
        }
    };

    // Set the `stream` field to `true`
    request_body.insert("stream".to_string(), serde_json::Value::Bool(true));

    // Set the `include_usage` wrapped in a `stream_options` object
    if include_usage {
        // Set the `stream_options` field
        request_body.insert(
            "stream_options".to_string(),
            json!({
                "include_usage": true
            })
        );
    }

    // Send the request
    let response = match client.post(CHAT_COMPLETION_API_ENDPOINT).json(&request_body).send().await {
        Ok(response) =>
            match response.error_for_status() {
                Ok(response) => response,
                Err(error) => {
                    return Err(Error::from(error));
                }
            }
        Err(error) => {
            return Err(Error::from(error));
        }
    };

    // Get the bytes stream
    let bytes_stream = response.bytes_stream();

    // Wrap the bytes stream in a ChatCompletionStream
    Ok(ChatCompletionStream::new(bytes_stream))
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use futures::StreamExt;
    use crate::prelude::*;
    use super::*;

    #[tokio::test]
    async fn test_create_chat_completion_stream() -> Result<()> {
        // Create a client
        let client = OpenAIClient::builder().timeout(Duration::from_millis(3000)).build()?;

        // Build the request body
        let request_body = ChatRequestBody::builder(
            "gpt-3.5-turbo",
            vec![
                system_message!("You are a helpful assistant."),
                user_message!("What is the tallest building in Hong Kong?")
            ]
        ).build();

        // Get the stream
        let mut chat_completion_stream = create_chat_completion_stream(
            &client,
            &request_body,
            true
        ).await?;

        while let Some(chunk) = chat_completion_stream.next().await {
            println!("{:#?}", chunk);
        }

        Ok(())
    }

    #[tokio::test]
    async fn tool_call() -> Result<()> {
        // Create a client
        let client = OpenAIClient::builder().timeout(Duration::from_millis(3000)).build()?;

        // Build the request body
        let request_body = ChatRequestBody::builder(
            "gpt-3.5-turbo",
            vec![
                system_message!("You are a helpful assistant."),
                user_message!("What is the tallest building in Hong Kong?")
            ]
        )
            .tools(
                vec![
                    function!(
                        "foo",
                        description = "A dummy function",
                        parameters = function_parameters! {
                            "a": json!({ "type": "number" });
                            "b": json!({ "type": "string" })
                        }
                    )
                ]
            )
            .tool_choice(tool_choice!("foo"))
            .build();

        // Get the stream
        let mut chat_completion_stream = create_chat_completion_stream(
            &client,
            &request_body,
            true
        ).await?;

        while let Some(chunk) = chat_completion_stream.next().await {
            println!("{:#?}", chunk);
        }

        Ok(())
    }
}
