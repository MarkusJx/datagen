#[cfg(feature = "generate")]
use crate::generate::current_schema::CurrentSchema;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::GeneratedSchema;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::IntoGeneratedArc;
use crate::generate::generated_schema::IntoRandom;
use crate::schema::any_value::AnyValue;
use crate::schema::transform::Transform;
#[cfg(feature = "generate")]
use crate::util::types::Result;
use rand::seq::SliceRandom;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "generate")]
use std::sync::Arc;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct AnyOf {
    pub values: Vec<AnyValue>,
    pub num: Option<i32>,
    pub transform: Option<Transform>,
}

#[cfg(feature = "generate")]
impl IntoGeneratedArc for AnyOf {
    fn into_generated_arc(mut self, schema: Arc<CurrentSchema>) -> Result<Arc<GeneratedSchema>> {
        self.values.shuffle(&mut rand::thread_rng());
        let values = self
            .values
            .drain(0..self.num.unwrap_or(1) as usize)
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

    fn get_transform(&self) -> Option<Transform> {
        self.transform.clone()
    }
}
