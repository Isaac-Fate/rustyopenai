use serde::Serialize;
use log::warn;
use crate::{ Result, Error, ChatApiError };

const MIN_FREQUENCY_PENALTY: f32 = -2.0;
const MAX_FREQUENCY_PENALTY: f32 = 2.0;
const MIN_TOP_P: f32 = 0.0;
const MAX_TOP_P: f32 = 1.0;

#[derive(Debug, Serialize)]
pub struct ChatRequestBody {
    model: String,
    messages: Vec<ChatRequestMessage>,

    frequency_penalty: Option<f32>,
    max_tokens: Option<u32>,
    n: Option<u32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
}

impl ChatRequestBody {
    pub fn builder<S: AsRef<str>>(
        model: S,
        messages: Vec<ChatRequestMessage>
    ) -> ChatRequestBodyBuilder {
        ChatRequestBodyBuilder::new(model, messages)
    }
}

#[derive(Debug, Serialize)]
pub struct ChatRequestMessage {
    pub role: String,
    pub content: String,
}

pub struct ChatRequestBodyBuilder {
    model: String,
    messages: Vec<ChatRequestMessage>,
    frequency_penalty: Option<f32>,
    max_tokens: Option<u32>,
    n: Option<u32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
}

impl ChatRequestBodyBuilder {
    /// Creates a new builder with `None` values for all fields.
    pub fn new<S: AsRef<str>>(model: S, messages: Vec<ChatRequestMessage>) -> Self {
        Self {
            model: model.as_ref().to_string(),
            messages,
            frequency_penalty: None,
            max_tokens: None,
            n: None,
            temperature: None,
            top_p: None,
        }
    }

    /// Builds the request body.
    pub fn build(self) -> Result<ChatRequestBody> {
        Ok(ChatRequestBody {
            messages: self.messages,
            model: self.model,
            frequency_penalty: self.frequency_penalty,
            max_tokens: self.max_tokens,
            n: self.n,
            temperature: self.temperature,
            top_p: self.top_p,
        })
    }

    /// Sets the frequency penalty.
    ///
    /// The input value will be clampped in between -2.0 and 2.0.
    ///
    /// Number between -2.0 and 2.0.
    /// Positive values penalize new tokens based on their existing frequency in the text so far,
    /// decreasing the model's likelihood to repeat the same line verbatim.
    pub fn frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        // Clamp the value to the valid range
        let frequency_penalty = if
            frequency_penalty < MIN_FREQUENCY_PENALTY ||
            frequency_penalty > MAX_FREQUENCY_PENALTY
        {
            // Clamp the value
            let penalty = frequency_penalty.clamp(MIN_FREQUENCY_PENALTY, MAX_FREQUENCY_PENALTY);

            // Warn the user
            warn!(
                "input value of frequency_penalty is {}, it is now revised to {}",
                frequency_penalty,
                penalty
            );

            penalty
        } else {
            frequency_penalty
        };

        self.frequency_penalty = Some(frequency_penalty);
        self
    }

    /// Sets the max tokens.
    ///
    /// The maximum number of tokens that can be generated in the chat completion.
    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Sets the n.
    ///
    /// It must be a positive integer.
    /// If the input value is 0, then it will be revised to 1.
    ///
    /// How many chat completion choices to generate for each input message.
    /// Note that you will be charged based on the number of generated tokens
    /// across all of the choices. Keep n as 1 to minimize costs.
    pub fn n(mut self, n: u32) -> Self {
        // Revise the value to 1 if it is 0
        let n = if n == 0 {
            // Warn the user
            warn!("input value of n is 0, it is now revised to 1");
            1
        } else {
            n
        };

        self.n = Some(n);
        self
    }

    /// Sets the temperature.
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Sets the top p.
    ///
    /// The input value will be clampped in between 0 and 1.
    ///
    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// We generally recommend altering this or temperature but not both.
    pub fn top_p(mut self, top_p: f32) -> Self {
        // Clamp the value to the valid range
        // Since it is a probability, it should be between 0 and 1
        let top_p = if top_p < MIN_TOP_P || top_p > MAX_TOP_P {
            // Clamp the value
            let p = top_p.clamp(MIN_TOP_P, MAX_TOP_P);

            // Warn the user
            warn!(
                "input value of top_p is {top_p} which is out of range, it is now clamped to {p}"
            );

            p
        } else {
            top_p
        };

        self.top_p = Some(top_p);
        self
    }
}
