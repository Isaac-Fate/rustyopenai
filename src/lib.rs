use std::path::PathBuf;
use lazy_static::lazy_static;
use tracing::info;

lazy_static! {
    pub static ref DOTENV_FILEPATH: Option<PathBuf> = {
        info!("Loading dotenv");
        dotenv::dotenv().ok()
    };
}

mod auth;
pub use auth::OPENAI_API_KEY;

pub mod chat;
