use serde::Deserialize;
use super::{ super::ChatMessage, ChatTokenUsage };

#[derive(Debug, Deserialize)]
pub struct ChatCompletion {
    pub id: String,
    pub choices: Vec<ChatCompletionChoice>,
    pub created: i64,
    pub model: String,
    pub system_fingerprint: Option<String>,
    pub object: String,
    pub usage: ChatTokenUsage,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChoice {
    pub finish_reason: String,
    pub index: u32,
    pub message: ChatMessage,
}
