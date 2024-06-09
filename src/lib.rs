mod client;
pub use client::OpenAIClient;

mod error;
pub use error::{ Result, Error, ModelsApiError, ChatApiError };

pub mod models;
pub mod chat;
pub mod embeddings;
pub mod images;

mod utils;

pub mod prelude;

#[macro_use]
mod macros;
