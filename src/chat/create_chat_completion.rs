use crate::{ Result, Error, OpenAIClient };
use super::{ CHAT_COMPLETION_API_ENDPOINT, ChatRequestBody, ChatCompletion };

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
            return Err(Error::ParseToChatCompletion { source: error });
        }
    };

    Ok(response)
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::prelude::*;
    use crate::utils::init_test_logger;
    use crate::chat::{
        UserMessage,
        Tool,
        Function,
        FunctionParameter,
        ToolChoice,
        ToolChoiceOption,
    };
    use super::*;

    #[tokio::test]
    async fn test_get_complete_chat_response() {
        // Initialize a logger
        init_test_logger();

        // Create a client
        let client = OpenAIClient::new().unwrap();

        // Prepare request body
        let request_body = ChatRequestBody::builder(
            "gpt-3.5-turbo",
            vec![user_message!("What is the breaking news today?")]
        )
            .tools(
                vec![
                    Tool::Function(
                        Function::builder("search_on_web")
                            .description("Search for information based on a query.")
                            .parameters(
                                function_parameters! {
                                    "query": json!({"type": "string", "description": "The query to search for."});
                                    "browser": json!({"type": "string", "enum": ["chrome", "firefox"], "description": "The browser to use."})
                                }
                            )
                            .build()
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
