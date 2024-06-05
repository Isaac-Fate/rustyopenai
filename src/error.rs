/// The result type of this library.
pub type Result<T> = std::result::Result<T, Error>;

/// The error type of this library.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("OpenAI API key is not set")]
    ApiKeyNotSet,

    #[error("failed to build HTTP client: {source}")] BuildHttpClient {
        #[source]
        source: reqwest::Error,
    },

    #[error("failed to list models: {source}")] ListModels {
        #[source]
        source: reqwest::Error,
    },

    #[error("failed to parse to list models response: {source}")] PraseToListModelsResponse {
        #[source]
        source: reqwest::Error,
    },

    #[error("failed to authenticate with the provided OpenAI API")]
    Authentication,

    #[error("unknown status code {status_code}: {source}")] UnknownStatusCode {
        status_code: reqwest::StatusCode,

        #[source]
        source: reqwest::Error,
    },

    #[error("the request timed out")]
    Timeout,

    #[error("failed to connect to endpoint: {source}")] Connection {
        #[source]
        source: reqwest::Error,
    },

    #[error("got a Reqwest error: {source}")] Reqwest {
        #[source]
        source: reqwest::Error,
    },

    #[error("failed to request the chat API: {0}")] ChatApi(ChatApiError),

    #[error("failed to parse to chat completion: {source}")] ParseToChatCompletion {
        #[source]
        source: reqwest::Error,
    },

    #[error("failed to parse the chat request body to JSON: {source}")] ChatRequestBodyToJson {
        #[source]
        source: serde_json::Error,
    },

    #[error(
        "failed to parse the JSON chat request body to a hash map: {source}"
    )] ChatRequestBodyJsonToMap {
        #[source]
        source: serde_json::Error,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum ChatApiError {
    #[error("the messages field is requried")]
    MissingMessagesField,

    #[error("the model field is requried")]
    MissingModelField,
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        match error.status() {
            None => {
                if error.is_timeout() {
                    Error::Timeout
                } else if error.is_connect() {
                    Error::Connection { source: error }
                } else {
                    Error::Reqwest { source: error }
                }
            }
            Some(status_code) =>
                match status_code {
                    reqwest::StatusCode::UNAUTHORIZED => Error::Authentication,
                    _ => Error::UnknownStatusCode { status_code, source: error },
                }
        }
    }
}
