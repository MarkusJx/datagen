use crate::schema::transform::Transform;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
pub struct Plugin {
    pub plugin_name: String,
    pub args: Option<Value>,
    pub transform: Option<Vec<Transform>>,
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::current_schema::CurrentSchemaRef;
    use crate::generate::generated_schema::generate::IntoGeneratedArc;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::schema::plugin::Plugin;
    use crate::schema::transform::Transform;
    use std::sync::Arc;

    impl IntoGeneratedArc for Plugin {
        fn into_generated_arc(
            self,
            schema: CurrentSchemaRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            schema
                .get_plugin(&self.plugin_name)?
                .generate(schema.clone(), self.args.unwrap_or_default())
        }

        fn get_transform(&self) -> Option<Vec<Transform>> {
            self.transform.clone()
        }
    }
}
