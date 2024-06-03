use serde::Deserialize;
use super::{ ChatCompletionChoice, ChatCompletionTokenUsage };

#[derive(Debug, Deserialize)]
pub struct ChatCompletion {
    pub id: String,
    pub created: u32,
    pub model: String,
    /// System fingerprint may be `None`.
    pub system_fingerprint: Option<String>,
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: ChatCompletionTokenUsage,
}
