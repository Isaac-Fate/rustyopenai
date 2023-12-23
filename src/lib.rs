mod logging;
pub use logging::init_logger;

mod client;
pub use client::{ OpenAIClient, OpenAIClientBuilder };

pub mod chat;
pub mod embeddings;
