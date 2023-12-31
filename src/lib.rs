mod logging;
pub use logging::init_logger;

mod client;
pub use client::{ OpenAIClient, OpenAIClientBuilder };

mod error;
pub use error::OpenAIError;
pub type OpenAIResult<T> = Result<T, OpenAIError>;

pub mod chat;
pub mod embeddings;
