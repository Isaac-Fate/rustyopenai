use anyhow::{ Result, anyhow };
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct EmbeddingRequestBody {
    pub input: EmbeddingRequestInput,
    pub model: String,
    pub encoding_format: EmbeddingEncodingFormat,
    pub user: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum EmbeddingRequestInput {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Serialize)]
pub enum EmbeddingEncodingFormat {
    #[serde(rename = "float")]
    Vector,

    #[serde(rename = "base64")]
    Base64,
}

impl EmbeddingRequestBody {
    pub fn new(
        input: EmbeddingRequestInput,
        model: String,
        encoding_format: EmbeddingEncodingFormat,
        user: Option<String>
    ) -> Self {
        Self {
            input,
            model,
            encoding_format,
            user,
        }
    }

    pub fn builder() -> EmbeddingRequestBodyBuilder {
        EmbeddingRequestBodyBuilder::new()
    }
}

pub struct EmbeddingRequestBodyBuilder {
    input: Option<EmbeddingRequestInput>,
    model: Option<String>,
    encoding_format: EmbeddingEncodingFormat,
    user: Option<String>,
}

impl EmbeddingRequestBodyBuilder {
    pub fn new() -> Self {
        Self {
            input: None,
            model: None,
            encoding_format: EmbeddingEncodingFormat::Vector,
            user: None,
        }
    }

    pub fn input(mut self, input: EmbeddingRequestInput) -> Self {
        self.input = Some(input);
        self
    }

    /// Set the input to a single text.
    pub fn single_text<S: AsRef<str>>(mut self, text: S) -> Self {
        self.input = Some(EmbeddingRequestInput::Single(text.as_ref().to_string()));
        self
    }

    /// Set the input to multiple texts.
    pub fn multiple_texts<S: AsRef<str>>(mut self, texts: Vec<S>) -> Self {
        self.input = Some(
            EmbeddingRequestInput::Multiple(
                texts
                    .iter()
                    .map(|text| text.as_ref().to_string())
                    .collect()
            )
        );
        self
    }

    pub fn model<S: AsRef<str>>(mut self, model: S) -> Self {
        self.model = Some(model.as_ref().to_string());
        self
    }

    pub fn encoding_format(mut self, encoding_format: EmbeddingEncodingFormat) -> Self {
        self.encoding_format = encoding_format;
        self
    }

    pub fn user<S: AsRef<str>>(mut self, user: S) -> Self {
        self.user = Some(user.as_ref().to_string());
        self
    }

    pub fn build(self) -> Result<EmbeddingRequestBody> {
        Ok(EmbeddingRequestBody {
            input: self.input.ok_or(anyhow!("'input' is required"))?,
            model: self.model.ok_or(anyhow!("'model' is required"))?,
            encoding_format: self.encoding_format,
            user: self.user,
        })
    }
}
