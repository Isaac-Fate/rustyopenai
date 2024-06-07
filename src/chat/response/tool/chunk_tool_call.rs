use serde::Deserialize;
use super::ChatCompletionChunkToolCallFunction;

#[derive(Debug, Deserialize, Clone)]
pub struct ChatCompletionChunkToolCall {
    pub id: Option<String>,
    pub function: ChatCompletionChunkToolCallFunction,
}
