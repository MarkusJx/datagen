#[cfg(feature = "generate")]
use crate::generate::current_schema::CurrentSchemaRef;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::GeneratedSchema;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::IntoGeneratedArc;
use crate::schema::transform::AnyTransform;
#[cfg(feature = "generate")]
use crate::util::types::Result;
#[cfg(feature = "generate")]
use rand::seq::SliceRandom;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "generate")]
use std::sync::Arc;
#[cfg(feature = "generate")]
use std::vec;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
pub struct Reference {
    pub reference: String,
    pub except: Option<Vec<StringOrNumber>>,
    pub keep_all: Option<bool>,
    pub transform: Option<Vec<AnyTransform>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(untagged, deny_unknown_fields))]
pub enum StringOrNumber {
    String(String),
    Number(f64),
}

#[cfg(feature = "generate")]
impl IntoGeneratedArc for Reference {
    fn into_generated_arc(self, schema: CurrentSchemaRef) -> Result<Arc<GeneratedSchema>> {
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
            .collect::<Result<Vec<_>>>()?
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

    fn get_transform(&self) -> Option<Vec<AnyTransform>> {
        self.transform.clone()
    }
}
