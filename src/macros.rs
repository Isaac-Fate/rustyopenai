#[allow(unused)]
use crate::prelude::*;

#[macro_export]
macro_rules! system_message {
    ($content:literal) => {
        ChatRequestMessage::System(SystemMessage::builder($content).build())
    };

    ($content:literal, name = $name:literal) => {
        ChatRequestMessage::System(SystemMessage::builder($content).name($name).build())
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

/// Creates a vector of function parameters.
///
///
/// ```
/// use rustyopenai::prelude::*;
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

/// Creates a function tool.
///
/// ```
/// use rustyopenai::prelude::*;
///
/// // Function with just a name
/// let function = function!("foo");
///
/// // Function with name and description
/// let function = function!("foo", description = "A dummy function.");
///
/// // Function with name and parameters
/// let function = function!("foo", parameters = function_parameters![
///     "a": json!({"type": "number"}),
///     "b": json!({"type": "string"})
/// ]);
///
/// // Function with name, description and parameters
/// let function = function!(
///     "foo",
///     description = "A dummy function.",
///     parameters = function_parameters![
///         "a": json!({"type": "number"}),
///         "b": json!({"type": "string"})
///     ]
/// );
/// ```
#[macro_export]
macro_rules! function {
    ($name:literal) => {
        Tool::Function(Function::builder($name).build())
    };

    ($name:literal, description = $description:literal) => {
        Tool::Function(Function::builder($name).description($description).build())
    };

    ($name:literal, parameters = $parameters:expr) => {
        Tool::Function(Function::builder($name).parameters($parameters).build())
    };

    ($name:literal, description = $description:literal, parameters = $parameters:expr) => {
        Tool::Function(Function::builder($name).description($description).parameters($parameters).build())
    };
}

/// Creates a ToolChoice from a string.
///
/// ```
/// use rustyopenai::prelude::*;
///
/// // The model can pick between generating a message or calling one or more tools
/// let tool_choice = tool_choice!(auto);
///
/// // The model will not call any tool and instead generates a message.
/// let tool_choice = tool_choice!(none);
///
/// // The model must call one or more tools
/// let tool_choice = tool_choice!(required);
///
/// // The model must use the specified function named foo
/// let tool_choice = tool_choice!("foo");
///
/// ```
#[macro_export]
macro_rules! tool_choice {
    (auto) => { ToolChoice::Option(ToolChoiceOption::Auto) };
    (none) => { ToolChoice::Option(ToolChoiceOption::None) };
    (required) => { ToolChoice::Option(ToolChoiceOption::Required) };
    ($name:literal) => {
        ToolChoice::ParticularTool(
            ToolChoiceParticularFunction::new($name),
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_message_macro() {
        assert_eq!(
            system_message!("Your are a helpful assistant."),
            ChatRequestMessage::System(
                SystemMessage::builder("Your are a helpful assistant.").build()
            )
        );

        assert_eq!(
            system_message!("Your are a helpful assistant.", name = "Ferris"),
            ChatRequestMessage::System(
                SystemMessage::builder("Your are a helpful assistant.").name("Ferris").build()
            )
        );
    }

    #[test]
    fn use_function_parameters_macro() {
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

    #[test]
    fn function_macro() {
        assert_eq!(function!("foo"), Tool::Function(Function::new("foo")));

        assert_eq!(
            function!("foo", description = "A dummy function."),
            Tool::Function(Function::builder("foo").description("A dummy function.").build())
        );

        assert_eq!(
            function!(
                "foo",
                parameters = function_parameters![
                    "a": json!({"type": "number"}), 
                    "b": json!({"type": "string"})
                ]
            ),
            Tool::Function(
                Function::builder("foo")
                    .parameters(
                        function_parameters![
                    "a": json!({"type": "number"}), 
                    "b": json!({"type": "string"})
                ]
                    )
                    .build()
            )
        );

        assert_eq!(
            function!(
                "foo",
                description = "A dummy function.",
                parameters = function_parameters![
                    "a": json!({"type": "number"}), 
                    "b": json!({"type": "string"})
                ]
            ),

            Tool::Function(
                Function::builder("foo")
                    .description("A dummy function.")
                    .parameters(
                        function_parameters![
                    "a": json!({"type": "number"}), 
                    "b": json!({"type": "string"})
                ]
                    )
                    .build()
            )
        );
    }

    #[test]
    fn tool_choice_macro() {
        assert_eq!(tool_choice!(none), ToolChoice::Option(ToolChoiceOption::None));

        assert_eq!(tool_choice!(auto), ToolChoice::Option(ToolChoiceOption::Auto));

        assert_eq!(tool_choice!(required), ToolChoice::Option(ToolChoiceOption::Required));

        assert_eq!(
            tool_choice!("foo"),
            ToolChoice::ParticularTool(ToolChoiceParticularFunction::new("foo"))
        )
    }
}
