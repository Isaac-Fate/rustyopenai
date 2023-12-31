use serde::Deserialize;

/// A macro to create an error enum with variants that have a message field.
macro_rules! create_openai_error {
    ($($simple_error:ident),* $(,)?; $($error_with_message:ident),* $(,)?) => {
        #[derive(Debug)]
        pub enum OpenAIError {
            $(
                $simple_error,
            )*
            $(
                $error_with_message {
                    message: String,
                }
            ),*
        }
    };
}

// Create OpenAIError enum
create_openai_error! {
    APIKeyNotSet,
    MessagesNotSet,
    ModelNotSet,
    ;
    Authentication,
    BadRequest,
    BuildClient,
    ModelNotFound,
    TimedOut,
    Other,
}

impl std::fmt::Display for OpenAIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenAIError::APIKeyNotSet => write!(f, "API key is not set"),
            OpenAIError::MessagesNotSet => write!(f, "`messages` must be set"),
            OpenAIError::ModelNotSet => write!(f, "`model` must be set"),
            OpenAIError::Authentication { message } => write!(f, "Authentication: {}", message),
            OpenAIError::BadRequest { message } => write!(f, "Bad Request: {}", message),
            OpenAIError::BuildClient { message } => write!(f, "Build Client: {}", message),
            OpenAIError::ModelNotFound { message } => write!(f, "Model Not Found: {}", message),
            OpenAIError::TimedOut { message } => write!(f, "Timed Out: {}", message),
            OpenAIError::Other { message } => write!(f, "Other: {}", message),
        }
    }
}

impl std::error::Error for OpenAIError {}

impl From<reqwest::Error> for OpenAIError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_body() {
            OpenAIError::BadRequest { message: error.to_string() }
        } else if error.is_builder() {
            OpenAIError::BuildClient { message: error.to_string() }
        } else if error.is_timeout() {
            OpenAIError::TimedOut { message: error.to_string() }
        } else {
            OpenAIError::Other { message: error.to_string() }
        }
    }
}

impl From<OpenAIErrorResponse> for OpenAIError {
    fn from(error_response: OpenAIErrorResponse) -> Self {
        if let Some(error_code) = error_response.detail.code {
            match error_code.as_str() {
                "invalid_api_key" =>
                    OpenAIError::Authentication { message: error_response.detail.message },
                "model_not_found" =>
                    OpenAIError::ModelNotFound { message: error_response.detail.message },
                _ =>
                    OpenAIError::Other {
                        message: format!("Unknown error code: {}", error_code),
                    },
            }
        } else {
            OpenAIError::BadRequest { message: error_response.detail.message }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OpenAIErrorResponse {
    #[serde(rename = "error")]
    detail: OpenAIErrorResponseDetail,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct OpenAIErrorResponseDetail {
    code: Option<String>,
    message: String,

    #[serde(rename = "type")]
    error_type: String,
}
