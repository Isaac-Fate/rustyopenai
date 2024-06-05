use serde::de::{ self, Deserialize, Visitor };

#[derive(Debug, Clone, PartialEq)]
pub struct ChatCompletionToolCallFunction {
    pub name: String,
    pub arguments: serde_json::Value,
}

struct ChatCompletionToolCallFunctionVisitor;

impl<'de> Visitor<'de> for ChatCompletionToolCallFunctionVisitor {
    type Value = ChatCompletionToolCallFunction;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct ChatCompletionToolCallFunction")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where A: serde::de::MapAccess<'de>
    {
        // Fields to set
        let mut name: Option<String> = None;
        let mut arguments: Option<serde_json::Value> = None;

        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "name" => {
                    // Set name
                    name = Some(map.next_value()?);
                }
                "arguments" => {
                    // Get arguments string
                    let arguments_string: String = map.next_value()?;

                    // Parse the string
                    arguments = match serde_json::from_str::<serde_json::Value>(&arguments_string) {
                        Ok(arguments) => Some(arguments),
                        Err(error) => {
                            return Err(
                                de::Error::custom(
                                    format!("failed to parse received `arguments` to JSON value: {}", error)
                                )
                            );
                        }
                    };
                }
                _ => {}
            }
        }

        // Unwrap name
        let name = if let Some(name) = name {
            name
        } else {
            return Err(de::Error::missing_field("name"));
        };

        // Unwrap arguments
        let arguments = if let Some(arguments) = arguments {
            arguments
        } else {
            return Err(de::Error::missing_field("arguments"));
        };

        Ok(ChatCompletionToolCallFunction {
            name,
            arguments,
        })
    }
}

impl<'de> Deserialize<'de> for ChatCompletionToolCallFunction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        deserializer.deserialize_map(ChatCompletionToolCallFunctionVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn deserialize_chat_completion_tool_call_function() {
        let json = r#"{
            "name": "foo",
            "arguments": "{\"a\": 100}"
        }"#;

        assert_eq!(
            serde_json::from_str::<ChatCompletionToolCallFunction>(json).unwrap(),
            ChatCompletionToolCallFunction {
                name: "foo".to_string(),
                arguments: json!({
                    "a": 100
                }),
            }
        );

        // Invalid JSON
        let json =
            r#"{
            "name": "foo",
            "arguments": "{\"a\": 100}}"
        }"#;

        let result = serde_json::from_str::<ChatCompletionToolCallFunction>(json);
        assert!(result.is_err());

        // Unwrap error
        let error = result.unwrap_err();
        println!("{}", error);
    }
}
