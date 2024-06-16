use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum TextToImageModel {
    DallE2,
    DallE3,
}
