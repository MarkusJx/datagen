use crate::schema::transform::Transform;
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
        transform: Option<Vec<Transform>>,
    },
    Constant {
        value: bool,
        transform: Option<Vec<Transform>>,
    },
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGenerated;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::schema::bool::Bool;
    use crate::schema::transform::Transform;
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

        fn get_transform(&self) -> Option<Vec<Transform>> {
            match self {
                Bool::Constant { transform, .. } => transform.clone(),
                Bool::Random { transform, .. } => transform.clone(),
            }
        }
    }
}
