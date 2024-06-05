use serde::Deserialize;
use super::ChatCompletionToolCallFunction;

#[derive(Debug, Deserialize, Clone)]
pub struct ChatCompletionToolCall {
    pub id: String,
    pub function: ChatCompletionToolCallFunction,
}
