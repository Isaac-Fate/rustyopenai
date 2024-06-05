use serde::{ Serialize, Serializer, ser::SerializeMap };
use super::Function;

#[derive(Debug, PartialEq)]
pub enum Tool {
    Function(Function),
}

impl Serialize for Tool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            Tool::Function(function) => {
                // Initialize the map
                let mut map = serializer.serialize_map(Some(2))?;

                // Add an additional type field
                map.serialize_entry("type", "function")?;

                // Serialize the function
                map.serialize_entry("function", function)?;

                // End serializing
                map.end()
            }
        }
    }
}
