#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
pub struct PluginTransform {
    /// The path of the plugin which will be used to transform the data
    pub name: String,
    /// The arguments which will be passed to the plugin
    pub args: Option<Value>,
}

#[cfg(feature = "map-schema")]
pub mod generate {
    use crate::generate::current_schema::CurrentSchemaRef;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::transform::plugin_transform::PluginTransform;
    use crate::util::traits::generate::TransformTrait;
    use std::sync::Arc;

    impl TransformTrait for PluginTransform {
        fn transform(
            self,
            schema: CurrentSchemaRef,
            value: Arc<GeneratedSchema>,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            schema.get_plugin(&self.name)?.transform(
                schema.clone(),
                value,
                self.args.unwrap_or_default(),
            )
        }
    }
}
