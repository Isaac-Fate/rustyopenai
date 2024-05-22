mod client;
pub use client::{ OpenAIClient, OpenAIClientBuilder };

mod error;
pub use error::{ Result, Error };

pub mod models;
pub mod chat;
pub mod embeddings;
