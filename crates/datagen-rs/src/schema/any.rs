use crate::schema::any_of::AnyOf;
use crate::schema::array::Array;
use crate::schema::bool::Bool;
use crate::schema::counter::Counter;
use crate::schema::file::File;
use crate::schema::flatten::Flatten;
use crate::schema::include::Include;
use crate::schema::integer::Integer;
use crate::schema::number::Number;
use crate::schema::object::Object;
use crate::schema::plugin::Plugin;
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
    Plugin(Plugin),
    Array(Box<Array>),
    Object(Box<Object>),
    Flatten(Flatten),
    File(File),
    Include(Include),
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGeneratedArc;
    use crate::generate::generated_schema::{GeneratedSchema, IntoRandom};
    use crate::schema::any::Any;
    use crate::schema::transform::Transform;
    use std::sync::Arc;

    impl IntoGeneratedArc for Any {
        fn into_generated_arc(
            self,
            schema: DatagenContextRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            match self {
                Any::Number(number) => number.into_random(schema),
                Any::Integer(integer) => integer.into_random(schema),
                Any::Counter(counter) => counter.into_random(schema),
                Any::Bool(bool) => bool.into_random(schema),
                Any::String(string) => string.into_random(schema),
                Any::AnyOf(any_of) => any_of.into_random(schema),
                Any::Reference(reference) => reference.into_random(schema),
                Any::Plugin(plugin) => plugin.into_random(schema),
                Any::Array(array) => array.into_random(schema),
                Any::Object(object) => object.into_random(schema),
                Any::Flatten(flatten) => flatten.into_random(schema),
                Any::File(file) => file.into_random(schema),
                Any::Include(include) => include.into_random(schema),
            }
        }

        fn get_transform(&self) -> Option<Vec<Transform>> {
            None
        }
    }
}
