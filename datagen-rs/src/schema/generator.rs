#[cfg(feature = "generate")]
use crate::generate::current_schema::CurrentSchema;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::{GeneratedSchema, IntoGeneratedArc};
use crate::schema::transform::Transform;
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
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
pub struct Generator {
    pub plugin_name: String,
    pub args: Option<Value>,
    pub transform: Option<Transform>,
}

#[cfg(feature = "generate")]
impl IntoGeneratedArc for Generator {
    fn into_generated_arc(self, schema: Arc<CurrentSchema>) -> Result<Arc<GeneratedSchema>> {
        schema
            .get_plugin(&self.plugin_name)?
            .generate(schema.clone(), self.args.unwrap_or(Value::Null))
    }

    fn get_transform(&self) -> Option<Transform> {
        self.transform.clone()
    }
}
