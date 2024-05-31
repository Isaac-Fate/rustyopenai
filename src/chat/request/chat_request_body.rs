use serde::Serialize;
use log::warn;
use super::{ message::ChatRequestMessage, tool::{ Tool, ToolChoice } };

const MIN_FREQUENCY_PENALTY: f32 = -2.0;
const MAX_FREQUENCY_PENALTY: f32 = 2.0;
const MIN_TOP_P: f32 = 0.0;
const MAX_TOP_P: f32 = 1.0;
const MAX_NUM_TOOLS: usize = 128;

#[derive(Debug, Serialize)]
pub struct ChatRequestBody {
    model: String,
    messages: Vec<ChatRequestMessage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<ToolChoice>,
}

impl ChatRequestBody {
    pub fn builder<S: AsRef<str>>(
        model: S,
        messages: Vec<ChatRequestMessage>
    ) -> ChatRequestBodyBuilder {
        ChatRequestBodyBuilder::new(model, messages)
    }
}

pub struct ChatRequestBodyBuilder {
    model: String,
    messages: Vec<ChatRequestMessage>,
    frequency_penalty: Option<f32>,
    max_tokens: Option<u32>,
    n: Option<u32>,
    temperature: Option<f32>,
    top_p: Option<f32>,

    tools: Option<Vec<Tool>>,
    tool_choice: Option<ToolChoice>,
}

impl ChatRequestBodyBuilder {
    /// Creates a new builder with `None` values for all fields.
    pub fn new<S: AsRef<str>>(model: S, messages: Vec<ChatRequestMessage>) -> Self {
        Self {
            model: model.as_ref().to_string(),
            messages,
            frequency_penalty: None,
            max_tokens: None,
            n: None,
            temperature: None,
            top_p: None,

            tools: None,
            tool_choice: None,
        }
    }

    /// Builds the request body.
    pub fn build(self) -> ChatRequestBody {
        ChatRequestBody {
            messages: self.messages,
            model: self.model,
            frequency_penalty: self.frequency_penalty,
            max_tokens: self.max_tokens,
            n: self.n,
            temperature: self.temperature,
            top_p: self.top_p,

            tools: self.tools,
            tool_choice: self.tool_choice,
        }
    }

    /// Sets the frequency penalty.
    ///
    /// The input value will be clampped in between -2.0 and 2.0.
    ///
    /// Number between -2.0 and 2.0.
    /// Positive values penalize new tokens based on their existing frequency in the text so far,
    /// decreasing the model's likelihood to repeat the same line verbatim.
    pub fn frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        // Clamp the value to the valid range
        let frequency_penalty = if
            frequency_penalty < MIN_FREQUENCY_PENALTY ||
            frequency_penalty > MAX_FREQUENCY_PENALTY
        {
            // Clamp the value
            let penalty = frequency_penalty.clamp(MIN_FREQUENCY_PENALTY, MAX_FREQUENCY_PENALTY);

            // Warn the user
            warn!(
                "input value of frequency_penalty is {}, it is now revised to {}",
                frequency_penalty,
                penalty
            );

            penalty
        } else {
            frequency_penalty
        };

