#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
pub struct SortTransform {
    pub by: Option<String>,
    pub reverse: Option<bool>,
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::current_schema::CurrentSchemaRef;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::plugins::transform::sort::SortTransform;
    use crate::util::traits::generate::TransformTrait;
    use crate::util::types::Result;
    use std::error::Error;
    use std::sync::Arc;

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    enum ComparableSchema {
        Number(f64),
        Integer(i64),
        String(String),
        Bool(bool),
        None,
    }

    impl TryFrom<&GeneratedSchema> for ComparableSchema {
        type Error = Box<dyn Error>;

        fn try_from(value: &GeneratedSchema) -> std::result::Result<Self, Self::Error> {
            match value {
                GeneratedSchema::Number(number) => Ok(ComparableSchema::Number((*number).into())),
                GeneratedSchema::Integer(integer) => Ok(ComparableSchema::Integer(*integer)),
                GeneratedSchema::String(string) => Ok(ComparableSchema::String(string.clone())),
                GeneratedSchema::Bool(bool) => Ok(ComparableSchema::Bool(*bool)),
                GeneratedSchema::None => Ok(ComparableSchema::None),
                _ => Err(format!("Cannot convert {} to comparable", value.name()).into()),
            }
        }
    }

    fn find_by_key(value: &Arc<GeneratedSchema>, key: &str) -> Result<ComparableSchema> {
        match value.as_ref() {
            GeneratedSchema::Object(obj) => {
                if let Some(value) = obj.get(key) {
                    value.as_ref().try_into()
                } else {
                    Err(format!("Key '{}' not found in object", key).into())
                }
            }
            GeneratedSchema::None => Ok(ComparableSchema::None),
            _ => Err("Sort can only be applied to objects".into()),
        }
    }

    impl TransformTrait for SortTransform {
        fn transform(
            self,
            _schema: CurrentSchemaRef,
            value: Arc<GeneratedSchema>,
        ) -> Result<Arc<GeneratedSchema>> {
            if let GeneratedSchema::Array(arr) = value.as_ref() {
                let array = arr.clone();

                if let Some(by) = self.by {
                    let mut array = array
                        .into_iter()
                        .map(|e| Ok((find_by_key(&e, &by)?, e)))
                        .collect::<Result<Vec<_>>>()?;

                    array.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

                    if let Some(reverse) = self.reverse {
                        if reverse {
                            array.reverse();
                        }
                    }

                    Ok(Arc::new(GeneratedSchema::Array(
                        array.into_iter().map(|(_, e)| e).collect(),
                    )))
                } else {
                    let mut array = array
                        .into_iter()
                        .map(|e| Ok((e.as_ref().try_into()?, e)))
                        .collect::<Result<Vec<(ComparableSchema, Arc<GeneratedSchema>)>>>(
                    )?;
                    array.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

                    if let Some(reverse) = self.reverse {
                        if reverse {
                            array.reverse();
                        }
                    }

                    Ok(Arc::new(GeneratedSchema::Array(
                        array.into_iter().map(|(_, e)| e).collect(),
                    )))
                }
            } else {
                Err("Sort can only be applied to arrays".into())
            }
        }
    }
}
