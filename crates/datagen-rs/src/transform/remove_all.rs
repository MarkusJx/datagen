#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase", untagged))]
pub enum RemoveAll {
    Include { include: Vec<String> },
    Exclude { exclude: Vec<String> },
}

#[cfg(feature = "map-schema")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::transform::remove_all::RemoveAll;
    use crate::util::traits::generate::TransformTrait;
    use std::sync::Arc;

    impl TransformTrait for RemoveAll {
        fn transform(
            self,
            _schema: DatagenContextRef,
            value: Arc<GeneratedSchema>,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            let GeneratedSchema::Object(object) = value.as_ref() else {
                anyhow::bail!("removeAll can only be applied to objects");
            };

            let mut object = object.clone();
            match self {
                RemoveAll::Include { include } => {
                    object.retain(|key, _| include.contains(key));
                }
                RemoveAll::Exclude { exclude } => {
                    object.retain(|key, _| !exclude.contains(key));
                }
            }

            Ok(GeneratedSchema::Object(object).into())
        }
    }
}

#[cfg(feature = "validate-schema")]
pub mod validate {
    use crate::transform::remove_all::RemoveAll;
    use crate::validation::path::ValidationPath;
    use crate::validation::result::{IterValidate, ValidationResult};
    use crate::validation::validate::Validate;

    impl Validate for RemoveAll {
        fn validate(&self, path: &ValidationPath) -> ValidationResult {
            match self {
                RemoveAll::Include { include } => {
                    ValidationResult::ensure(!include.is_empty(), "include must not be empty", path)
                }
                RemoveAll::Exclude { exclude } => {
                    ValidationResult::ensure(!exclude.is_empty(), "exclude must not be empty", path)
                }
            }
        }
    }
}
