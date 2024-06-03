mod endpoint;
use endpoint::CHAT_COMPLETION_API_ENDPOINT;

mod create_chat_completion;
pub use create_chat_completion::create_chat_completion;

// mod chat_request_body;
// pub use chat_request_body::{ ChatRequestBody, ChatRequestMessage };

mod chat_completion_response;
pub use chat_completion_response::{ ChatCompletionResponse, ChatCompletionChoice };

// mod complete_chat;

pub mod request;
pub use request::ChatRequestBody;
pub use request::message::{
    self,
    ChatRequestMessage,
    SystemMessage,
    UserMessage,
    AssistantMessage,
};
pub use request::tool::{
    self,
    Tool,
    Function,
    FunctionParameter,
    ToolChoice,
    ToolChoiceOption,
    ToolChoiceParticularFunction,
};

mod response;
pub use response::ChatCompletion;
