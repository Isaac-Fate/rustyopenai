use crate::{ Result, Error, OpenAIClient, ChatApiError };
use super::super::{ endpoint::CHAT_COMPLETION_API_ENDPOINT, ChatRequestBody, ChatCompletion };

pub async fn create_chat_completion(
    client: &OpenAIClient,
    request_body: &ChatRequestBody
) -> Result<ChatCompletion> {
    // Send the request
    let response = match client.post(CHAT_COMPLETION_API_ENDPOINT).json(request_body).send().await {
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

    // Parse the response
    let response = match response.json::<ChatCompletion>().await {
        Ok(response) => response,
        Err(error) => {
            return Err(Error::ChatApi(ChatApiError::ParseToChatCompletion { source: error }));
        }
    };

    Ok(response)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use futures::StreamExt;
    use crate::prelude::*;
    use crate::utils::init_test_logger;
    use super::*;

    #[tokio::test]
    async fn get_complete_chat_response() -> Result<()> {
        // Create a client
        let client = OpenAIClient::new()?;

        // Build the request body
        let request_body = ChatRequestBody::builder(
            "gpt-3.5-turbo",
            vec![
                system_message!("You are a helpful assistant."),
                user_message!("What is the tallest building in Hong Kong?"),
                user_message!("And in Shanghai?")
            ]
        ).build();

        // Send the request
        let chat_completion = create_chat_completion(&client, &request_body).await?;

        println!("{:#?}", chat_completion);

        Ok(())
    }

    #[tokio::test]
    async fn get_streamed_chat_response() -> Result<()> {
        // Create a client
        let client = OpenAIClient::new()?;

        // Build the request body
        let request_body = ChatRequestBody::builder(
            "gpt-3.5-turbo",
            vec![
                system_message!("You are a helpful assistant."),
                user_message!("What is the tallest building in Hong Kong?")
            ]
        ).build();

        let request_body = serde_json::to_value(&request_body).unwrap();

        let mut request_body: HashMap<String, serde_json::Value> = serde_json
            ::from_value(request_body)
            .unwrap();

        request_body.insert("stream".to_string(), serde_json::Value::Bool(true));

        // Send the request
        let response = client
            .post(CHAT_COMPLETION_API_ENDPOINT)
            // .json(&request_body)
            .json(&request_body)
            .send().await
            .unwrap();

        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(chunk) => {
                    println!("{}", String::from_utf8_lossy(&chunk));
                }
                Err(error) => {
                    println!("Error: {}", error);
                }
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_get_complete_chat_response() {
        // Initialize a logger
        init_test_logger();

        // Create a client
        let client = OpenAIClient::new().unwrap();

        // Build the request body
        let request_body = ChatRequestBody::builder(
            "gpt-3.5-turbo",
            vec![user_message!("What is the breaking news today?")]
        )
            .tools(
                vec![
                    function!(
                        "search_on_web",
                        description = "Search for information based on a query.",
                        parameters = function_parameters! {
                            "query": json!({"type": "string", "description": "The query to search for."});
                            "browser": json!({"type": "string", "enum": ["chrome", "firefox"], "description": "The browser to use."})
                        }
                    )
                ]
            )
            .tool_choice(tool_choice!(auto))
            .build();

        println!("{:#?}", serde_json::to_value(&request_body));
        println!("{}", serde_json::to_string_pretty(&request_body).unwrap());

        // Get the complete chat response
        let response = create_chat_completion(&client, &request_body).await;

        println!("{:#?}", response);
    }
}
