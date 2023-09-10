use crate::generate::current_schema::CurrentSchemaRef;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::GeneratedSchema;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::IntoGeneratedArc;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::IntoRandom;
use crate::schema::any_value::AnyValue;
use crate::schema::transform::AnyTransform;
#[cfg(feature = "generate")]
use crate::util::types::Result;
#[cfg(feature = "generate")]
use rand::seq::SliceRandom;
#[cfg(feature = "generate")]
use rand::Rng;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
#[cfg(feature = "generate")]
use std::sync::Arc;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct AnyOf {
    pub values: Vec<AnyValue>,
    pub num: Option<i64>,
    pub transform: Option<Vec<AnyTransform>>,
}

#[cfg(feature = "generate")]
impl IntoGeneratedArc for AnyOf {
    fn into_generated_arc(mut self, schema: CurrentSchemaRef) -> Result<Arc<GeneratedSchema>> {
        self.values.shuffle(&mut rand::thread_rng());
        let mut num = self.num.unwrap_or(1);
        match num.cmp(&0) {
            Ordering::Equal => num = self.values.len() as i64,
            Ordering::Less => num = rand::thread_rng().gen_range(0..self.values.len() as i64),
            _ => {}
        }

        let values = self
            .values
            .drain(0..num as usize)
            .map(|value| value.into_random(schema.clone()))
            .collect::<Result<Vec<_>>>()?;

        if values.is_empty() {
            Ok(Arc::new(GeneratedSchema::None))
        } else if values.len() == 1 {
            Ok(values[0].clone())
        } else {
            Ok(Arc::new(GeneratedSchema::Array(values)))
        }
    }

    fn get_transform(&self) -> Option<Vec<AnyTransform>> {
        self.transform.clone()
    }
}
