use crate::schema::transform::Transform;
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
        transform: Option<Vec<Transform>>,
    },
    Constant {
        value: i64,
        transform: Option<Vec<Transform>>,
    },
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::current_schema::CurrentSchemaRef;
    use crate::generate::generated_schema::generate::IntoGenerated;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::schema::integer::Integer;
    use crate::schema::transform::Transform;
    use crate::util::types::Result;
    use rand::Rng;

    impl IntoGenerated for Integer {
        fn into_generated(self, _: CurrentSchemaRef) -> Result<GeneratedSchema> {
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

        fn get_transform(&self) -> Option<Vec<Transform>> {
            match self {
                Integer::Constant { transform, .. } => transform.clone(),
                Integer::Random { transform, .. } => transform.clone(),
            }
        }
    }
}
