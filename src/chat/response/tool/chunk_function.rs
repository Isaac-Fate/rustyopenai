use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionChunkToolCallFunction {
    pub name: Option<String>,

    #[serde(rename = "arguments")]
    pub arguments_string: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_chat_completion_chunk_tool_call_function() {
        let json = r#"{
            "name": "foo",
            "arguments": "{\"a\": 100}"
        }"#;

        assert_eq!(
            serde_json::from_str::<ChatCompletionChunkToolCallFunction>(json).unwrap(),
            ChatCompletionChunkToolCallFunction {
                name: Some("foo".to_string()),
                arguments_string: Some("{\"a\": 100}".to_string()),
            }
        );
    }
}
