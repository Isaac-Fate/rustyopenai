use serde::{ Serialize, Serializer, ser::SerializeStruct };

#[derive(Debug)]
pub struct SystemMessage {
    content: String,
    name: Option<String>,
}

pub struct SystemMessageBuilder {
    content: String,
    name: Option<String>,
}

impl SystemMessage {
    pub fn new<S: AsRef<str>>(content: S) -> Self {
        Self::builder(content).build()
    }

    pub fn builder<S: AsRef<str>>(content: S) -> SystemMessageBuilder {
        SystemMessageBuilder::new(content)
    }
}

impl Serialize for SystemMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        // Calculate number of fields to serialize
        let num_fields = if self.name.is_some() { 3 } else { 2 };

        // Initialize a struct for serializing
        let mut s = serializer.serialize_struct("SystemMessage", num_fields)?;

        // Add an additional role field
        s.serialize_field("role", "system")?;

        // Serialize content
        s.serialize_field("content", &self.content)?;

        // Serialize name if it exists
        if self.name.is_some() {
            s.serialize_field("name", &self.name)?;
        }

        // End serializing
        s.end()
    }
}

impl SystemMessageBuilder {
    pub fn new<S: AsRef<str>>(content: S) -> Self {
        Self {
            content: content.as_ref().to_string(),
            name: None,
        }
    }

    pub fn build(self) -> SystemMessage {
        SystemMessage {
            content: self.content,
            name: self.name,
        }
    }

    /// Sets name.
    ///
    /// An optional name for the participant.
    /// Provides the model information to differentiate between participants of the same role.
    pub fn name<S: AsRef<str>>(mut self, name: S) -> Self {
        self.name = Some(name.as_ref().to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_system_message() {
        let message = SystemMessage::builder("Your are a helpful assistant.").build();
        let json = serde_json::to_string(&message).unwrap();
        println!("{}", json);
        assert_eq!(json, r#"{"role":"system","content":"Your are a helpful assistant."}"#);

        let message = SystemMessage::builder("Your are a helpful assistant.").name("bot A").build();
        let json = serde_json::to_string(&message).unwrap();
        println!("{}", json);
        assert_eq!(
            json,
            r#"{"role":"system","content":"Your are a helpful assistant.","name":"bot A"}"#
        );
    }
}
