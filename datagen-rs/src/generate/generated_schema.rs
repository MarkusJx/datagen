use crate::generate::current_schema::CurrentSchemaRef;
#[cfg(feature = "generate")]
use crate::schema::transform::AnyTransform;
#[cfg(feature = "generate")]
use crate::util::traits::TransformTrait;
use crate::util::types::Result;
use indexmap::IndexMap;
use ordered_float::OrderedFloat;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum GeneratedSchema {
    None,
    Number(OrderedFloat<f64>),
    Integer(i64),
    Bool(bool),
    String(String),
    Array(Vec<Arc<GeneratedSchema>>),
    Object(IndexMap<String, Arc<GeneratedSchema>>),
    Value(Value),
}

impl GeneratedSchema {
    pub fn name(&self) -> &'static str {
        match self {
            GeneratedSchema::None => "None",
            GeneratedSchema::Number(_) => "Number",
            GeneratedSchema::Integer(_) => "Integer",
            GeneratedSchema::Bool(_) => "Bool",
            GeneratedSchema::String(_) => "String",
            GeneratedSchema::Array(_) => "Array",
            GeneratedSchema::Object(_) => "Object",
            GeneratedSchema::Value(_) => "Value",
        }
    }
}

#[cfg(feature = "generate")]
pub(crate) trait IntoGenerated: Sized {
    fn into_generated(self, schema: CurrentSchemaRef) -> Result<GeneratedSchema>;

    fn get_transform(&self) -> Option<Vec<AnyTransform>>;

    fn should_finalize(&self) -> bool {
        true
    }
}

#[cfg(feature = "generate")]
pub(crate) trait IntoGeneratedArc: Sized {
    fn into_generated_arc(self, schema: CurrentSchemaRef) -> Result<Arc<GeneratedSchema>>;

    fn get_transform(&self) -> Option<Vec<AnyTransform>>;

    fn should_finalize(&self) -> bool {
        true
    }
}

#[cfg(feature = "generate")]
impl<T> IntoGeneratedArc for T
where
    T: IntoGenerated,
{
    fn into_generated_arc(self, schema: CurrentSchemaRef) -> Result<Arc<GeneratedSchema>> {
        Ok(Arc::new(self.into_generated(schema)?))
    }

    fn get_transform(&self) -> Option<Vec<AnyTransform>> {
        self.get_transform()
    }

    fn should_finalize(&self) -> bool {
        self.should_finalize()
    }
}

pub trait IntoRandom {
    fn into_random(self, schema: CurrentSchemaRef) -> Result<Arc<GeneratedSchema>>;
}

#[cfg(feature = "generate")]
impl<T> IntoRandom for T
where
    T: IntoGeneratedArc,
{
    fn into_random(self, schema: CurrentSchemaRef) -> Result<Arc<GeneratedSchema>> {
        let transform = self.get_transform();
        let should_finalize = self.should_finalize();

        let mut res = self.into_generated_arc(schema.clone())?;
        if let Some(transform) = transform {
            res = transform.transform(schema.clone(), res)?;
        }

        Ok(if should_finalize {
            schema.finalize(res)
        } else {
            res
        })
    }
}
