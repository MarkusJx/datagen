#[cfg(feature = "generate")]
use crate::generate::current_schema::CurrentSchema;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::GeneratedSchema;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::IntoGeneratedArc;
use crate::schema::reference::Reference;
use crate::schema::transform::Transform;
#[cfg(feature = "generate")]
use crate::util::types::Result;
#[cfg(feature = "generate")]
use rand::Rng;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "generate")]
use std::sync::Arc;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(untagged, deny_unknown_fields))]
pub enum AnyOfValue {
    String(String),
    Reference(Reference),
    Value(Value),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct AnyOf {
    pub values: Vec<AnyOfValue>,
    pub num: Option<i32>,
    pub transform: Option<Transform>,
}

#[cfg(feature = "generate")]
impl IntoGeneratedArc for AnyOf {
    fn into_generated_arc(self, schema: Arc<CurrentSchema>) -> Result<Arc<GeneratedSchema>> {
        let mut values = Vec::with_capacity(self.values.len());

        for value in self.values {
            match value {
                AnyOfValue::String(string) => {
                    values.push(schema.resolve_ref(string)?.into_random()?);
                }
                AnyOfValue::Reference(reference) => {
                    values.push(reference.into_generated_arc(schema.clone())?);
                }
                AnyOfValue::Value(value) => {
                    values.push(Arc::new(GeneratedSchema::Value(value)));
                }
            }
        }

        if values.is_empty() {
            Ok(Arc::new(GeneratedSchema::None))
        } else {
            let diff = (values.len() as i32) - self.num.unwrap_or(1);
            if diff > 0 {
                let mut rng = rand::thread_rng();
                for _ in 0..diff {
                    values.remove(rng.gen_range(0..values.len()));
                }
            }

            if values.is_empty() {
                Ok(Arc::new(GeneratedSchema::None))
            } else if values.len() == 1 {
                Ok(values[0].clone())
            } else {
                Ok(Arc::new(GeneratedSchema::Array(values)))
            }
        }
    }

    fn get_transform(&self) -> Option<Transform> {
        self.transform.clone()
    }
}
