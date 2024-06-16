#[cfg(feature = "serialize")]
use crate::schema::any::Any;
#[cfg(feature = "serialize")]
use crate::util::json_deserialize::from_reader;
#[cfg(feature = "serialize")]
use anyhow::Context;
#[cfg(feature = "serialize")]
use log::debug;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serialize")]
use std::fs::File;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
pub struct Include {
    pub path: String,
}

#[cfg(feature = "serialize")]
impl Include {
    pub fn as_schema(&self) -> anyhow::Result<Any> {
        debug!("Loading file at '{}'", self.path);
        let file =
            File::open(&self.path).context(format!("Could include file at '{}'", self.path))?;
        let deserialized: Any =
            from_reader(file).context(format!("Could not deserialize file at '{}'", self.path))?;

        Ok(deserialized)
    }
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGeneratedArc;
    use crate::generate::generated_schema::{GeneratedSchema, IntoRandom};
    use crate::schema::include::Include;
    use crate::schema::transform::Transform;
    use std::sync::Arc;

    impl IntoGeneratedArc for Include {
        fn into_generated_arc(
            self,
            schema: DatagenContextRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            self.as_schema()?.into_random(schema)
        }

        fn get_transform(&self) -> Option<Vec<Transform>> {
            None
        }
    }
}
