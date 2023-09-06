#[cfg(feature = "generate")]
use crate::generate::current_schema::CurrentSchema;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::GeneratedSchema;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::{IntoGenerated, IntoGeneratedArc, IntoRandom};
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
use std::sync::Arc;

/// Flatten is a special case of Object that allows
/// you to flatten multiple objects into one.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Flatten {
    /// The values to flatten.
    /// These can be objects, references, or generators.
    /// These values must return objects, otherwise an error will be thrown.
    pub values: Vec<FlattenableValue>,
    pub transform: Option<Transform>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(untagged, deny_unknown_fields))]
pub enum FlattenableValue {
    Object(Object),
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
        }
    }

    fn get_transform(&self) -> Option<Transform> {
        None
    }
}

#[cfg(feature = "generate")]
impl IntoGenerated for Flatten {
    fn into_generated(self, schema: Arc<CurrentSchema>) -> Result<GeneratedSchema> {
        Ok(GeneratedSchema::Object(
            self.values
                .into_iter()
                .map(|value| value.into_generated_arc(schema.clone()))
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .map(|value| match value.as_ref() {
                    GeneratedSchema::Object(object) => Ok(object.clone()),
                    _ => Err("Unable to flatten non-objects".into()),
                })
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect::<IndexMap<_, _>>(),
        ))
    }

    fn get_transform(&self) -> Option<Transform> {
        self.transform.clone()
    }
}
