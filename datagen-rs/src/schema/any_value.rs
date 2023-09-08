#[cfg(feature = "generate")]
use crate::generate::current_schema::CurrentSchema;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::GeneratedSchema;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::{IntoGeneratedArc, IntoRandom};
use crate::schema::any::Any;
#[cfg(feature = "generate")]
use crate::schema::transform::Transform;
#[cfg(feature = "generate")]
use crate::util::types::Result;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "generate")]
use std::sync::Arc;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum AnyValue {
    Any(Any),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

#[cfg(feature = "generate")]
impl IntoGeneratedArc for AnyValue {
    fn into_generated_arc(self, schema: Arc<CurrentSchema>) -> Result<Arc<GeneratedSchema>> {
        match self {
            AnyValue::Any(any) => any.into_random(schema),
            AnyValue::String(string) => schema.resolve_ref(string)?.into_random(),
            AnyValue::Number(number) => Ok(schema.finalize(GeneratedSchema::Number(number).into())),
            AnyValue::Bool(bool) => Ok(schema.finalize(GeneratedSchema::Bool(bool).into())),
            AnyValue::Null => Ok(schema.finalize(GeneratedSchema::None.into())),
        }
    }

    fn get_transform(&self) -> Option<Transform> {
        None
    }

    fn should_finalize(&self) -> bool {
        !matches!(self, AnyValue::Any(..))
    }
}
