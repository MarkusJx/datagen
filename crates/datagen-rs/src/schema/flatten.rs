use crate::schema::array::Array;
use crate::schema::object::Object;
use crate::schema::plugin::Plugin;
use crate::schema::reference::Reference;
use crate::schema::transform::Transform;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

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
    pub transform: Option<Vec<Transform>>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(untagged, deny_unknown_fields))]
pub enum FlattenableValue {
    Object(Object),
    Array(Array),
    Reference(Reference),
    Plugin(Plugin),
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::{IntoGenerated, IntoGeneratedArc};
    use crate::generate::generated_schema::{GeneratedSchema, IntoRandom};
    use crate::schema::flatten::{Flatten, FlattenableValue};
    use crate::schema::transform::Transform;
    use anyhow::anyhow;
    use indexmap::IndexMap;
    use std::any::{Any, TypeId};
    use std::sync::Arc;

    impl IntoGeneratedArc for FlattenableValue {
        fn into_generated_arc(
            self,
            schema: DatagenContextRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            match self {
                FlattenableValue::Object(object) => object.into_random(schema),
                FlattenableValue::Reference(reference) => reference.into_random(schema),
                FlattenableValue::Plugin(plugin) => plugin.into_random(schema),
                FlattenableValue::Array(array) => array.into_random(schema),
            }
        }

        fn get_transform(&self) -> Option<Vec<Transform>> {
            None
        }
    }

    impl IntoGenerated for Flatten {
        fn into_generated(self, schema: DatagenContextRef) -> anyhow::Result<GeneratedSchema> {
            let generated = self
                .values
                .into_iter()
                .map(|value| value.into_generated_arc(schema.clone()))
                .collect::<anyhow::Result<Vec<_>>>()?;

            let type_id = if let Some(gen) = generated.first() {
                match gen.as_ref() {
                    GeneratedSchema::Object(o) => o.type_id(),
                    GeneratedSchema::Array(a) => a.type_id(),
                    _ => return Err(anyhow!("Flatten values must be objects or arrays")),
                }
            } else {
                return Ok(GeneratedSchema::None);
            };

            if type_id == TypeId::of::<Vec<Arc<GeneratedSchema>>>() {
                Ok(GeneratedSchema::Array(
                    generated.into_iter()
                        .map(|value| match value.as_ref() {
                            GeneratedSchema::Array(array) => Ok(array.clone()),
                            _ => Err(anyhow!("Flatten values must all either be objects or arrays (the first value was an array, this one is not)")),
                        })
                        .collect::<anyhow::Result<Vec<_>>>()?
                        .into_iter()
                        .flatten()
                        .collect::<Vec<_>>(),
                ))
            } else {
                Ok(GeneratedSchema::Object(
                    generated.into_iter()
                        .map(|value| match value.as_ref() {
                            GeneratedSchema::Object(object) => Ok(object.clone()),
                            _ => Err(anyhow!("Flatten values must all either be objects or arrays (the first value was an object, this one is not)")),
                        })
                        .collect::<anyhow::Result<Vec<_>>>()?
                        .into_iter()
                        .flatten()
                        .collect::<IndexMap<_, _>>(),
                ))
            }
        }

        fn get_transform(&self) -> Option<Vec<Transform>> {
            self.transform.clone()
        }
    }
}
