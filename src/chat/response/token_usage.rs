use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChatCompletionTokenUsage {
    pub completion_tokens: u32,
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}
