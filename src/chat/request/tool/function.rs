use std::collections::HashMap;
use serde::{ Serialize, Serializer, ser::SerializeMap };
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct Function {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<FunctionParameters>,
}

/// A wrapper around a vector of function parameters.
/// This struct is invented so that we may define custom serialization.
#[derive(Debug)]
pub struct FunctionParameters(Vec<FunctionParameter>);

#[derive(Debug, Serialize)]
pub struct FunctionParameter {
    pub name: String,
    pub required: bool,
    pub schema: Value,
}

#[derive(Debug)]
pub struct FunctionBuilder {
    name: String,
    description: Option<String>,
    parameters: Option<FunctionParameters>,
}

impl Function {
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        Self::builder(name).build()
    }

    pub fn builder<S: AsRef<str>>(name: S) -> FunctionBuilder {
        FunctionBuilder::new(name)
    }
}

impl FunctionBuilder {
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        Self { name: name.as_ref().to_string(), description: None, parameters: None }
    }

    pub fn build(self) -> Function {
        Function { name: self.name, description: self.description, parameters: self.parameters }
    }

    pub fn description<S: AsRef<str>>(mut self, description: S) -> Self {
        self.description = Some(description.as_ref().to_string());
        self
    }

    pub fn parameters(mut self, parameters: Vec<FunctionParameter>) -> Self {
        self.parameters = Some(FunctionParameters(parameters));
        self
    }
}

impl FunctionParameter {
    pub fn new<S: AsRef<str>>(name: S, required: bool, schema: Value) -> Self {
        Self { name: name.as_ref().to_string(), required, schema }
    }
}

impl Serialize for FunctionParameters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        // Begin to serialize a map
        // * Keep in mind that `self.0` is the inner vector
        let mut map = serializer.serialize_map(Some(self.0.len()))?;

        map.serialize_entry("type", "object")?;

        let mut properties: HashMap<String, Value> = HashMap::new();

        let mut required_parameter_names: Vec<String> = vec![];

        // Serialize each function parameter
        for parameter in self.0.iter() {
            // Add the parameter name and description
            properties.insert(parameter.name.clone(), parameter.schema.clone());

            // Add the required parameter name
            if parameter.required {
                required_parameter_names.push(parameter.name.clone());
            }
        }

        map.serialize_entry("properties", &properties)?;

        // Serialize the required parameter names if there are any
        if !required_parameter_names.is_empty() {
            map.serialize_entry("required", &required_parameter_names)?;
        }

        // End serializing
        map.end()
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
    use serde_json::json;
    use super::*;

    #[test]
    fn serialize_function() {
        let function = Function {
            name: "foo".to_string(),
            description: Some("bar".to_string()),
            parameters: Some(
                FunctionParameters(
                    vec![
                        FunctionParameter::new("file_path", true, json!({"type": "string"})),
                        FunctionParameter::new("output_dir_path", true, json!({"type": "string"})),
                        FunctionParameter::new("timeout", false, json!({"type": "number"}))
                    ]
                )
            ),
        };

        let json_string = serde_json::to_string_pretty(&function).unwrap();

        println!("{}", json_string)
    }

    #[test]
    fn serialize_function_parameters() {
        let function_parameters = FunctionParameters(
            vec![
                FunctionParameter::new("file_path", true, json!({"type": "string"})),
                FunctionParameter::new("output_dir_path", true, json!({"type": "string"})),
                FunctionParameter::new("timeout", false, json!({"type": "number"}))
            ]
        );

        let json_string = serde_json::to_string(&function_parameters).unwrap();

        println!("{}", json_string)
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
}
