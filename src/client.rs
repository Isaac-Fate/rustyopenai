use std::{ path::PathBuf, time::Duration };
use reqwest::{ Client, ClientBuilder, IntoUrl, RequestBuilder };
use lazy_static::lazy_static;
use tracing::{ info, warn };
use crate::{ OpenAIResult, OpenAIError };

lazy_static! {
    /// The path to the dotenv file.
    pub static ref DOTENV_FILEPATH: Option<PathBuf> = {
        info!("Loading dotenv");
        match dotenv::dotenv().ok() {
            Some(path) => {
                info!("Loaded dotenv from {:?}", path);
                Some(path)
            }
            None => {
                warn!("Failed to load dotenv");
                None
            }
        }
    };

    /// The OpenAI API key.
    pub static ref OPENAI_API_KEY: Option<String> = {
        let _ = DOTENV_FILEPATH.as_ref();
        dotenv::var("OPENAI_API_KEY").ok()
    };
}

pub struct OpenAIClient {
    api_key: String,
    http_client: Client,
}

impl OpenAIClient {
    /// Create a builder for OpenAIClient.
    pub fn builder() -> OpenAIClientBuilder {
        OpenAIClientBuilder::new()
    }

    /// Create a POST request builder.
    pub fn post<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.http_client
            .post(url)

            // Set the authorization header
            .header("Authorization", format!("Bearer {}", self.api_key.as_str()))
    }
}

pub struct OpenAIClientBuilder {
    api_key: Option<String>,
    http_client_builder: ClientBuilder,
}

impl OpenAIClientBuilder {
    pub fn new() -> Self {
        Self {
            api_key: None,
            http_client_builder: Client::builder(),
        }
    }

    /// Set the API key.
    pub fn api_key<S: AsRef<str>>(mut self, api_key: S) -> Self {
        self.api_key = Some(api_key.as_ref().to_string());
        self
    }

    /// Set the timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.http_client_builder = self.http_client_builder.timeout(timeout);
        self
    }

    /// Build the OpenAI client.
    pub fn build(self) -> OpenAIResult<OpenAIClient> {
        // Get the API key
        // If the API key is not set, try to get it from the environment variable
        let api_key = self.api_key
            .or(OPENAI_API_KEY.as_ref().map(|s| s.to_string()))
            .ok_or(OpenAIError::APIKeyNotSet)?;

        Ok(OpenAIClient {
            api_key,

            // Build the HTTP client
            http_client: self.http_client_builder.build()?,
        })
    }
}
