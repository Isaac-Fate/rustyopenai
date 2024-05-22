/// The result type of this library.
pub type Result<T> = std::result::Result<T, Error>;

/// The error type of this library.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("OpenAI API key is not set")]
    ApiKeyNotSet,

    #[error("failed to build HTTP client")] BuildHttpClient {
        source: reqwest::Error,
    },
}
