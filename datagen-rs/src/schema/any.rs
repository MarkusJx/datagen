use crate::schema::any_of::AnyOf;
use crate::schema::array::Array;
use crate::schema::bool::Bool;
use crate::schema::counter::Counter;
use crate::schema::flatten::Flatten;
use crate::schema::generator::Generator;
use crate::schema::integer::Integer;
use crate::schema::number::Number;
use crate::schema::object::Object;
use crate::schema::reference::Reference;
use crate::schema::string::StringSchema;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type", rename_all = "camelCase"))]
pub enum Any {
    Number(Number),
    Integer(Integer),
    Counter(Counter),
    Bool(Bool),
    String(StringSchema),
    AnyOf(AnyOf),
    Reference(Reference),
    Generator(Generator),
    Array(Box<Array>),
    Object(Box<Object>),
    Flatten(Flatten),
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::current_schema::CurrentSchemaRef;
    use crate::generate::generated_schema::generate::IntoGeneratedArc;
    use crate::generate::generated_schema::{GeneratedSchema, IntoRandom};
    use crate::schema::any::Any;
    use crate::schema::transform::AnyTransform;
    use crate::util::types::Result;
    use std::sync::Arc;

    impl IntoGeneratedArc for Any {
        fn into_generated_arc(self, schema: CurrentSchemaRef) -> Result<Arc<GeneratedSchema>> {
            match self {
                Any::Number(number) => number.into_random(schema),
                Any::Integer(integer) => integer.into_random(schema),
                Any::Counter(counter) => counter.into_random(schema),
                Any::Bool(bool) => bool.into_random(schema),
                Any::String(string) => string.into_random(schema),
                Any::AnyOf(any_of) => any_of.into_random(schema),
                Any::Reference(reference) => reference.into_random(schema),
                Any::Generator(generator) => generator.into_random(schema),
                Any::Array(array) => array.into_random(schema),
                Any::Object(object) => object.into_random(schema),
                Any::Flatten(flatten) => flatten.into_random(schema),
            }
        }

        fn get_transform(&self) -> Option<Vec<AnyTransform>> {
            None
        }
    }
}
