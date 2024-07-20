use crate::schema::transform::MaybeValidTransform;
use crate::util::traits::GetTransform;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
pub struct Reference {
    pub reference: String,
    pub except: Option<Vec<StringOrNumber>>,
    pub keep_all: Option<bool>,
    pub transform: Option<Vec<MaybeValidTransform>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(untagged, deny_unknown_fields))]
pub enum StringOrNumber {
    String(String),
    Number(f64),
}

impl GetTransform for Reference {
    fn get_transform(&self) -> Option<Vec<MaybeValidTransform>> {
        self.transform.clone()
    }
}

#[cfg(feature = "map-schema")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGeneratedArc;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::schema::reference::{Reference, StringOrNumber};
    use rand::prelude::SliceRandom;
    use std::sync::Arc;

    impl IntoGeneratedArc for Reference {
        fn into_generated_arc(
            self,
            schema: DatagenContextRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            let mut reference = self.reference;
            if !reference.starts_with("ref:") {
                reference = format!("ref:{reference}");
            }

            let resolved = schema.resolve_ref(&reference)?;
            let Some(except) = self.except else {
                return resolved.into_random();
            };

            let Some(resolved) = resolved.into_vec() else {
                return Ok(Arc::new(GeneratedSchema::None));
            };

            let except = except
                .into_iter()
                .map(|x| match x {
                    StringOrNumber::String(string) => {
                        Ok(schema.resolve_ref(&string)?.into_vec().unwrap_or(vec![]))
                    }
                    StringOrNumber::Number(number) => {
                        Ok(vec![Arc::new(GeneratedSchema::Number(number.into()))])
                    }
                })
                .collect::<anyhow::Result<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

            let resolved = resolved
                .iter()
                .filter(|x| !except.contains(x))
                .cloned()
                .collect::<Vec<_>>();

            Ok(if self.keep_all.unwrap_or(false) {
                Arc::new(GeneratedSchema::Array(resolved))
            } else if let Some(resolved) = resolved.choose(&mut rand::thread_rng()) {
                resolved.clone()
            } else {
                Arc::new(GeneratedSchema::None)
            })
        }
    }
}

#[cfg(feature = "validate-schema")]
pub mod validate {
    use crate::schema::reference::Reference;
    use crate::validation::path::ValidationPath;
    use crate::validation::result::{IterValidate, ValidationResult};
    use crate::validation::validate::ValidateGenerateSchema;

    impl ValidateGenerateSchema for Reference {
        fn validate_generate_schema(&self, path: &ValidationPath) -> ValidationResult {
            ValidationResult::ensure(
                !self.reference.is_empty(),
                "reference must not be empty",
                path,
            )
        }
    }
}
