use serde::{ Deserialize, Deserializer };

#[derive(Debug, Deserialize)]
pub struct EmbeddingReponse {
    pub object: String,
    pub data: Vec<Embedding>,
    pub model: String,
    pub usage: EmbeddingTokenUsage,
}

#[derive(Debug, Deserialize)]
pub struct Embedding {
    pub index: u32,

    #[serde(deserialize_with = "EmbeddingContent::deserialize")]
    pub embedding: EmbeddingContent,

    pub object: String,
}

#[derive(Debug)]
pub enum EmbeddingContent {
    Vector(Vec<f32>),
    Base64(String),
}

impl<'de> Deserialize<'de> for EmbeddingContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_any(EmbeddingContentVisitor)
    }
}

struct EmbeddingContentVisitor;

impl<'de> serde::de::Visitor<'de> for EmbeddingContentVisitor {
    type Value = EmbeddingContent;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A vector or a base64 string")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where A: serde::de::SeqAccess<'de>
    {
        // Create a new vector to hold the sequence
        let mut vec = Vec::new();

        // Collect each element in the sequence
        while let Some(value) = seq.next_element()? {
            vec.push(value);
        }

        Ok(EmbeddingContent::Vector(vec))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: serde::de::Error {
        Ok(EmbeddingContent::Base64(value.to_string()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> where E: serde::de::Error {
        Ok(EmbeddingContent::Base64(value))
    }
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingTokenUsage {
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}
