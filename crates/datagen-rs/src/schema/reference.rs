use crate::schema::transform::Transform;
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
    pub transform: Option<Vec<Transform>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(untagged, deny_unknown_fields))]
pub enum StringOrNumber {
    String(String),
    Number(f64),
}

#[cfg(feature = "map-schema")]
pub mod generate {
    use crate::generate::current_schema::CurrentSchemaRef;
    use crate::generate::generated_schema::generate::IntoGeneratedArc;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::schema::reference::{Reference, StringOrNumber};
    use crate::schema::transform::Transform;
    use rand::prelude::SliceRandom;
    use std::sync::Arc;

    impl IntoGeneratedArc for Reference {
        fn into_generated_arc(
            self,
            schema: CurrentSchemaRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            let mut reference = self.reference;
            if !reference.starts_with("ref:") {
                reference = format!("ref:{reference}");
            }

            let resolved = schema.resolve_ref(reference)?;
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
                        Ok(schema.resolve_ref(string)?.into_vec().unwrap_or(vec![]))
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

        fn get_transform(&self) -> Option<Vec<Transform>> {
            self.transform.clone()
        }
    }
}
