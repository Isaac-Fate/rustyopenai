use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Model {
    pub id: String,
    pub created: u32,
    pub owned_by: String,
}
