use serde::Deserialize;
use super::{ ChatCompletionMessage, ChatCompletionFinishReason };

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChoice {
    pub finish_reason: ChatCompletionFinishReason,
    pub index: u32,
    pub message: ChatCompletionMessage,
}
