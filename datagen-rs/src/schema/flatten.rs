#[cfg(feature = "generate")]
use crate::generate::current_schema::CurrentSchema;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::GeneratedSchema;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::{IntoGenerated, IntoGeneratedArc, IntoRandom};
use crate::schema::array::Array;
use crate::schema::generator::Generator;
use crate::schema::object::Object;
use crate::schema::reference::Reference;
use crate::schema::transform::Transform;
#[cfg(feature = "generate")]
use crate::util::types::Result;
#[cfg(feature = "generate")]
use indexmap::IndexMap;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "generate")]
use std::any::{Any, TypeId};
#[cfg(feature = "generate")]
use std::sync::Arc;

/// Flatten is a special case of Object that allows
/// you to flatten multiple objects or arrays into one.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Flatten {
    /// The values to flatten.
    /// These can be objects, references, or generators.
    /// These values must all return either objects or arrays,
    /// not both, otherwise an error will be thrown.
    /// If no values are provided, null will be returned.
    pub values: Vec<FlattenableValue>,
    pub transform: Option<Transform>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(untagged, deny_unknown_fields))]
pub enum FlattenableValue {
    Object(Object),
    Array(Array),
    Reference(Reference),
    Generator(Generator),
}

#[cfg(feature = "generate")]
impl IntoGeneratedArc for FlattenableValue {
    fn into_generated_arc(self, schema: Arc<CurrentSchema>) -> Result<Arc<GeneratedSchema>> {
        match self {
            FlattenableValue::Object(object) => object.into_random(schema),
            FlattenableValue::Reference(reference) => reference.into_random(schema),
            FlattenableValue::Generator(generator) => generator.into_random(schema),
            FlattenableValue::Array(array) => array.into_random(schema),
        }
    }

    fn get_transform(&self) -> Option<Transform> {
        None
    }
}

#[cfg(feature = "generate")]
impl IntoGenerated for Flatten {
    fn into_generated(self, schema: Arc<CurrentSchema>) -> Result<GeneratedSchema> {
        let generated = self
            .values
            .into_iter()
            .map(|value| value.into_generated_arc(schema.clone()))
            .collect::<Result<Vec<_>>>()?;

        let type_id = if let Some(gen) = generated.first() {
            match gen.as_ref() {
                GeneratedSchema::Object(o) => o.type_id(),
                GeneratedSchema::Array(a) => a.type_id(),
                _ => return Err("Flatten values must be objects or arrays".into()),
            }
        } else {
            return Ok(GeneratedSchema::None);
        };

        if type_id == TypeId::of::<Vec<Arc<GeneratedSchema>>>() {
            Ok(GeneratedSchema::Array(
                generated.into_iter()
                    .map(|value| match value.as_ref() {
                        GeneratedSchema::Array(array) => Ok(array.clone()),
                        _ => Err("Flatten values must all either be objects or arrays (the first value was an array, this one is not)".into()),
                    })
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>(),
            ))
        } else {
            Ok(GeneratedSchema::Object(
                generated.into_iter()
                    .map(|value| match value.as_ref() {
                        GeneratedSchema::Object(object) => Ok(object.clone()),
                        _ => Err("Flatten values must all either be objects or arrays (the first value was an object, this one is not)".into()),
                    })
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .flatten()
                    .collect::<IndexMap<_, _>>(),
            ))
        }
    }

    fn get_transform(&self) -> Option<Transform> {
        self.transform.clone()
    }
}
