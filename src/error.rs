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

    #[error("failed to authenticate with the provided OpenAI API")]
    Authentication,

    #[error("you are accessing the API from an unsupported country, region, or territory")]
    UnsupportedRegion,

    #[error(
        "you are sending requests too quickly, or run out of credits or hit your maximum monthly spend"
    )]
    ExceedRateLimitOrQuota,

    #[error("issue on OpenAI servers")]
    Server,

    #[error("OpenAI servers are experiencing high traffic")]
    Overloaded,

    #[error("unknown status code {status_code}: {source}")] UnknownStatusCode {
        status_code: reqwest::StatusCode,

        #[source]
        source: reqwest::Error,
    },

    #[error("the request timed out")]
    Timeout,

    #[error("failed to connect to endpoint")]
    Connection,

    #[error("got a Reqwest error: {source}")] Reqwest {
        #[source]
        source: reqwest::Error,
    },

    #[error("failed to request the models API: {0}")] ModelsApi(ModelsApiError),

    #[error("failed to request the chat API: {0}")] ChatApi(ChatApiError),
}

#[derive(Debug, thiserror::Error)]
pub enum ModelsApiError {
    #[error("failed to parse the response to JSON: {source}")] ParseToJson {
        #[source]
        source: reqwest::Error,
    },

    #[error("the received JSON response does not contain the data property")]
    MissingDataProperty,

    #[error(
        "failed to parse the data property of the JSON response to a vector of Models: {source}"
    )] ParseToModels {
        #[source]
        source: serde_json::Error,
    },

    #[error("model with name {0} is not found")] ModelNotFound(String),

    #[error("failed to parse JSON response to a Model: {source}")] ParseToModel {
        #[source]
        source: reqwest::Error,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum ChatApiError {
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

    #[error("failed to receive a chunk of bytes from the API: {source}")] ReceiveStreamedBytes {
        #[source]
        source: reqwest::Error,
    },

    /// This error shouldn't happen
    #[error("failed to get the first matching data chunk from the captures")]
    GetFirstMatchingDataChunk,

    #[error("failed to parse to a chat completion chunk: {source}")] ParseToChatCompletionChunk {
        #[source]
        source: serde_json::Error,
    },
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        match error.status() {
            None => {
                if error.is_timeout() {
                    Error::Timeout
                } else if error.is_connect() {
                    Error::Connection
                } else {
                    Error::Reqwest { source: error }
                }
            }
            Some(status_code) =>
                match status_code {
                    // 401
                    reqwest::StatusCode::UNAUTHORIZED => Error::Authentication,

                    // 403
                    reqwest::StatusCode::FORBIDDEN => Error::UnsupportedRegion,

                    // 429
                    reqwest::StatusCode::TOO_MANY_REQUESTS => Error::ExceedRateLimitOrQuota,

                    // 500
                    reqwest::StatusCode::INTERNAL_SERVER_ERROR => Error::Server,

                    // 503
                    reqwest::StatusCode::SERVICE_UNAVAILABLE => Error::Overloaded,

                    // Other
                    _ => Error::UnknownStatusCode { status_code, source: error },
                }
        }
    }
}
