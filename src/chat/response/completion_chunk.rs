use serde::Deserialize;
use super::super::OpenAIChatRole;

#[derive(Debug, Deserialize)]
pub struct OpenAIChatCompletionChunk {
    pub id: String,
    pub choices: Vec<OpenAIChatCompletionChunkChoice>,
    pub created: i64,
    pub model: String,
    pub object: String,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIChatCompletionChunkChoice {
    pub delta: OpenAIChatCompletionChunkChoiceDelta,
    pub finish_reason: Option<String>,
    pub index: u32,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIChatCompletionChunkChoiceDelta {
    pub content: Option<String>,
    pub role: Option<OpenAIChatRole>,
}


