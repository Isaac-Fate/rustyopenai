use serde::Serialize;
use crate::{ OpenAIResult, OpenAIError };
use super::ChatMessage;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct ChatRequestBody {
    pub messages: Vec<ChatMessage>,
    pub model: String,
    pub logprobs: bool,
    pub temperature: f32,
    pub top_p: f32,
    pub presence_penalty: f32,
    pub user_id: Option<String>,
}

impl ChatRequestBody {
    pub fn new(
        model: &str,
        messages: Vec<ChatMessage>,
        logprobs: bool,
        temperature: f32,
        top_p: f32,
        presence_penalty: f32,
        user_id: Option<String>
    ) -> Self {
        Self {
            model: model.to_string(),
            messages,
            logprobs,
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
    model: Option<String>,
    messages: Option<Vec<ChatMessage>>,
    logprobs: bool,
    temperature: f32,
    top_p: f32,
    presence_penalty: f32,
    user_id: Option<String>,
}

impl ChatRequestBodyBuilder {
    pub fn new() -> Self {
        Self {
            model: None,
            messages: None,
            logprobs: false,
            temperature: 0.0,
            top_p: 0.0,
            presence_penalty: 0.0,
            user_id: None,
        }
    }

    /// Set model name.
    pub fn model(mut self, model: &str) -> Self {
        self.model = Some(model.to_string());
        self
    }

    /// Set messages.
    pub fn messages(mut self, messages: Vec<ChatMessage>) -> Self {
        self.messages = Some(messages);
        self
    }

    /// Set logprobs.
    pub fn logprobs(mut self, logprobs: bool) -> Self {
        self.logprobs = logprobs;
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

    /// Build `ChatRequestBody`.
    pub fn build(self) -> OpenAIResult<ChatRequestBody> {
        Ok(ChatRequestBody {
            model: self.model.ok_or(OpenAIError::ModelNotSet)?,
            messages: self.messages.ok_or(OpenAIError::ChatMessagesNotSet)?,
            logprobs: self.logprobs,
            temperature: self.temperature,
            top_p: self.top_p,
            presence_penalty: self.presence_penalty,
            user_id: self.user_id,
        })
    }
}
