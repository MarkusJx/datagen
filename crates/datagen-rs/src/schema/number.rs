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
pub enum Number {
    Random {
        #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
        min: Option<f64>,
        #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
        max: Option<f64>,
        #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
        precision: Option<u8>,
        transform: Option<Vec<MaybeValidTransform>>,
    },
    Constant {
        value: f64,
        transform: Option<Vec<MaybeValidTransform>>,
    },
}

impl GetTransform for Number {
    fn get_transform(&self) -> Option<Vec<MaybeValidTransform>> {
        match self {
            Number::Constant { transform, .. } => transform.clone(),
            Number::Random { transform, .. } => transform.clone(),
        }
    }
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGenerated;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::schema::number::Number;
    use rand::Rng;

    impl IntoGenerated for Number {
        fn into_generated(self, _: DatagenContextRef) -> anyhow::Result<GeneratedSchema> {
            Ok(match self {
                Number::Constant { value, .. } => GeneratedSchema::Number(value.into()),
                Number::Random {
                    min,
                    max,
                    precision,
                    ..
                } => {
                    let mut rng = rand::thread_rng();
                    let min = min.unwrap_or(0_f64);
                    let max = max.unwrap_or(1_f64);
                    let mut value = rng.gen_range(min..max);
                    if let Some(precision) = precision {
                        value = (value * 10.0_f64.powi(precision as i32)).round()
                            / 10.0_f64.powi(precision as i32);
                    }
                    GeneratedSchema::Number(value.into())
                }
            })
        }
    }
}

#[cfg(feature = "validate-schema")]
pub mod validate {
    use crate::schema::number::Number;
    use crate::validation::path::ValidationPath;
    use crate::validation::result::{IterValidate, ValidationResult};
    use crate::validation::validate::ValidateGenerateSchema;

    impl ValidateGenerateSchema for Number {
        fn validate_generate_schema(&self, path: &ValidationPath) -> ValidationResult {
            match self {
                Number::Constant { value, .. } => {
                    return ValidationResult::ensure(
                        value.is_finite(),
                        "Number::Constant value must be finite",
                        path,
                    );
                }
                Number::Random { min, max, .. } => {
                    if let Some(min) = min {
                        if min.is_nan() {
                            return ValidationResult::single(
                                "Number::Random min must not be NaN",
                                path,
                                None,
                                None,
                            );
                        }
                    }
                    if let Some(max) = max {
                        if max.is_nan() {
                            return ValidationResult::single(
                                "Number::Random max must not be NaN",
                                path,
                                None,
                                None,
                            );
                        }
                    }

                    if let (Some(min), Some(max)) = (min, max) {
                        if min > max {
                            return ValidationResult::single(
                                "Number::Random min must be less than or equal to max",
                                &path,
                                None,
                                None,
                            );
                        }
                    }
                }
            }

            Ok(())
        }
    }
}
