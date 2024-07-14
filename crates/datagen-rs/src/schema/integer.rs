use crate::schema::transform::MaybeValidTransform;
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

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGenerated;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::schema::integer::Integer;
    use crate::schema::transform::MaybeValidTransform;
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

        fn get_transform(&self) -> Option<Vec<MaybeValidTransform>> {
            match self {
                Integer::Constant { transform, .. } => transform.clone(),
                Integer::Random { transform, .. } => transform.clone(),
            }
        }
    }
}
