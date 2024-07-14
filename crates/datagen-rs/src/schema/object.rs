use crate::schema::any_value::AnyValue;
use crate::schema::transform::MaybeValidTransform;
use indexmap::IndexMap;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Object {
    pub properties: IndexMap<String, AnyValue>,
    pub transform: Option<Vec<MaybeValidTransform>>,
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGeneratedArc;
    use crate::generate::generated_schema::{GeneratedSchema, IntoRandom};
    use crate::generate::schema_mapper::MapSchema;
    use crate::schema::object::Object;
    use crate::schema::transform::MaybeValidTransform;
    use std::sync::Arc;

    impl IntoGeneratedArc for Object {
        fn into_generated_arc(
            self,
            schema: DatagenContextRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            schema.map_index_map(self.properties, None, false, |schema, value| {
                value.into_random(schema.clone())
            })
        }

        fn get_transform(&self) -> Option<Vec<MaybeValidTransform>> {
            self.transform.clone()
        }
    }
}
