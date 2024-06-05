use serde::Deserialize;
use super::{ ChatCompletionChunkChoice, ChatCompletionTokenUsage };

#[derive(Debug, Deserialize, Clone)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub created: u32,
    pub model: String,
    pub system_fingerprint: Option<String>,
    pub choices: Vec<ChatCompletionChunkChoice>,
    pub usage: Option<ChatCompletionTokenUsage>,
}
