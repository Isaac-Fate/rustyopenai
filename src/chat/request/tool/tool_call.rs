use serde::{ Serialize, Serializer, ser::SerializeStruct };

#[derive(Debug)]
pub struct ToolCall {
    pub id: String,
    pub function: ToolCallFunction,
}

#[derive(Debug, Serialize)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String,
}

impl ToolCall {
    pub fn new<S: AsRef<str>>(id: S, function: ToolCallFunction) -> Self {
        Self {
            id: id.as_ref().to_string(),
            function,
        }
    }
}

impl ToolCallFunction {
    pub fn new<S: AsRef<str>>(name: S, arguments: S) -> Self {
        Self {
            name: name.as_ref().to_string(),
            arguments: arguments.as_ref().to_string(),
        }
    }
}

impl Serialize for ToolCall {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        // Initialize a struct for serializing
        let mut s = serializer.serialize_struct("ToolCall", 3)?;

        // Add an additional type field
        s.serialize_field("type", "function")?;

        // Serialize other fields
        s.serialize_field("id", &self.id)?;
        s.serialize_field("function", &self.function)?;

        // End serializing
        s.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_tool_call() {
        let tool_call = ToolCall::new("123", ToolCallFunction::new("foo", "bar"));
        let json = serde_json::to_string(&tool_call).unwrap();
        assert_eq!(
            json,
            r#"{"type":"function","id":"123","function":{"name":"foo","arguments":"bar"}}"#
        );
    }
}
