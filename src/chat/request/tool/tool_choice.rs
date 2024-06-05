use serde::{ Serialize, Serializer, ser::SerializeMap };
use serde_json::json;

#[derive(Debug, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ToolChoice {
    Option(ToolChoiceOption),
    ParticularTool(ToolChoiceParticularFunction),
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoiceOption {
    /// The model will not call any tool and instead generates a message.
    None,

    /// The model can pick between generating a message or calling one or more tools.
    Auto,

    /// The model must call one or more tools.
    Required,
}

#[derive(Debug, PartialEq)]
pub struct ToolChoiceParticularFunction {
    name: String,
}

impl ToolChoiceParticularFunction {
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        Self { name: name.as_ref().to_string() }
    }
}

impl Serialize for ToolChoiceParticularFunction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        // Initialize a map with two entries
        let mut map = serializer.serialize_map(Some(2))?;

        // Tool type is function
        map.serialize_entry("type", "function")?;

        // Function name
        map.serialize_entry("function", &json!({"name": self.name}))?;

        // End serializing
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_particular_function() {
        let function = ToolChoiceParticularFunction {
            name: "foo".to_string(),
        };
        let json = serde_json::to_string(&function).unwrap();
        println!("{}", json);
        assert_eq!(json, r#"{"type":"function","function":{"name":"foo"}}"#);
    }
}
