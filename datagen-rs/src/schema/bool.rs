use crate::schema::transform::AnyTransform;
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
pub mod generate {
    use crate::generate::current_schema::CurrentSchemaRef;
    use crate::generate::generated_schema::generate::IntoGenerated;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::schema::bool::Bool;
    use crate::schema::transform::AnyTransform;
    use crate::util::types::Result;
    use rand::Rng;

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
}
