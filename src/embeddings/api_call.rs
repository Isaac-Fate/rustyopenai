use crate::{ OpenAIResult, OpenAIClient };
use super::{ EmbeddingRequestBody, EmbeddingReponse };

pub async fn get_embedding(
    client: &OpenAIClient,
    request_body: EmbeddingRequestBody
) -> OpenAIResult<EmbeddingReponse> {
    Ok(
        client
            .post("https://api.openai.com/v1/embeddings")
            .json(&request_body)
            .send().await?
            .json().await?
    )
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use serde_json::{ json, Value };
    use crate::{
        OpenAIResult,
        OpenAIClient,
        embeddings::{ EmbeddingRequestBody, EmbeddingEncodingFormat, EmbeddingContent },
    };
    use super::get_embedding;

    #[tokio::test]
    async fn call_api_using_json_body() -> OpenAIResult<()> {
        let json_response = OpenAIClient::builder()
            .timeout(Duration::from_secs(10))
            .build()?
            .post("https://api.openai.com/v1/embeddings")
            .json(
                &json!(
                    {
                        "input": ["What is Rust?", "What is OpwnAI?"],
                        "model": "text-embedding-ada-002",
                        "encoding_format": "float",
                        "user": "user-1234",
                    }
                )
            )
            .send().await?
            .json::<Value>().await?;

        println!("{:?}", json_response);
        println!("{:#?}", json_response["data"].as_array().unwrap().len());

        Ok(())
    }

    #[tokio::test]
    async fn get_embedding_of_single_text() -> OpenAIResult<()> {
        let response = get_embedding(
            &OpenAIClient::builder().timeout(Duration::from_secs(10)).build()?,
            EmbeddingRequestBody::builder()
                .single_text("What is Rust?")
                .model("text-embedding-ada-002")
                .build()?
        ).await?;

        println!("{:?}", response);
        assert_eq!(response.data.len(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn get_embeddings_of_multiple_texts() -> OpenAIResult<()> {
        let response = get_embedding(
            &OpenAIClient::builder().timeout(Duration::from_secs(10)).build()?,
            EmbeddingRequestBody::builder()
                .multiple_texts(vec!["What is Rust?", "What is OpenAI?"])
                .model("text-embedding-ada-002")
                .build()?
        ).await?;

        println!("{:?}", response);
        assert_eq!(response.data.len(), 2);
        Ok(())
    }

    #[tokio::test]
    async fn get_base64_embedding() -> OpenAIResult<()> {
        let response = get_embedding(
            &OpenAIClient::builder().timeout(Duration::from_secs(10)).build()?,
            EmbeddingRequestBody::builder()
                .single_text("What is Rust?")
                .model("text-embedding-ada-002")
                .encoding_format(EmbeddingEncodingFormat::Base64)
                .build()?
        ).await?;

        println!("{:?}", response);
        assert_eq!(response.data.len(), 1);
        assert!(matches!(response.data.first().unwrap().embedding, EmbeddingContent::Base64(_)));
        Ok(())
    }
}
