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
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum Bool {
    Random {
        probability: Option<f64>,
        transform: Option<Vec<AnyTransform>>,
    },
    Constant {
        value: bool,
        transform: Option<Vec<AnyTransform>>,
    },
}

#[cfg(feature = "generate")]
impl IntoGenerated for Bool {
    fn into_generated(self, _: CurrentSchemaRef) -> Result<GeneratedSchema> {
        Ok(match self {
            Bool::Constant { value, .. } => GeneratedSchema::Bool(value),
            Bool::Random { probability, .. } => {
                let mut rng = rand::thread_rng();
                let value = rng.gen_bool(probability.unwrap_or(0.5));
                GeneratedSchema::Bool(value)
            }
        })
    }

    fn get_transform(&self) -> Option<Vec<AnyTransform>> {
        match self {
            Bool::Constant { transform, .. } => transform.clone(),
            Bool::Random { transform, .. } => transform.clone(),
        }
    }
}
