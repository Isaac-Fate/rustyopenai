use lazy_static::lazy_static;
use crate::{ Result, Error, OpenAIClient };
use super::super::{ IMAGES_API_ENDPOINT, ImageGenerationRequestBody };

lazy_static! {
    static ref GENERATE_IMAGE_API_ENDPOINT: String = format!(
        "{}/{}",
        IMAGES_API_ENDPOINT,
        "generations"
    );
}

pub async fn generate_images(
    client: &OpenAIClient,
    request_body: ImageGenerationRequestBody
) -> Result<()> {
    todo!()
}
