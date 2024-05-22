use std::{ path::PathBuf, time::Duration };
use reqwest::{ Client, ClientBuilder, IntoUrl, RequestBuilder };
use lazy_static::lazy_static;
use log::*;
use crate::{ Result, Error };

lazy_static! {
    /// The path to the dotenv file.
    pub static ref DOTENV_FILEPATH: Option<PathBuf> = {
        match dotenv::dotenv().ok() {
            Some(path) => {
                info!("loaded environment variables from {:?}", path);
                Some(path)
            }
            None => {
                warn!("failed to load environment variables");
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
    /// Creates a builder for OpenAIClient.
    pub fn builder() -> OpenAIClientBuilder {
        OpenAIClientBuilder::new()
    }

    /// Creates a POST request builder.
    pub fn post<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.http_client
            .post(url)

            // Set the authorization header
            .header("Authorization", format!("Bearer {}", self.api_key.as_str()))
    }
}

/// Builder for `OpenAIClient`.
pub struct OpenAIClientBuilder {
    api_key: Option<String>,
    http_client_builder: ClientBuilder,
}

impl OpenAIClientBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self {
            api_key: None,
            http_client_builder: Client::builder(),
        }
    }

    /// Sets the API key.
    pub fn api_key<S: AsRef<str>>(mut self, api_key: S) -> Self {
        self.api_key = Some(api_key.as_ref().to_string());
        self
    }

    /// Sets the timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.http_client_builder = self.http_client_builder.timeout(timeout);
        self
    }

    /// Builds the OpenAI client.
    pub fn build(self) -> Result<OpenAIClient> {
        // Get the API key
        // If the API key is not set, try to get it from the environment variable
        let api_key = self.api_key
            .or(OPENAI_API_KEY.as_ref().map(|s| s.to_string()))
            .ok_or(Error::ApiKeyNotSet)?;

        // Build an HTTP client
        match self.http_client_builder.build() {
            // Return the OpenAI client
            Ok(http_client) => {
                Ok(OpenAIClient {
                    api_key,
                    http_client,
                })
            }

            // Return the error
            Err(error) => { Err(Error::BuildHttpClient { source: error }) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_logger() {
        let _ = env_logger
            ::builder()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .try_init();
    }

    #[test]
    fn test_openai_api_key() {
        // Initialize a logger
        init_logger();

        // A logging message will be printed
        // since this variable is accessed for the first time
        assert!(OPENAI_API_KEY.as_ref().is_some());

        // No more messages will be printed
        assert!(OPENAI_API_KEY.as_ref().is_some());
    }

    #[test]
    fn test_build() {
        // Initialize a logger
        init_logger();

        let client = OpenAIClientBuilder::new().build();

        assert!(client.is_ok());
    }
}
