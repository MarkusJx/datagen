#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
pub struct RegexFilter {
    pub pattern: String,
    pub serialize_non_strings: Option<bool>,
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::current_schema::CurrentSchemaRef;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::plugins::transform::regex_filter::RegexFilter;
    use crate::util::traits::TransformTrait;
    use crate::util::types::Result;
    use std::sync::Arc;

    impl TransformTrait for RegexFilter {
        fn transform(
            self,
            schema: CurrentSchemaRef,
            value: Arc<GeneratedSchema>,
        ) -> Result<Arc<GeneratedSchema>> {
            let str = match value.as_ref() {
                GeneratedSchema::String(str) => str.clone(),
                _ => {
                    if self
                        .serialize_non_strings
                        .or(schema.options().serialize_non_strings)
                        .unwrap_or(false)
                    {
                        serde_json::to_string(&value)?
                    } else {
                        return Err("Cannot filter non-string value by regex".into());
                    }
                }
            };

            let regex = regex::Regex::new(&self.pattern)?;
            if regex.is_match(&str) {
                Ok(value)
            } else {
                Ok(GeneratedSchema::None.into())
            }
        }
    }
}
