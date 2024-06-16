use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Image {
    pub revised_prompt: String,
    pub image: ImageContent,
}

#[derive(Debug, Deserialize)]
pub enum ImageContent {
    Url(String),
    Base64(String),
}
