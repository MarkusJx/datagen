#[cfg(feature = "generate")]
use crate::generate::current_schema::CurrentSchema;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::IntoRandom;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::{GeneratedSchema, IntoGeneratedArc};
#[cfg(feature = "generate")]
use crate::generate::schema_mapper::MapSchema;
use crate::schema::any_value::AnyValue;
use crate::schema::transform::Transform;
#[cfg(feature = "generate")]
use crate::util::types::Result;
use indexmap::IndexMap;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "generate")]
use std::sync::Arc;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Object {
    pub properties: IndexMap<String, AnyValue>,
    pub transform: Option<Transform>,
}

#[cfg(feature = "generate")]
impl IntoGeneratedArc for Object {
    fn into_generated_arc(self, schema: Arc<CurrentSchema>) -> Result<Arc<GeneratedSchema>> {
        schema.map_index_map(self.properties, false, |schema, value| {
            value.into_random(schema.clone())
        })
    }

    fn get_transform(&self) -> Option<Transform> {
        self.transform.clone()
    }
}
