use serde::Deserialize;
use super::{ ChatCompletionFinishReason, ChatCompletionToolCall };

#[derive(Debug, Deserialize, Clone)]
pub struct ChatCompletionChunkChoice {
    pub finish_reason: Option<ChatCompletionFinishReason>,
    pub index: u32,
    pub delta: ChatCompletionChunkChoiceDelta,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChatCompletionChunkChoiceDelta {
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ChatCompletionToolCall>>,
}
