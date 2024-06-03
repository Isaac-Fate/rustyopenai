use serde::Deserialize;
use super::ChatCompletionMessage;

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChoice {
    pub finish_reason: String,
    pub index: u32,
    pub message: ChatCompletionMessage,
}
