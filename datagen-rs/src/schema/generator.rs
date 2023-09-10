use crate::generate::current_schema::CurrentSchemaRef;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::{GeneratedSchema, IntoGeneratedArc};
use crate::schema::transform::AnyTransform;
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
    pub transform: Option<Vec<AnyTransform>>,
}

#[cfg(feature = "generate")]
impl IntoGeneratedArc for Generator {
    fn into_generated_arc(self, schema: CurrentSchemaRef) -> Result<Arc<GeneratedSchema>> {
        schema
            .get_plugin(&self.plugin_name)?
            .generate(schema.clone(), self.args.unwrap_or_default())
    }

    fn get_transform(&self) -> Option<Vec<AnyTransform>> {
        self.transform.clone()
    }
}
