use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ImageResponseFormat {
    Url,
    Base64,
}
