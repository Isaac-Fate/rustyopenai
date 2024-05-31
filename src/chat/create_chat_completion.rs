use crate::{ Result, Error, ChatApiError, OpenAIClient };
use super::{
    CHAT_COMPLETION_API_ENDPOINT,
    ChatCompletionResponse,
    ChatRequestBody,
    message::ChatRequestMessage,
};

pub async fn create_chat_completion(
    client: &OpenAIClient,
    request_body: &ChatRequestBody
) -> Result<ChatCompletionResponse> {
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

    // let response = response.json::<serde_json::Value>().await.unwrap();
    // println!("{:#?}", response);

    // let response = serde_json::from_value::<ChatCompletionResponse>(response).unwrap();

    let response = match response.json::<ChatCompletionResponse>().await {
        Ok(response) => response,
        Err(error) => {
            return Err(Error::PraseToListModelsResponse { source: error });
        }
    };

    Ok(response)
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::utils::init_test_logger;
    use crate::chat::{
        UserMessage,
        Tool,
        Function,
        FunctionParameter,
        ToolChoice,
        ToolChoiceOption,
        ToolChoiceParticularFunction,
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
            vec![ChatRequestMessage::User(UserMessage::new("What is the breaking news today?"))]
        )
            .tools(
                vec![
                    Tool::Function(
                        Function::builder("search_on_web")
                            .description("Search for information based on a query.")
                            .parameters(
                                vec![
                                    FunctionParameter::new(
                                        "query",
                                        true,
                                        json!({"type": "string", "description": "The query to search for."})
                                    ),
                                    FunctionParameter::new(
                                        "browser",
                                        true,
                                        json!({"type": "string", "enum": ["chrome", "firefox"], "description": "The browser to use."})
                                    )
                                ]
                            )
                            .build()
                    )
                ]
            )
            .tool_choice(ToolChoice::Option(ToolChoiceOption::Auto))
            .build();

        println!("{:#?}", serde_json::to_value(&request_body));
        println!("{}", serde_json::to_string_pretty(&request_body).unwrap());

        // Get the complete chat response
        let response = create_chat_completion(&client, &request_body).await;

        println!("{:#?}", response);
    }
}
