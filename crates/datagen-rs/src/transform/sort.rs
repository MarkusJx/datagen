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

#[cfg(feature = "map-schema")]
pub mod generate {
    use crate::generate::current_schema::CurrentSchemaRef;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::transform::sort::SortTransform;
    use crate::util::traits::generate::TransformTrait;
    use anyhow::anyhow;
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
        type Error = anyhow::Error;

        fn try_from(value: &GeneratedSchema) -> Result<Self, Self::Error> {
            match value {
                GeneratedSchema::Number(number) => Ok(ComparableSchema::Number((*number).into())),
                GeneratedSchema::Integer(integer) => Ok(ComparableSchema::Integer(*integer)),
                GeneratedSchema::String(string) => Ok(ComparableSchema::String(string.clone())),
                GeneratedSchema::Bool(bool) => Ok(ComparableSchema::Bool(*bool)),
                GeneratedSchema::None => Ok(ComparableSchema::None),
                _ => Err(anyhow!("Cannot convert {} to comparable", value.name())),
            }
        }
    }

    fn find_by_key(value: &Arc<GeneratedSchema>, key: &str) -> anyhow::Result<ComparableSchema> {
        match value.as_ref() {
            GeneratedSchema::Object(obj) => {
                if let Some(value) = obj.get(key) {
                    value.as_ref().try_into()
                } else {
                    Err(anyhow!("Key '{}' not found in object", key))
                }
            }
            GeneratedSchema::None => Ok(ComparableSchema::None),
            _ => Err(anyhow!("Sort can only be applied to objects")),
        }
    }

    impl TransformTrait for SortTransform {
        fn transform(
            self,
            _schema: CurrentSchemaRef,
            value: Arc<GeneratedSchema>,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            if let GeneratedSchema::Array(arr) = value.as_ref() {
                let array = arr.clone();

                if let Some(by) = self.by {
                    let mut array = array
                        .into_iter()
                        .map(|e| Ok((find_by_key(&e, &by)?, e)))
                        .collect::<anyhow::Result<Vec<_>>>()?;

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
                        .collect::<anyhow::Result<Vec<(ComparableSchema, Arc<GeneratedSchema>)>>>(
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
                Err(anyhow!("Sort can only be applied to arrays"))
            }
        }
    }
}
