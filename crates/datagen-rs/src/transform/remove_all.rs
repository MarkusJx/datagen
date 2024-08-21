#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase", untagged))]
pub enum RemoveAllTransform {
    Include { include: Vec<String> },
    Exclude { exclude: Vec<String> },
}

#[cfg(feature = "map-schema")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::transform::remove_all::RemoveAllTransform;
    use crate::util::traits::generate::TransformTrait;
    use std::sync::Arc;

    impl TransformTrait for RemoveAllTransform {
        fn transform(
            self,
            schema: DatagenContextRef,
            value: Arc<GeneratedSchema>,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            match value.as_ref() {
                GeneratedSchema::Object(object) => {
                    let mut object = object.clone();
                    match self {
                        RemoveAllTransform::Include { include } => {
                            object.retain(|key, _| !include.contains(key));
                        }
                        RemoveAllTransform::Exclude { exclude } => {
                            object.retain(|key, _| exclude.contains(key));
                        }
                    }

                    Ok(GeneratedSchema::Object(object).into())
                }
                GeneratedSchema::None => Ok(GeneratedSchema::None.into()),
                invalid => Err(anyhow::anyhow!(
                    "removeAll can only be applied to objects. Actual type was {}",
                    invalid.name()
                )
                .context(anyhow::anyhow!("Invalid schema at {}", schema.path()?))),
            }
        }
    }
}

#[cfg(feature = "validate-schema")]
pub mod validate {
    use crate::transform::remove_all::RemoveAllTransform;
    use crate::validation::path::ValidationPath;
    use crate::validation::result::{IterValidate, ValidationResult};
    use crate::validation::validate::Validate;

    impl Validate for RemoveAllTransform {
        fn validate(&self, path: &ValidationPath) -> ValidationResult {
            match self {
                RemoveAllTransform::Include { include } => {
                    ValidationResult::ensure(!include.is_empty(), "include must not be empty", path)
                }
                RemoveAllTransform::Exclude { exclude } => {
                    ValidationResult::ensure(!exclude.is_empty(), "exclude must not be empty", path)
                }
            }
        }
    }
}
