use crate::generate::generated_schema::GeneratedSchema;
use crate::schema::transform::ReferenceOrString;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
pub struct FilterTransform {
    /// The field or reference to the field which will be used to compare the value
    /// with the other value specified in the `other` field. If this is not specified,
    /// the value itself will be used. Also if this is specified and used on an array or object,
    /// the whole array or object will be removed if the value of the field is not equal to the
    /// other value.
    pub field: Option<ReferenceOrString>,
    /// The operator which will be used to compare the value with the other value
    pub operator: FilterTransformOp,
    /// The value which will be used to compare the value with
    pub other: GeneratedSchema,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
pub enum FilterTransformOp {
    /// The value must be equal to the other value
    Equals,
    /// The value must not be equal to the other value
    NotEquals,
}

#[cfg(feature = "map-schema")]
pub mod generate {
    use crate::generate::current_schema::CurrentSchemaRef;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::transform::filter::{FilterTransform, FilterTransformOp};
    use crate::util::traits::generate::{ResolveRef, TransformTrait};
    use indexmap::IndexMap;
    use std::sync::Arc;

    impl TransformTrait for FilterTransform {
        fn transform(
            self,
            schema: CurrentSchemaRef,
            value: Arc<GeneratedSchema>,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            if let Some(current) = self
                .field
                .clone()
                .map(|reference| reference.resolve_ref(&schema))
                .map_or(Ok(None), |res| res.map(Some))?
            {
                let matches = match self.operator {
                    FilterTransformOp::Equals => *current == self.other,
                    FilterTransformOp::NotEquals => *current != self.other,
                };

                return if matches {
                    Ok(value)
                } else {
                    Ok(GeneratedSchema::None.into())
                };
            }

            match value.as_ref() {
                GeneratedSchema::Array(arr) => Ok(GeneratedSchema::Array(
                    arr.iter()
                        .map(|e| self.clone().transform(schema.clone(), e.clone()))
                        .collect::<anyhow::Result<Vec<_>>>()?,
                )
                .into()),
                GeneratedSchema::Object(obj) => Ok(GeneratedSchema::Object(
                    obj.iter()
                        .map(|(key, val)| {
                            Ok((
                                key.clone(),
                                self.clone().transform(schema.clone(), val.clone())?,
                            ))
                        })
                        .collect::<anyhow::Result<IndexMap<_, _>>>()?,
                )
                .into()),
                rest => {
                    let matches = match self.operator {
                        FilterTransformOp::Equals => *rest == self.other,
                        FilterTransformOp::NotEquals => *rest != self.other,
                    };

                    if matches {
                        Ok(value)
                    } else {
                        Ok(GeneratedSchema::None.into())
                    }
                }
            }
        }
    }
}
