#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
pub struct ChooseFromArray {
    /// The index or indices to choose from.
    /// Returns one random element from the array if unset.
    pub indices: Option<Vec<usize>>,
}

#[cfg(feature = "map-schema")]
pub mod generate {
    use std::sync::Arc;

    use rand::thread_rng;

    use crate::{
        generate::{datagen_context::DatagenContextRef, generated_schema::GeneratedSchema},
        util::traits::generate::TransformTrait,
    };
    use rand::prelude::SliceRandom;

    use super::ChooseFromArray;

    impl TransformTrait for ChooseFromArray {
        fn transform(
            self,
            schema: DatagenContextRef,
            value: Arc<GeneratedSchema>,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            match value.as_ref() {
                GeneratedSchema::Array(array) => match self.indices {
                    Some(indices) => {
                        if array.is_empty() {
                            return Ok(GeneratedSchema::None.into());
                        }

                        let mut vec = indices
                            .into_iter()
                            .map(|i| {
                                array.get(i).map(Clone::clone).ok_or_else(|| {
                                    anyhow::anyhow!(
                                        "Could not get index {i} from array at {}",
                                        schema
                                            .path()
                                            .map(|p| p.to_string())
                                            .unwrap_or("UNKNOWN".to_string())
                                    )
                                })
                            })
                            .collect::<anyhow::Result<Vec<_>>>()?;

                        if vec.len() == 1 {
                            Ok(vec.pop().unwrap())
                        } else {
                            Ok(GeneratedSchema::Array(vec).into())
                        }
                    }
                    None => {
                        let mut rng = thread_rng();
                        Ok(array
                            .choose(&mut rng)
                            .map(Clone::clone)
                            .unwrap_or_else(|| GeneratedSchema::None.into()))
                    }
                },
                _ => Ok(value),
            }
        }
    }
}
