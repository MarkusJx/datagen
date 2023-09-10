#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serialize",
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
pub struct ToUpperCase {
    /// Whether to convert non-string values to strings
    /// using serde_json.
    pub serialize_non_strings: Option<bool>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serialize",
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
pub struct ToLowerCase {
    /// Whether to convert non-string values to strings
    /// using serde_json.
    pub serialize_non_strings: Option<bool>,
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::current_schema::CurrentSchemaRef;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::plugins::transform::string_transform::{ToLowerCase, ToUpperCase};
    use crate::util::traits::TransformTrait;
    use crate::util::types::Result;
    use std::sync::Arc;

    fn transform_value(
        upper_case: bool,
        serialize_non_strings: Option<bool>,
        schema: CurrentSchemaRef,
        value: Arc<GeneratedSchema>,
    ) -> Result<Arc<GeneratedSchema>> {
        let str = match value.as_ref() {
            GeneratedSchema::String(str) => str.clone(),
            GeneratedSchema::Integer(i) => i.to_string(),
            GeneratedSchema::Number(n) => n.to_string(),
            GeneratedSchema::Bool(b) => b.to_string(),
            _ => {
                if serialize_non_strings
                    .or(schema.options().serialize_refs)
                    .unwrap_or(false)
                {
                    serde_json::to_string(&value)?
                } else {
                    return Err("Cannot convert non-string value to upper/lowercase".into());
                }
            }
        };

        Ok(GeneratedSchema::String(if upper_case {
            str.to_uppercase()
        } else {
            str.to_lowercase()
        })
        .into())
    }

    impl TransformTrait for ToUpperCase {
        fn transform(
            self,
            schema: CurrentSchemaRef,
            value: Arc<GeneratedSchema>,
        ) -> Result<Arc<GeneratedSchema>> {
            transform_value(true, self.serialize_non_strings, schema, value)
        }
    }

    impl TransformTrait for ToLowerCase {
        fn transform(
            self,
            schema: CurrentSchemaRef,
            value: Arc<GeneratedSchema>,
        ) -> Result<Arc<GeneratedSchema>> {
            transform_value(false, self.serialize_non_strings, schema, value)
        }
    }
}
