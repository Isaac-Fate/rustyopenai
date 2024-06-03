use serde::Deserialize;
use super::ChatCompletionToolCall;

#[derive(Debug, Deserialize)]
pub struct ChatCompletionMessage {
    pub role: String,
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ChatCompletionToolCall>>,
}
