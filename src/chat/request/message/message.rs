use serde::Serialize;
use super::{ SystemMessage, UserMessage, AssistantMessage };

#[derive(Debug, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ChatRequestMessage {
    System(SystemMessage),
    User(UserMessage),
    Assistant(AssistantMessage),
}
