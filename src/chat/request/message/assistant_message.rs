use serde::{ Serialize, Serializer, ser::SerializeStruct };
use crate::chat::request::tool::ToolCall;

#[derive(Debug)]
pub struct AssistantMessage {
    content: Option<String>,
    name: Option<String>,
    tool_calls: Option<Vec<ToolCall>>,
}

pub struct AssistantMessageBuilder {
    content: Option<String>,
    name: Option<String>,
    tool_calls: Option<Vec<ToolCall>>,
}

impl AssistantMessage {
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn builder() -> AssistantMessageBuilder {
        AssistantMessageBuilder::new()
    }
}

impl Serialize for AssistantMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        // Calculate number of fields to serialize
        let mut num_fields = 1;
        if self.content.is_some() {
            num_fields += 1;
        }
        if self.name.is_some() {
            num_fields += 1;
        }
        if self.tool_calls.is_some() {
            num_fields += 1;
        }

        // Initialize a struct for serializing
        let mut s = serializer.serialize_struct("AssistantMessage", num_fields)?;

        // Add an additional role field
        s.serialize_field("role", "assistant")?;

        // Serialize content
        if self.content.is_some() {
            s.serialize_field("content", &self.content)?;
        }

        // Serialize name
        if self.name.is_some() {
            s.serialize_field("name", &self.name)?;
        }

        // Serialize tool calls
        if self.tool_calls.is_some() {
            s.serialize_field("tool_calls", &self.tool_calls)?;
        }

        // End serializing
        s.end()
    }
}

impl AssistantMessageBuilder {
    pub fn new() -> Self {
        Self {
            content: None,
            name: None,
            tool_calls: None,
        }
    }

    pub fn content<S: AsRef<str>>(mut self, content: S) -> Self {
        self.content = Some(content.as_ref().to_string());
        self
    }

    pub fn build(self) -> AssistantMessage {
        AssistantMessage {
            content: self.content,
            name: self.name,
            tool_calls: self.tool_calls,
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

    pub fn tool_calls(mut self, tool_calls: Vec<ToolCall>) -> Self {
        self.tool_calls = Some(tool_calls);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chat::request::tool::ToolCallFunction;

    #[test]
    fn serialize_assistant_message() {
        let message = AssistantMessage::builder().content("How may I help you?").build();
        let json = serde_json::to_string(&message).unwrap();
        println!("{}", json);
        assert_eq!(json, r#"{"role":"assistant","content":"How may I help you?"}"#);

        let message = AssistantMessage::builder()
            .content("How may I help you?")
            .name("bot A")
            .build();
        let json = serde_json::to_string(&message).unwrap();
        println!("{}", json);
        assert_eq!(json, r#"{"role":"assistant","content":"How may I help you?","name":"bot A"}"#);

        let message = AssistantMessage::builder()
            .content("How may I help you?")
            .name("bot A")
            .tool_calls(vec![ToolCall::new("tool A", ToolCallFunction::new("foo", "{\"a\":42}"))])
            .build();
        let json = serde_json::to_string(&message).unwrap();
        println!("{}", json);
        assert_eq!(
            json,
            r#"{"role":"assistant","content":"How may I help you?","name":"bot A","tool_calls":[{"type":"function","id":"tool A","function":{"name":"foo","arguments":"{\"a\":42}"}}]}"#
        );
    }
}
