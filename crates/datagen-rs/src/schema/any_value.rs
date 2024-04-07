use crate::schema::any::Any;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

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
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGeneratedArc;
    use crate::generate::generated_schema::{GeneratedSchema, IntoRandom};
    use crate::schema::any_value::AnyValue;
    use crate::schema::transform::Transform;
    use std::sync::Arc;

    impl IntoGeneratedArc for AnyValue {
        fn into_generated_arc(
            self,
            schema: DatagenContextRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            match self {
                AnyValue::Any(any) => any.into_random(schema),
                AnyValue::String(string) => schema.resolve_ref(&string)?.into_random(),
                AnyValue::Number(number) => {
                    schema.finalize(GeneratedSchema::Number(number.into()).into())
                }
                AnyValue::Bool(bool) => schema.finalize(GeneratedSchema::Bool(bool).into()),
                AnyValue::Null => schema.finalize(GeneratedSchema::None.into()),
            }
        }

        fn get_transform(&self) -> Option<Vec<Transform>> {
            None
        }

        fn should_finalize(&self) -> bool {
            !matches!(self, AnyValue::Any(..))
        }
    }
}
