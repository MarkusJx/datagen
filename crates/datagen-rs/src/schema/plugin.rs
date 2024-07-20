use crate::schema::transform::MaybeValidTransform;
use crate::util::traits::GetTransform;
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
    pub transform: Option<Vec<MaybeValidTransform>>,
}

impl GetTransform for Plugin {
    fn get_transform(&self) -> Option<Vec<MaybeValidTransform>> {
        self.transform.clone()
    }
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGeneratedArc;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::schema::plugin::Plugin;
    use std::sync::Arc;

    impl IntoGeneratedArc for Plugin {
        fn into_generated_arc(
            self,
            schema: DatagenContextRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            schema
                .get_plugin(&self.plugin_name)?
                .generate(schema, self.args.unwrap_or_default())
        }
    }
}

#[cfg(feature = "validate-schema")]
pub mod validate {
    use crate::schema::plugin::Plugin;
    use crate::validation::path::ValidationPath;
    use crate::validation::result::ValidationResult;
    use crate::validation::validate::ValidateGenerateSchema;

    impl ValidateGenerateSchema for Plugin {
        fn validate_generate_schema(&self, _path: &ValidationPath) -> ValidationResult {
            Ok(())
        }
    }
}
