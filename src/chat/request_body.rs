use serde::Serialize;
use serde_with::skip_serializing_none;
use super::ChatMessage;

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct ChatRequestBody {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
    pub top_p: f32,
    pub presence_penalty: f32,
    pub user_id: Option<String>,
}

impl ChatRequestBody {
    pub fn new(
        model: &str,
        messages: Vec<ChatMessage>,
        temperature: f32,
        top_p: f32,
        presence_penalty: f32,
        user_id: Option<String>
    ) -> Self {
        Self {
            model: model.to_string(),
            messages,
            temperature,
            top_p,
            presence_penalty,
            user_id,
        }
    }

    pub fn builder() -> ChatRequestBodyBuilder {
        ChatRequestBodyBuilder::new()
    }
}

pub struct ChatRequestBodyBuilder {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    top_p: f32,
    presence_penalty: f32,
    user_id: Option<String>,
}

impl ChatRequestBodyBuilder {
    pub fn new() -> Self {
        Self {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![],
            temperature: 0.0,
            top_p: 0.0,
            presence_penalty: 0.0,
            user_id: None,
        }
    }

    /// Set model name.
    pub fn model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }

    /// Set messages.
    pub fn messages(mut self, messages: Vec<ChatMessage>) -> Self {
        self.messages = messages;
        self
    }

    /// Set temperature, which controls the randomness of the output.
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }

    /// Set top_p, which controls the diversity of the output via nucleus sampling.
    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = top_p;
        self
    }

    /// Set presence_penalty, which penalizes new tokens based on whether they appear in the text so far.
    pub fn presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.presence_penalty = presence_penalty;
        self
    }

    /// Set user ID.
    pub fn user_id(mut self, user_id: &str) -> Self {
        self.user_id = Some(user_id.to_string());
        self
    }

    pub fn build(self) -> ChatRequestBody {
        ChatRequestBody {
            model: self.model,
            messages: self.messages,
            temperature: self.temperature,
            top_p: self.top_p,
            presence_penalty: self.presence_penalty,
            user_id: self.user_id,
        }
    }
}
