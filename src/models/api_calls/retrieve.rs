use crate::{ Result, Error, ModelsApiError, OpenAIClient };
use super::super::{ MODELS_API_ENDPOINT, Model };

/// Retrieves a model instance, providing basic information about the model such as the owner and permissioning.
pub async fn retrieve_model<S: AsRef<str>>(client: &OpenAIClient, model_name: S) -> Result<Model> {
    // Send the request
    let response = match
        client.get(format!("{}/{}", MODELS_API_ENDPOINT, model_name.as_ref())).send().await
    {
        Ok(response) =>
            match response.error_for_status() {
                Ok(response) => response,
                Err(error) => {
                    if let Some(reqwest::StatusCode::NOT_FOUND) = error.status() {
                        return Err(
                            Error::ModelsApi(
                                ModelsApiError::ModelNotFound(model_name.as_ref().to_string())
                            )
                        );
                    }

                    return Err(Error::from(error));
                }
            }
        Err(error) => {
            return Err(Error::from(error));
        }
    };

    // Deserialize the response
    let model = match response.json::<Model>().await {
        Ok(model) => model,
        Err(error) => {
            return Err(Error::ModelsApi(ModelsApiError::ParseToModel { source: error }));
        }
    };

    Ok(model)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_retrieve_model() -> Result<()> {
        // Create a client
        let client = OpenAIClient::new()?;

        // Send the request
        let model = retrieve_model(&client, "dall-e-2").await;
        assert!(model.is_ok());

        // Unwrap the models
        let model = model?;
        println!("{:#?}", model);

        // Send the request
        let model = retrieve_model(&client, "dall-e").await;
        assert!(model.is_err());

        // Unwrap the models
        let error = model.unwrap_err();
        eprintln!("{}", error);

        Ok(())
    }
}
