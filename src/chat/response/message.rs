use serde::Deserialize;
use super::ChatCompletionToolCall;

/// The role is neglected since it is always `"assistant"`.
#[derive(Debug, Deserialize)]
pub struct ChatCompletionMessage {
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ChatCompletionToolCall>>,
}
