#[cfg(feature = "generate")]
use crate::generate::current_schema::CurrentSchema;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::IntoRandom;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::{GeneratedSchema, IntoGenerated};
use crate::schema::any_value::AnyValue;
use crate::schema::transform::Transform;
#[cfg(feature = "generate")]
use crate::util::types::Result;
#[cfg(feature = "generate")]
use rand::Rng;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "generate")]
use std::sync::Arc;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(untagged, deny_unknown_fields))]
pub enum ArrayLength {
    Constant { value: u32 },
    Random { min: u32, max: u32 },
}

#[cfg(feature = "generate")]
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

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Array {
    pub length: ArrayLength,
    pub items: AnyValue,
    pub transform: Option<Transform>,
}

#[cfg(feature = "generate")]
impl IntoGenerated for Array {
    fn into_generated(self, schema: Arc<CurrentSchema>) -> Result<GeneratedSchema> {
        let length = self.length.get_length();
        let mut res = Vec::with_capacity(length as _);
        let mut current_schema: Option<Arc<CurrentSchema>> = None;

        for i in 0..length {
            //if i > 1 && i % 5 == 0 {
            //    println!("Generated {} items", i);
            //}

            current_schema = if let Some(cur) = current_schema {
                Some(Arc::new(CurrentSchema::child(
                    schema.clone(),
                    Some(cur),
                    i.to_string(),
                )))
            } else {
                Some(Arc::new(CurrentSchema::child(
                    schema.clone(),
                    None,
                    i.to_string(),
                )))
            };

            res.push(
                self.items
                    .clone()
                    .into_random(current_schema.clone().unwrap())?,
            );
        }

        Ok(GeneratedSchema::Array(res))
    }

    fn get_transform(&self) -> Option<Transform> {
        self.transform.clone()
    }
}
