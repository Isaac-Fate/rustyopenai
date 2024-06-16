use serde::Serialize;
use super::{ TextToImageModel, ImageResponseFormat };

#[derive(Debug, Serialize)]
pub struct ImageGenerationRequestBody {
    prompt: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<TextToImageModel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<ImageResponseFormat>,
}

pub struct ImageGenerationRequestBodyBuilder {
    prompt: String,
    model: Option<TextToImageModel>,
    n: Option<u32>,
    response_format: Option<ImageResponseFormat>,
}

impl ImageGenerationRequestBodyBuilder {
    pub fn new(prompt: String) -> Self {
        Self {
            prompt,
            model: None,
            n: None,
            response_format: None,
        }
    }

    pub fn build(self) -> ImageGenerationRequestBody {
        ImageGenerationRequestBody {
            prompt: self.prompt,
            model: self.model,
            n: self.n,
            response_format: self.response_format,
        }
    }

    pub fn model(mut self, model: TextToImageModel) -> Self {
        self.model = Some(model);
        self
    }

    pub fn n(mut self, n: u32) -> Self {
        self.n = Some(n);
        self
    }

    pub fn response_format(mut self, response_format: ImageResponseFormat) -> Self {
        self.response_format = Some(response_format);
        self
    }
}
