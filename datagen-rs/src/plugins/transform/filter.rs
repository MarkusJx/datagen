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
    /// the value itself will be used.
    field: Option<ReferenceOrString>,
    /// The operator which will be used to compare the value with the other value
    operator: FilterTransformOp,
    /// The value which will be used to compare the value with
    other: GeneratedSchema,
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

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::current_schema::CurrentSchemaRef;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::plugins::transform::filter::{FilterTransform, FilterTransformOp};
    use crate::util::traits::{ResolveRef, TransformTrait};
    use crate::util::types::Result;
    use std::sync::Arc;

    impl TransformTrait for FilterTransform {
        fn transform(
            self,
            schema: CurrentSchemaRef,
            value: Arc<GeneratedSchema>,
        ) -> Result<Arc<GeneratedSchema>> {
            let current = self
                .field
                .map(|reference| reference.resolve_ref(&schema))
                .map_or(Ok(None), |res| res.map(Some))?
                .unwrap_or_else(|| value.clone());

            let matches = match self.operator {
                FilterTransformOp::Equals => *current == self.other,
                FilterTransformOp::NotEquals => *current != self.other,
            };

            if matches {
                Ok(value)
            } else {
                Ok(GeneratedSchema::None.into())
            }
        }
    }
}
