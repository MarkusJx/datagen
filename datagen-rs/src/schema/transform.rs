#[cfg(feature = "generate")]
use crate::generate::current_schema::CurrentSchema;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::GeneratedSchema;
#[cfg(feature = "generate")]
use crate::util::types::Result;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "generate")]
use std::sync::Arc;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Transform {
    /// The path of the plugin which will be used to transform the data
    pub name: String,
    /// The arguments which will be passed to the plugin
    pub args: Option<Value>,
}

#[cfg(feature = "generate")]
impl Transform {
    pub fn transform(
        self,
        schema: Arc<CurrentSchema>,
        value: Arc<GeneratedSchema>,
    ) -> Result<Arc<GeneratedSchema>> {
        schema.get_plugin(&self.name)?.transform(
            schema.clone(),
            value,
            self.args.unwrap_or(Value::Null),
        )
    }
}
