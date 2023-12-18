use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatMessage {
    
    pub role: OpenAIChatRole,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OpenAIChatRole {
    #[serde(rename = "system")]
    System,

    #[serde(rename = "user")]
    User,

    #[serde(rename = "assistant")]
    Assistant,
}