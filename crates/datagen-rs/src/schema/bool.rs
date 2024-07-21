use crate::schema::transform::MaybeValidTransform;
use crate::util::traits::GetTransform;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum Bool {
    Random {
        probability: Option<f64>,
        transform: Option<Vec<MaybeValidTransform>>,
    },
    Constant {
        value: bool,
        transform: Option<Vec<MaybeValidTransform>>,
    },
}

impl GetTransform for Bool {
    fn get_transform(&self) -> Option<Vec<MaybeValidTransform>> {
        match self {
            Bool::Constant { transform, .. } => transform.clone(),
            Bool::Random { transform, .. } => transform.clone(),
        }
    }
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGenerated;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::schema::bool::Bool;
    use rand::Rng;

    impl IntoGenerated for Bool {
        fn into_generated(self, _: DatagenContextRef) -> anyhow::Result<GeneratedSchema> {
            Ok(match self {
                Bool::Constant { value, .. } => GeneratedSchema::Bool(value),
                Bool::Random { probability, .. } => {
                    let mut rng = rand::thread_rng();
                    let value = rng.gen_bool(probability.unwrap_or(0.5));
                    GeneratedSchema::Bool(value)
                }
            })
        }
    }
}

#[cfg(feature = "validate-schema")]
pub mod validate {
    use crate::schema::bool::Bool;
    use crate::validation::path::ValidationPath;
    use crate::validation::result::{IterValidate, ValidationResult};
    use crate::validation::validate::ValidateGenerateSchema;

    impl ValidateGenerateSchema for Bool {
        fn validate_generate_schema(&self, path: &ValidationPath) -> ValidationResult {
            match self {
                Bool::Constant { .. } => Ok(()),
                Bool::Random { probability, .. } => {
                    if let Some(probability) = probability {
                        if *probability < 0.0 || *probability > 1.0 {
                            return ValidationResult::single(
                                format!(
                                    "Probability must be between 0.0 and 1.0, got {}",
                                    probability
                                ),
                                path,
                                None,
                                None,
                            );
                        }
                    }

                    Ok(())
                }
            }
        }
    }
}
