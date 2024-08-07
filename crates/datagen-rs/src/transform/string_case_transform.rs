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
    /// Whether to apply the transform recursively.
    /// Defaults to false.
    pub recursive: Option<bool>,
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
    /// Whether to apply the transform recursively.
    /// Defaults to false.
    pub recursive: Option<bool>,
}

#[cfg(feature = "map-schema")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::transform::string_case_transform::{ToLowerCase, ToUpperCase};
    use crate::util::traits::generate::TransformTrait;
    use anyhow::anyhow;
    use indexmap::IndexMap;
    use std::sync::Arc;

    fn transform_value(
        upper_case: bool,
        serialize_non_strings: bool,
        recursive: bool,
        schema: DatagenContextRef,
        value: Arc<GeneratedSchema>,
    ) -> anyhow::Result<Arc<GeneratedSchema>> {
        let str = match value.as_ref() {
            GeneratedSchema::String(str) => str.clone(),
            GeneratedSchema::Integer(i) => i.to_string(),
            GeneratedSchema::Number(n) => n.to_string(),
            GeneratedSchema::Bool(b) => b.to_string(),
            rest => {
                if recursive {
                    return match rest {
                        GeneratedSchema::Object(obj) => Ok(GeneratedSchema::Object(
                            obj.iter()
                                .map(|(key, value)| {
                                    Ok((
                                        key.clone(),
                                        transform_value(
                                            upper_case,
                                            serialize_non_strings,
                                            recursive,
                                            schema.clone(),
                                            value.clone(),
                                        )?,
                                    ))
                                })
                                .collect::<anyhow::Result<IndexMap<_, _>>>()?,
                        )
                        .into()),
                        GeneratedSchema::Array(arr) => Ok(GeneratedSchema::Array(
                            arr.iter()
                                .map(|value| {
                                    transform_value(
                                        upper_case,
                                        serialize_non_strings,
                                        recursive,
                                        schema.clone(),
                                        value.clone(),
                                    )
                                })
                                .collect::<anyhow::Result<Vec<_>>>()?,
                        )
                        .into()),
                        _ => Err(anyhow!(
                            "Cannot convert non-string value '{}' to {}case",
                            rest.name(),
                            if upper_case { "upper" } else { "lower" }
                        )),
                    };
                } else if serialize_non_strings {
                    serde_json::to_string(&value)?
                } else {
                    return Err(anyhow!(
                        "Cannot convert non-string value '{}' to {}case",
                        value.name(),
                        if upper_case { "upper" } else { "lower" }
                    ));
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
            schema: DatagenContextRef,
            value: Arc<GeneratedSchema>,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            transform_value(
                true,
                self.serialize_non_strings
                    .or(schema.options()?.serialize_non_strings)
                    .unwrap_or(false),
                self.recursive.unwrap_or(false),
                schema,
                value,
            )
        }
    }

    impl TransformTrait for ToLowerCase {
        fn transform(
            self,
            schema: DatagenContextRef,
            value: Arc<GeneratedSchema>,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            transform_value(
                false,
                self.serialize_non_strings
                    .or(schema.options()?.serialize_non_strings)
                    .unwrap_or(false),
                self.recursive.unwrap_or(false),
                schema,
                value,
            )
        }
    }
}

#[cfg(feature = "validate-schema")]
pub mod validate {
    use crate::transform::string_case_transform::{ToLowerCase, ToUpperCase};
    use crate::validation::path::ValidationPath;
    use crate::validation::result::ValidationResult;
    use crate::validation::validate::Validate;

    impl Validate for ToUpperCase {
        fn validate(&self, _path: &ValidationPath) -> ValidationResult {
            Ok(())
        }
    }

    impl Validate for ToLowerCase {
        fn validate(&self, _path: &ValidationPath) -> ValidationResult {
            Ok(())
        }
    }
}
