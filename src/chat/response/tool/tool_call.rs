use serde::Deserialize;
use super::ChatCompletionToolCallFunction;

#[derive(Debug, Deserialize)]
pub struct ChatCompletionToolCall {
    pub id: String,
    pub function: ChatCompletionToolCallFunction,
}
