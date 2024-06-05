use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ChatCompletionFinishReason {
    Stop,
    Length,
    ContentFilter,
    ToolCalls,
}
