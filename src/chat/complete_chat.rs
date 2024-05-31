use crate::{ Result, Error, ChatApiError, OpenAIClient };
use super::{
    CHAT_COMPLETION_API_ENDPOINT,
    ChatCompletionResponse,
    ChatRequestBody,
    ChatRequestMessage,
};

pub async fn get_complete_chat_response(
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
    use crate::utils::init_test_logger;
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
            vec![ChatRequestMessage {
                role: "user".to_string(),
                content: "Hello, how are you?".to_string(),
            }]
        )
            .frequency_penalty(100.0)
            .build();

        println!("{:#?}", serde_json::to_value(&request_body));

        // Get the complete chat response
        let response = get_complete_chat_response(&client, &request_body).await;

        println!("{:#?}", response);
    }
}
