use crate::schema::any_value::AnyValue;
use crate::schema::transform::AnyTransform;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(untagged, deny_unknown_fields))]
pub enum ArrayLength {
    Constant { value: u32 },
    Random { min: u32, max: u32 },
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Array {
    pub length: ArrayLength,
    pub items: AnyValue,
    pub transform: Option<Vec<AnyTransform>>,
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::current_schema::CurrentSchemaRef;
    use crate::generate::generated_schema::generate::IntoGeneratedArc;
    use crate::generate::generated_schema::{GeneratedSchema, IntoRandom};
    use crate::generate::schema_mapper::MapSchema;
    use crate::schema::array::{Array, ArrayLength};
    use crate::schema::transform::AnyTransform;
    use crate::util::types::Result;
    use rand::Rng;
    use std::sync::Arc;

    impl ArrayLength {
        pub fn get_length(&self) -> u32 {
            match self {
                ArrayLength::Constant { value } => *value,
                ArrayLength::Random { min, max } => {
                    let mut rng = rand::thread_rng();
                    rng.gen_range(*min..=*max)
                }
            }
        }
    }

    impl IntoGeneratedArc for Array {
        fn into_generated_arc(self, schema: CurrentSchemaRef) -> Result<Arc<GeneratedSchema>> {
            let length = self.length.get_length();
            schema.map_array(length as _, self.items, None, false, |cur, value| {
                value.into_random(cur.clone())
            })
        }

        fn get_transform(&self) -> Option<Vec<AnyTransform>> {
            self.transform.clone()
        }
    }
}
