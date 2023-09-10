use crate::generate::current_schema::CurrentSchemaRef;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::{GeneratedSchema, IntoGenerated};
use crate::schema::transform::AnyTransform;
#[cfg(feature = "generate")]
use crate::util::types::Result;
#[cfg(feature = "generate")]
use rand::Rng;
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
        min: Option<i32>,
        #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
        max: Option<i32>,
        transform: Option<Vec<AnyTransform>>,
    },
    Constant {
        value: i32,
        transform: Option<Vec<AnyTransform>>,
    },
}

#[cfg(feature = "generate")]
impl IntoGenerated for Integer {
    fn into_generated(self, _: CurrentSchemaRef) -> Result<GeneratedSchema> {
        Ok(match self {
            Integer::Constant { value, .. } => GeneratedSchema::Integer(value),
            Integer::Random { min, max, .. } => {
                let mut rng = rand::thread_rng();
                let min = min.unwrap_or(i32::MIN);
                let max = max.unwrap_or(i32::MAX);
                let value = rng.gen_range(min..=max);
                GeneratedSchema::Integer(value)
            }
        })
    }

    fn get_transform(&self) -> Option<Vec<AnyTransform>> {
        match self {
            Integer::Constant { transform, .. } => transform.clone(),
            Integer::Random { transform, .. } => transform.clone(),
        }
    }
}
