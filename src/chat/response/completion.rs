use serde::Deserialize;
use super::{super::OpenAIChatMessage, OpenAIChatTokenUsage};

#[derive(Debug, Deserialize)]
pub struct OpenAIChatCompletion {
    pub id: String,
    pub choices: Vec<OpenAIChatCompletionChoice>,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub usage: OpenAIChatTokenUsage,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIChatCompletionChoice {
    pub finish_reason: String,
    pub index: u32,
    pub message: OpenAIChatMessage,
}