        self.frequency_penalty = Some(frequency_penalty);
        self
    }

    /// Sets the max tokens.
    ///
    /// The maximum number of tokens that can be generated in the chat completion.
    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Sets the n.
    ///
    /// It must be a positive integer.
    /// If the input value is 0, then it will be revised to 1.
    ///
    /// How many chat completion choices to generate for each input message.
    /// Note that you will be charged based on the number of generated tokens
    /// across all of the choices. Keep n as 1 to minimize costs.
    pub fn n(mut self, n: u32) -> Self {
        // Revise the value to 1 if it is 0
        let n = if n == 0 {
            // Warn the user
            warn!("input value of n is 0, it is now revised to 1");
            1
        } else {
            n
        };

        self.n = Some(n);
        self
    }

    /// Sets the temperature.
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Sets the top p.
    ///
    /// The input value will be clampped in between 0 and 1.
    ///
    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// We generally recommend altering this or temperature but not both.
    pub fn top_p(mut self, top_p: f32) -> Self {
        // Clamp the value to the valid range
        // Since it is a probability, it should be between 0 and 1
        let top_p = if top_p < MIN_TOP_P || top_p > MAX_TOP_P {
            // Clamp the value
            let p = top_p.clamp(MIN_TOP_P, MAX_TOP_P);

            // Warn the user
            warn!(
                "input value of top_p is {top_p} which is out of range, it is now clamped to {p}"
            );

            p
        } else {
            top_p
        };

        self.top_p = Some(top_p);
        self
    }

    /// Sets the tools.
    ///
    /// If there are more than 128 tools, then only the first 128 tools will be kept.
    ///
    /// A list of tools the model may call. Currently, only functions are supported as a tool.
    /// Use this to provide a list of functions the model may generate JSON inputs for.
    /// A max of 128 functions are supported.
    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        // Only keep the first 128 functions if the nunber of provided functions exceeds that number
        let mut tools = tools;
        if tools.len() > MAX_NUM_TOOLS {
            tools = tools.into_iter().take(MAX_NUM_TOOLS).collect();

            // Warn the user
            warn!("too many provided tools, only the first {MAX_NUM_TOOLS} are kept");
        }

        self.tools = Some(tools);
        self
    }

    /// Sets the tool choice.
    ///
    /// Controls which (if any) tool is called by the model.
    pub fn tool_choice(mut self, tool_choice: ToolChoice) -> Self {
        self.tool_choice = Some(tool_choice);
        self
    }
}

/// Creates a vector of function parameters.
///
///
/// ```
/// use rustyopenai::prelude::*;
/// use serde_json::json;
///
/// // All parameters are required
/// let parameters = function_parameters! {
///     "id": json!({ "type": "string" }),
///     "name": json!({ "type": "string" }),
/// };
///
/// // All parameters are optional
/// let parameters = function_parameters! {
///     optional
///     "email": json!({ "type": "string" }),
///     "age": json!({ "type": "number" }),
/// };
///
/// // There are both required and optional parameters
/// // Use a semicolon ; to separate them
/// let parameters = function_parameters! {
///     "id": json!({ "type": "string" }),
///     "name": json!({ "type": "string" });
///     "email": json!({ "type": "string" }),
///     "age": json!({ "type": "number" }),
/// };
/// ```
#[macro_export]
macro_rules! function_parameters {
    // All parameters are required
    ($($required_parameter_name:literal: $required_parameter_schema:expr),* $(,)?) => {
        vec![
            $(
                function_parameter!($required_parameter_name: $required_parameter_schema),
            )*
        ]
    };

    // All parameters are optional
    (optional $($optional_parameter_name:literal: $optional_parameter_schema:expr),* $(,)?) => {
        vec![
            $(
                function_parameter!(optional $optional_parameter_name: $optional_parameter_schema),
            )*
        ]
    };

    // There is at least one required parameter, and
    // at least one optional parameter
    (
        $($required_parameter_name:literal: $required_parameter_schema:expr),+;
        $($optional_parameter_name:literal: $optional_parameter_schema:expr),+ $(,)?
    ) => {
        {
            // Add the required parameters
            let mut parameters = vec![
                $(
                    function_parameter!($required_parameter_name: $required_parameter_schema),
                )*
            ];

            // Add the optional parameters
            parameters.extend(vec![
                $(
                    function_parameter!(optional $optional_parameter_name: $optional_parameter_schema),
                )*
            ]);

            parameters
        }
    };
}

