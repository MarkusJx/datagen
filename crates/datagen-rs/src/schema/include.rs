#[cfg(feature = "serialize")]
use crate::schema::any::MaybeValidAny;
use crate::schema::transform::MaybeValidTransform;
#[cfg(feature = "serialize")]
use crate::util::json_deserialize::from_reader;
use crate::util::traits::GetTransform;
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
    pub fn as_schema(&self) -> anyhow::Result<MaybeValidAny> {
        debug!("Loading file at '{}'", self.path);
        let file = File::open(&self.path)
            .context(format!("Could not open file to include at '{}'", self.path))?;
        let deserialized: MaybeValidAny =
            from_reader(file).context(format!("Could not deserialize file at '{}'", self.path))?;

        Ok(deserialized)
    }
}

impl GetTransform for Include {
    fn get_transform(&self) -> Option<Vec<MaybeValidTransform>> {
        None
    }
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGeneratedArc;
    use crate::generate::generated_schema::{GeneratedSchema, IntoRandom};
    use crate::schema::include::Include;
    use std::sync::Arc;

    impl IntoGeneratedArc for Include {
        fn into_generated_arc(
            self,
            schema: DatagenContextRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            self.as_schema()?.into_random(schema)
        }
    }
}

#[cfg(feature = "validate-schema")]
pub mod validate {
    use crate::schema::include::Include;
    use crate::validation::path::ValidationPath;
    use crate::validation::result::{ValidationErrors, ValidationResult};
    use crate::validation::validate::{Validate, ValidateGenerateSchema};

    impl ValidateGenerateSchema for Include {
        fn validate_generate_schema(&self, path: &ValidationPath) -> ValidationResult {
            self.as_schema()
                .map_err(|e| {
                    ValidationErrors::single("Invalid include schema", path, Some(e), None)
                })
                .and_then(|schema| schema.validate(path))
        }
    }
}
