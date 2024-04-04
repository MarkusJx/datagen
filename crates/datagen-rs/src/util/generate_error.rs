use crate::generate::datagen_context::DatagenContextRef;
use crate::generate::schema_path::SchemaPath;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct GenerateError {
    path: String,
    message: String,
}

impl GenerateError {
    pub fn new(schema: &DatagenContextRef, message: &str) -> Self {
        Self {
            path: schema.path().as_ref().map_or_else(
                |e| format!("Failed to get schema path: {e}"),
                SchemaPath::to_normalized_path,
            ),
            message: message.to_string(),
        }
    }
}

impl Display for GenerateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error generating schema at path '{}': {}",
            self.path, self.message
        )
    }
}

impl Error for GenerateError {}