#[macro_export]
macro_rules! function_parameter {
    ($name:literal: $schema:expr) => {
        FunctionParameter::new(
            $name,
            true,
            $schema
        )
    };

    (optional $name:literal: $schema:expr) => {
        FunctionParameter::new(
            $name,
            false,
            $schema
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use crate::{
        chat::request::{
            message::{ AssistantMessage, UserMessage },
            tool::{ Function, FunctionParameter, ToolChoice, ToolChoiceOption },
        },
        tool_choice,
        function_parameter,
        function_parameters,
    };

    #[test]
    fn chat_request_body() {
        let body = ChatRequestBody::builder(
            "gpt-3.5-turbo",
            vec![
                ChatRequestMessage::User(UserMessage::new("Hello?")),
                ChatRequestMessage::Assistant(
                    AssistantMessage::builder().content("Hi, how can I help you?").build()
                )
            ]
        ).build();

        let json = serde_json::to_value(body).unwrap();
        println!("{}", serde_json::to_string_pretty(&json).unwrap())
    }

    #[test]
    fn tools() {
        let body = ChatRequestBody::builder(
            "gpt-3.5-turbo",
            vec![ChatRequestMessage::User(UserMessage::new("Use the provided function."))]
        )
            .tools(
                vec![
                    Tool::Function(
                        Function::builder("foo")
                            .description("A dummy function")
                            .parameters(
                                vec![
                                    FunctionParameter::new("a", true, json!({"type": "number"})),
                                    FunctionParameter::new(
                                        "b",
                                        true,
                                        json!({"type": "array", "items": "number"})
                                    )
                                ]
                            )
                            .build()
                    )
                ]
            )
            .build();

        let json = serde_json::to_value(body).unwrap();
        println!("{}", serde_json::to_string_pretty(&json).unwrap())
    }

    #[test]
    fn large_request_body() {
        // Prepare request body
        let request_body = ChatRequestBody::builder(
            "gpt-3.5-turbo",
            vec![ChatRequestMessage::User(UserMessage::new("What is the breaking news today?"))]
        )
            .tools(
                vec![
                    Tool::Function(
                        Function::builder("search_on_web")
                            .description("Search for information based on a query.")
                            .parameters(
                                vec![
                                    FunctionParameter::new(
                                        "query",
                                        true,
                                        json!({"type": "string", "description": "The query to search for."})
                                    ),
                                    FunctionParameter::new(
                                        "browser",
                                        true,
                                        json!({"type": "string", "enum": ["chrome", "firefox"], "description": "The browser to use."})
                                    )
                                ]
                            )
                            .build()
                    )
                ]
            )
            .tool_choice(ToolChoice::Option(ToolChoiceOption::Auto))
            .build();

        println!("{}", serde_json::to_string_pretty(&request_body).unwrap());
    }

    #[test]
    fn build_request_body_with_macros() {
        // Prepare request body
        let request_body = ChatRequestBody::builder(
            "gpt-3.5-turbo",
            vec![ChatRequestMessage::User(UserMessage::new("What is the breaking news today?"))]
        )
            .tools(
                vec![
                    Tool::Function(
                        Function::builder("search_on_web")
                            .description("Search for information based on a query.")
                            .parameters(
                                vec![
                                    function_parameter!("query": json!({"type": "string", "description": "The query to search for."})),
                                    function_parameter!(optional "browser": json!({"type": "string", "enum": ["chrome", "firefox"], "description": "The browser to use."}))
                                ]
                            )
                            .build()
                    )
                ]
            )
            .tool_choice(tool_choice!(auto))
            .build();

        println!("{}", serde_json::to_string_pretty(&request_body).unwrap());
    }

    #[test]
    fn function_parameters_macro() {
        // All parameters are required
        // There is a trailing comma
        let parameters =
            function_parameters![
            "query": json!({"type": "string", "description": "The query to search for."}), 
            "num_results": json!({"type": "number", "description": "Number of search results to return."}),
        ];
        println!("{}", serde_json::to_string_pretty(&parameters).unwrap());

        // All parameters are required
        // The trailing comma may be omitted
        let parameters =
            function_parameters![
            "query": json!({"type": "string", "description": "The query to search for."}), 
            "num_results": json!({"type": "number", "description": "Number of search results to return."})
        ];
        println!("{}", serde_json::to_string_pretty(&parameters).unwrap());

        // All parameters are optional
        // There is a trailing comma
        let parameters =
            function_parameters! {
                optional
                "browser": json!({"type": "string", "enum": ["chrome", "firefox"], "description": "The browser to use."}),
            };
        println!("{}", serde_json::to_string_pretty(&parameters).unwrap());

        // All parameters are optional
        // The trailing comma may be omitted
        let parameters =
            function_parameters! {
                optional
                "browser": json!({"type": "string", "enum": ["chrome", "firefox"], "description": "The browser to use."})
            };

        println!("{}", serde_json::to_string_pretty(&parameters).unwrap());

        // There are both required and optional parameters
        let parameters =
            function_parameters! {
                "query": json!({"type": "string", "description": "The query to search for."}), 
                "num_results": json!({"type": "number", "description": "Number of search results to return."});
                "browser": json!({"type": "string", "enum": ["chrome", "firefox"], "description": "The browser to use."}),
            };
        println!("{}", serde_json::to_string_pretty(&parameters).unwrap());
    }
}
