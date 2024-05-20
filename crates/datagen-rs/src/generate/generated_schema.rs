use crate::generate::datagen_context::DatagenContextRef;
use indexmap::IndexMap;
use ordered_float::OrderedFloat;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};
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

impl Display for GeneratedSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneratedSchema::None => write!(f, "None"),
            GeneratedSchema::Number(n) => write!(f, "{}", n),
            GeneratedSchema::Integer(n) => write!(f, "{}", n),
            GeneratedSchema::Bool(b) => write!(f, "{}", b),
            GeneratedSchema::String(s) => write!(f, "{}", s),
            GeneratedSchema::Array(a) => {
                write!(f, "[")?;
                for (i, v) in a.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            GeneratedSchema::Object(o) => {
                write!(f, "{{")?;
                for (i, (k, v)) in o.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
            GeneratedSchema::Value(v) => write!(f, "{}", v),
        }
    }
}

pub trait IntoRandom {
    fn into_random(self, schema: DatagenContextRef) -> anyhow::Result<Arc<GeneratedSchema>>;
}

#[cfg(feature = "map-schema")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::{GeneratedSchema, IntoRandom};
    use crate::schema::transform::Transform;
    use crate::util::traits::generate::TransformTrait;
    use std::sync::Arc;

    pub(crate) trait IntoGenerated: Sized {
        fn into_generated(self, schema: DatagenContextRef) -> anyhow::Result<GeneratedSchema>;

        fn get_transform(&self) -> Option<Vec<Transform>>;

        fn should_finalize(&self) -> bool {
            true
        }
    }

    pub(crate) trait IntoGeneratedArc: Sized {
        fn into_generated_arc(
            self,
            schema: DatagenContextRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>>;

        fn get_transform(&self) -> Option<Vec<Transform>>;

        fn should_finalize(&self) -> bool {
            true
        }
    }

    impl<T> IntoGeneratedArc for T
    where
        T: IntoGenerated,
    {
        fn into_generated_arc(
            self,
            schema: DatagenContextRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            Ok(Arc::new(self.into_generated(schema)?))
        }

        fn get_transform(&self) -> Option<Vec<Transform>> {
            self.get_transform()
        }

        fn should_finalize(&self) -> bool {
            self.should_finalize()
        }
    }

    impl<T> IntoRandom for T
    where
        T: IntoGeneratedArc,
    {
        fn into_random(self, schema: DatagenContextRef) -> anyhow::Result<Arc<GeneratedSchema>> {
            let transform = self.get_transform();
            let should_finalize = self.should_finalize();

            let mut res = self.into_generated_arc(schema.clone())?;
            if let Some(transform) = transform {
                res = transform.transform(schema.clone(), res)?;
            }

            Ok(if should_finalize {
                schema.finalize(res)?
            } else {
                res
            })
        }
    }
}
