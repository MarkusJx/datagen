use crate::generate::current_schema::CurrentSchemaRef;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::{GeneratedSchema, IntoGenerated};
use crate::schema::transform::AnyTransform;
#[cfg(feature = "generate")]
use crate::util::types::Result;
#[cfg(feature = "generate")]
use rand::Rng;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(untagged, deny_unknown_fields))]
pub enum Number {
    Random {
        #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
        min: Option<f64>,
        #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
        max: Option<f64>,
        #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
        precision: Option<u8>,
        transform: Option<Vec<AnyTransform>>,
    },
    Constant {
        value: f64,
        transform: Option<Vec<AnyTransform>>,
    },
}

#[cfg(feature = "generate")]
impl IntoGenerated for Number {
    fn into_generated(self, _: CurrentSchemaRef) -> Result<GeneratedSchema> {
        Ok(match self {
            Number::Constant { value, .. } => GeneratedSchema::Number(value.into()),
            Number::Random {
                min,
                max,
                precision,
                ..
            } => {
                let mut rng = rand::thread_rng();
                let min = min.unwrap_or(f64::MIN);
                let max = max.unwrap_or(f64::MAX);
                let mut value = rng.gen_range(min..=max);
                if let Some(precision) = precision {
                    value = (value * 10.0_f64.powi(precision as i32)).round()
                        / 10.0_f64.powi(precision as i32);
                }
                GeneratedSchema::Number(value.into())
            }
        })
    }

    fn get_transform(&self) -> Option<Vec<AnyTransform>> {
        match self {
            Number::Constant { transform, .. } => transform.clone(),
            Number::Random { transform, .. } => transform.clone(),
        }
    }
}
