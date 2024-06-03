use serde::ser::{ Serialize, Serializer, SerializeStruct };

#[derive(Debug, PartialEq)]
pub struct UserMessage {
    content: String,
    name: Option<String>,
}

pub struct UserMessageBuilder {
    content: String,
    name: Option<String>,
}

impl UserMessage {
    pub fn new<S: AsRef<str>>(content: S) -> Self {
        Self::builder(content).build()
    }

    pub fn builder<S: AsRef<str>>(content: S) -> UserMessageBuilder {
        UserMessageBuilder::new(content)
    }
}

impl Serialize for UserMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        // Calculate number of fields to serialize
        let num_fields = if self.name.is_some() { 3 } else { 2 };

        // Initialize a struct for serializing
        let mut s = serializer.serialize_struct("UserMessage", num_fields)?;

        // Add an additional role field
        s.serialize_field("role", "user")?;

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

impl UserMessageBuilder {
    pub fn new<S: AsRef<str>>(content: S) -> Self {
        Self {
            content: content.as_ref().to_string(),
            name: None,
        }
    }

    pub fn build(self) -> UserMessage {
        UserMessage {
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

#[macro_export]
macro_rules! user_message {
    ($content:literal) => {
        ChatRequestMessage::User(UserMessage::builder($content).build())
    };

    ($content:literal, name = $name:literal) => {
        ChatRequestMessage::User(UserMessage::builder($content).name($name).build())
    };
}

#[cfg(test)]
mod tests {
    use crate::chat::ChatRequestMessage;
    use super::*;

    #[test]
    fn serialize_user_message() {
        let message = UserMessage::builder("Hello.").build();
        let json = serde_json::to_string(&message).unwrap();
        println!("{}", json);
        assert_eq!(json, r#"{"role":"user","content":"Hello."}"#);

        let message = UserMessage::builder("Hello.").name("Isaac").build();
        let json = serde_json::to_string(&message).unwrap();
        println!("{}", json);
        assert_eq!(json, r#"{"role":"user","content":"Hello.","name":"Isaac"}"#);
    }

    #[test]
    fn user_message_macro() {
        assert_eq!(
            user_message!("Hello."),
            ChatRequestMessage::User(UserMessage::builder("Hello.").build())
        );

        assert_eq!(
            user_message!("Hello.", name = "Isaac"),
            ChatRequestMessage::User(UserMessage::builder("Hello.").name("Isaac").build())
        );
    }
}
