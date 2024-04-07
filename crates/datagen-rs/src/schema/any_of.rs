use crate::schema::any_value::AnyValue;
use crate::schema::transform::Transform;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct AnyOf {
    pub values: Vec<AnyValue>,
    pub num: Option<i64>,
    pub allow_null: Option<bool>,
    pub transform: Option<Vec<Transform>>,
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGeneratedArc;
    use crate::generate::generated_schema::{GeneratedSchema, IntoRandom};
    use crate::schema::any_of::AnyOf;
    use crate::schema::transform::Transform;
    use rand::seq::SliceRandom;
    use rand::Rng;
    use std::cmp::Ordering;
    use std::sync::Arc;

    impl IntoGeneratedArc for AnyOf {
        fn into_generated_arc(
            mut self,
            schema: DatagenContextRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            self.values.shuffle(&mut rand::thread_rng());
            let min = if self.allow_null.unwrap_or(false) {
                0
            } else {
                1
            };

            let mut num = self.num.unwrap_or(1);
            match num.cmp(&0) {
                Ordering::Equal => num = self.values.len() as i64,
                Ordering::Less => {
                    num = rand::thread_rng().gen_range(min..=self.values.len() as i64)
                }
                _ => {}
            }

            let values = self
                .values
                .drain(0..num as usize)
                .map(|value| value.into_random(schema.clone()))
                .collect::<anyhow::Result<Vec<_>>>()?;

            if values.is_empty() {
                Ok(Arc::new(GeneratedSchema::None))
            } else if values.len() == 1 {
                Ok(values[0].clone())
            } else {
                Ok(Arc::new(GeneratedSchema::Array(values)))
            }
        }

        fn get_transform(&self) -> Option<Vec<Transform>> {
            self.transform.clone()
        }
    }
}
