use crate::schema::transform::MaybeValidTransform;
use crate::util::traits::GetTransform;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(untagged, deny_unknown_fields))]
pub enum Integer {
    Random {
        #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
        min: Option<i64>,
        #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
        max: Option<i64>,
        transform: Option<Vec<MaybeValidTransform>>,
    },
    Constant {
        value: i64,
        transform: Option<Vec<MaybeValidTransform>>,
    },
}

impl GetTransform for Integer {
    fn get_transform(&self) -> Option<Vec<MaybeValidTransform>> {
        match self {
            Integer::Constant { transform, .. } => transform.clone(),
            Integer::Random { transform, .. } => transform.clone(),
        }
    }
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGenerated;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::schema::integer::Integer;
    use rand::Rng;

    impl IntoGenerated for Integer {
        fn into_generated(self, _: DatagenContextRef) -> anyhow::Result<GeneratedSchema> {
            Ok(match self {
                Integer::Constant { value, .. } => GeneratedSchema::Integer(value),
                Integer::Random { min, max, .. } => {
                    let mut rng = rand::thread_rng();
                    let min = min.unwrap_or(i64::MIN);
                    let max = max.unwrap_or(i64::MAX);
                    let value = rng.gen_range(min..=max);
                    GeneratedSchema::Integer(value)
                }
            })
        }
    }
}

#[cfg(feature = "validate-schema")]
pub mod validate {
    use crate::schema::integer::Integer;
    use crate::validation::path::ValidationPath;
    use crate::validation::result::{IterValidate, ValidationResult};
    use crate::validation::validate::ValidateGenerateSchema;

    impl ValidateGenerateSchema for Integer {
        fn validate_generate_schema(&self, path: &ValidationPath) -> ValidationResult {
            match self {
                Integer::Constant { value, .. } => {
                    return ValidationResult::ensure(
                        *value <= i64::MAX && *value >= i64::MIN,
                        "Integer value out of range",
                        path,
                    );
                }
                Integer::Random { min, max, .. } => {
                    if let Some(min) = min {
                        return ValidationResult::ensure(
                            *min <= i64::MAX,
                            "Integer min value out of range",
                            path,
                        );
                    }
                    if let Some(max) = max {
                        return ValidationResult::ensure(
                            *max <= i64::MAX,
                            "Integer min value greater than max value",
                            path,
                        );
                    }

                    if let (Some(min), Some(max)) = (min, max) {
                        return ValidationResult::ensure(
                            min <= max,
                            "Integer min value greater than max value",
                            path,
                        );
                    }
                }
            }

            ValidationResult::valid()
        }
    }
}
