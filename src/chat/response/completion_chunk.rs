use serde::Deserialize;
use super::super::ChatRole;

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub choices: Vec<ChatCompletionChunkChoice>,
    pub created: i64,
    pub model: String,
    pub system_fingerprint: Option<String>,
    pub object: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChunkChoice {
    pub delta: ChatCompletionChunkChoiceDelta,
    pub logprobs: Option<ChatCompletionChunkChoiceLogprobs>,
    pub finish_reason: Option<String>,
    pub index: u32,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChunkChoiceDelta {
    pub content: Option<String>,
    pub role: Option<ChatRole>,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChunkChoiceLogprobs {
    pub content: Option<Vec<ChatCompletionChunkChoiceLogprobsContentItem>>,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChunkChoiceLogprobsContentItem {
    pub token: String,
    pub logprob: f32,
    pub bytes: Option<Vec<u8>>,
}
