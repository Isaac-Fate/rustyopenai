use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub created: u32,
    pub model: String,
    /// System fingerprint may be `None`.
    pub system_fingerprint: Option<String>,
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: ChatCompletionTokenUsage,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChoice {
    pub finish_reason: String,
    pub index: u32,
    pub message: ChatCompletionMessage,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionMessage {
    pub role: String,
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ChatCompletionMessageToolCall>>,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionMessageToolCall {
    pub id: String,

    #[serde(rename = "type")]
    pub tool_type: String,

    pub function: ChatCompletionMessageToolCallFunction,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionMessageToolCallFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionTokenUsage {
    pub completion_tokens: u32,
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}
