use crate::schema::transform::MaybeValidTransform;
#[cfg(feature = "schema")]
use schemars::JsonSchema;

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

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGenerated;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::schema::number::Number;
    use crate::schema::transform::MaybeValidTransform;
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

        fn get_transform(&self) -> Option<Vec<MaybeValidTransform>> {
            match self {
                Number::Constant { transform, .. } => transform.clone(),
                Number::Random { transform, .. } => transform.clone(),
            }
        }
    }
}
