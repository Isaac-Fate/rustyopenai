mod client;
pub use client::OpenAIClient;

mod error;
pub use error::{ Result, Error, ChatApiError };

pub mod models;
pub use models::{ list_models, ListModelsResponse, ModelInfo };

pub mod chat;
pub mod embeddings;

mod utils;

pub mod prelude;

#[macro_use]
mod macros;
