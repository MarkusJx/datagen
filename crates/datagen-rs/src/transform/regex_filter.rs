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

#[cfg(feature = "map-schema")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::transform::regex_filter::RegexFilter;
    use crate::util::traits::generate::TransformTrait;
    use anyhow::anyhow;
    use std::sync::Arc;

    impl TransformTrait for RegexFilter {
        fn transform(
            self,
            schema: DatagenContextRef,
            value: Arc<GeneratedSchema>,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            let str = match value.as_ref() {
                GeneratedSchema::String(str) => str.clone(),
                _ => {
                    if self
                        .serialize_non_strings
                        .or(schema.options()?.serialize_non_strings)
                        .unwrap_or(false)
                    {
                        serde_json::to_string(&value)?
                    } else {
                        return Err(anyhow!("Cannot filter non-string value by regex"));
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

#[cfg(feature = "validate-schema")]
pub mod validate {
    use crate::transform::regex_filter::RegexFilter;
    use crate::validation::path::ValidationPath;
    use crate::validation::result::{IterValidate, ValidationResult};
    use crate::validation::validate::Validate;
    use serde_json::Value;

    impl Validate for RegexFilter {
        fn validate(&self, path: &ValidationPath) -> ValidationResult {
            if self.pattern.is_empty() {
                return ValidationResult::single("pattern must not be empty", path, None, None);
            }

            ValidationResult::ensure_ok(
                regex::Regex::new(&self.pattern),
                "invalid regex pattern",
                path,
                Some(Value::String(self.pattern.clone())),
            )
        }
    }
}
