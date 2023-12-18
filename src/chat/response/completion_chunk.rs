use serde::Deserialize;
use super::super::ChatRole;

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub choices: Vec<ChatCompletionChunkChoice>,
    pub created: i64,
    pub model: String,
    pub object: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChunkChoice {
    pub delta: ChatCompletionChunkChoiceDelta,
    pub finish_reason: Option<String>,
    pub index: u32,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChunkChoiceDelta {
    pub content: Option<String>,
    pub role: Option<ChatRole>,
}
