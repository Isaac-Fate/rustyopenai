use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OpenAIChatResponse {
    pub id: String,
    pub choices: Vec<OpenAIChatResponseChoice>,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub usage: ChatTokenUsage,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIChatResponseChoice {
    pub finish_reason: String,
    pub index: u32,
    pub message: OpenAIChatResponseMessage,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIChatResponseMessage {
    pub content: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatTokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
