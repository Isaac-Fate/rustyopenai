use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OpenAIChatTokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
