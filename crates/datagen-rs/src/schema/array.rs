use crate::schema::any_value::AnyValue;
use crate::schema::transform::Transform;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(untagged, deny_unknown_fields))]
pub enum ArrayLength {
    ShortConstant(u32),
    Constant { value: u32 },
    Random { min: u32, max: u32 },
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(untagged, deny_unknown_fields))]
pub enum Array {
    RandomArray(RandomArray),
    ArrayWithValues(ArrayWithValues),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct RandomArray {
    pub length: ArrayLength,
    pub items: AnyValue,
    pub transform: Option<Vec<Transform>>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct ArrayWithValues {
    pub values: Vec<AnyValue>,
    pub transform: Option<Vec<Transform>>,
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGeneratedArc;
    use crate::generate::generated_schema::{GeneratedSchema, IntoRandom};
    use crate::generate::schema_mapper::MapSchema;
    use crate::schema::array::{Array, ArrayLength};
    use crate::schema::transform::Transform;
    use rand::Rng;
    use std::sync::Arc;

    use super::{ArrayWithValues, RandomArray};

    impl ArrayLength {
        pub fn get_length(&self) -> u32 {
            match self {
                ArrayLength::ShortConstant(value) => *value,
                ArrayLength::Constant { value } => *value,
                ArrayLength::Random { min, max } => {
                    let mut rng = rand::thread_rng();
                    rng.gen_range(*min..=*max)
                }
            }
        }
    }

    impl IntoGeneratedArc for Array {
        fn into_generated_arc(
            self,
            schema: DatagenContextRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            match self {
                Array::RandomArray(random_array) => random_array.into_generated_arc(schema),
                Array::ArrayWithValues(array_with_values) => {
                    array_with_values.into_generated_arc(schema)
                }
            }
        }

        fn get_transform(&self) -> Option<Vec<Transform>> {
            match self {
                Array::RandomArray(random_array) => random_array.get_transform(),
                Array::ArrayWithValues(array_with_values) => array_with_values.get_transform(),
            }
        }
    }

    impl IntoGeneratedArc for RandomArray {
        fn into_generated_arc(
            self,
            schema: DatagenContextRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            let length = self.length.get_length();
            schema.map_array(length as _, self.items, None, false, |cur, value| {
                value.into_random(cur.clone())
            })
        }

        fn get_transform(&self) -> Option<Vec<Transform>> {
            self.transform.clone()
        }
    }

    impl IntoGeneratedArc for ArrayWithValues {
        fn into_generated_arc(
            self,
            schema: DatagenContextRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            Ok(GeneratedSchema::Array(
                self.values
                    .into_iter()
                    .map(|e| e.into_generated_arc(schema.clone()))
                    .collect::<anyhow::Result<Vec<_>>>()?,
            )
            .into())
        }

        fn get_transform(&self) -> Option<Vec<Transform>> {
            self.transform.clone()
        }
    }
}
