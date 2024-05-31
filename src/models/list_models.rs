use serde::Deserialize;
use crate::{ Error, OpenAIClient, Result };
use super::MODELS_API_ENDPOINT;

/// Lists the currently available models, and
/// provides basic information about each one such as the owner and availability.
pub async fn list_models(client: &OpenAIClient) -> Result<ListModelsResponse> {
    // Send the request
    let response = match client.get(MODELS_API_ENDPOINT).send().await {
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

    // Deserialize the response
    let response = match response.json::<ListModelsResponse>().await {
        Ok(response) => response,
        Err(error) => {
            return Err(Error::from(error));
        }
    };

    Ok(response)
}

/// Lists the currently available model names.
/// The model name is exactly its ID.
pub async fn list_model_names(client: &OpenAIClient) -> Result<Vec<String>> {
    // List models
    let response = list_models(client).await?;

    // Extract the model names
    Ok(
        response.data
            .into_iter()
            // We are only interested in the model name, i.e., its ID
            .map(|model_info| model_info.id)
            .collect()
    )
}

#[derive(Debug, Deserialize)]
pub struct ListModelsResponse {
    pub object: String,
    pub data: Vec<ModelInfo>,
}

#[derive(Debug, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub owned_by: String,
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;

    #[tokio::test]
    async fn test_list_models() {
        // Create a client
        let client = OpenAIClient::new().unwrap();

        // Get the list of models
        let response = list_models(&client).await.unwrap();

        println!("{:#?}", response);
    }

    #[tokio::test]
    async fn test_list_model_names() {
        // Create a client
        let client = OpenAIClient::new().unwrap();

        // Get the list of models
        let model_names = list_model_names(&client).await.unwrap();

        println!("{:#?}", model_names);
    }

    #[tokio::test]
    async fn test_wrong_api_key() {
        // Create a client with an invalid API key
        let client = OpenAIClient::builder().api_key("xxx").build().unwrap();

        // Send the request
        let response = list_models(&client).await;

        // Check the error
        assert!(response.is_err());
        assert!(matches!(response, Err(Error::Authentication)));

        // Unwrap the error
        let error = response.unwrap_err();

        // Display the error
        eprintln!("{}", error);
    }

    #[tokio::test]
    async fn test_timeout() {
        // Create a client with a short timeout
        // * Normally, no requests can be completed within 1ms
        let client = OpenAIClient::builder().timeout(Duration::from_millis(1)).build().unwrap();

        // Send the request
        let response = list_models(&client).await;

        // Check the error
        assert!(response.is_err());
        assert!(matches!(response, Err(Error::Timeout)));

        // Unwrap the error
        let error = response.unwrap_err();

        // Display the error
        eprintln!("{}", error);
    }

    #[tokio::test]
    async fn test_no_connection() {
        // Create a client
        let client = OpenAIClient::new().unwrap();

        // Send the request
        let response = list_models(&client).await;

        // Print the result
        match response {
            Ok(_) => { println!("connection is fine") }
            Err(error) => { eprintln!("{}", error) }
        }
    }
}
