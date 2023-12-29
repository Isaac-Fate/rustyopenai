use anyhow::anyhow;
use serde::Deserialize;

/// A macro to create an error enum with variants that have a message field.
macro_rules! create_openai_error {
    ($error:ident; $($variant:ident),* $(,)?) => {
        #[derive(Debug)]
        pub enum $error {
            $(
                $variant {
                    message: String,
                }
            ),*
        }
    };
}

// Create OpenAIError enum
create_openai_error! {
    OpenAIError;
    Authentication,
    BadRequest,
}

impl std::fmt::Display for OpenAIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenAIError::Authentication { message } =>
                write!(f, "Authentication error: {}", message),
            OpenAIError::BadRequest { message } => write!(f, "Bad request: {}", message),
        }
    }
}

impl std::error::Error for OpenAIError {}

impl TryFrom<RawOpenAIError> for OpenAIError {
    type Error = anyhow::Error;

    fn try_from(raw_openai_error: RawOpenAIError) -> Result<Self, Self::Error> {
        if let Some(error_code) = raw_openai_error.detail.code {
            match error_code.as_str() {
                "invalid_api_key" =>
                    Ok(OpenAIError::Authentication { message: raw_openai_error.detail.message }),
                _ => Err(anyhow!("Unknown error code: {}", error_code)),
            }
        } else {
            Ok(OpenAIError::BadRequest { message: raw_openai_error.detail.message })
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RawOpenAIError {
    #[serde(rename = "error")]
    detail: RawOpenAIErrorDetail,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct RawOpenAIErrorDetail {
    code: Option<String>,
    message: String,

    #[serde(rename = "type")]
    error_type: String,
}
