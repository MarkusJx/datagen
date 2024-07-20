#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serialize",
    serde(tag = "subType", rename_all = "camelCase", deny_unknown_fields)
)]
pub enum ToStringTransform {
    /// The default transform will serialize the generated schema to a string
    /// using serde_json.
    Default,
    /// The format transform will serialize the generated schema to a string
    /// using a handlebars template.
    #[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
    Format {
        /// The handlebars template
        format: String,
        /// Whether to convert non-string values to strings
        /// using serde_json.
        serialize_non_strings: Option<bool>,
    },
}

#[cfg(feature = "map-schema")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::transform::to_string::ToStringTransform;
    use crate::util::generate_error::GenerateError;
    use crate::util::traits::generate::TransformTrait;
    use anyhow::{anyhow, Context};
    use handlebars::Handlebars;
    use std::collections::HashMap;
    use std::sync::Arc;

    impl TransformTrait for ToStringTransform {
        fn transform(
            self,
            schema: DatagenContextRef,
            value: Arc<GeneratedSchema>,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            match self {
                ToStringTransform::Format {
                    format,
                    serialize_non_strings,
                } => match value.as_ref() {
                    GeneratedSchema::Object(map) => {
                        let mut hbs = Handlebars::new();
                        hbs.register_template_string("template", format)?;
                        hbs.set_strict_mode(true);

                        let data = map
                            .iter()
                            .map(|(k, v)| match v.as_ref() {
                                GeneratedSchema::String(s) => Ok((k, s.clone())),
                                GeneratedSchema::Number(num) => Ok((k, num.to_string())),
                                GeneratedSchema::Bool(b) => Ok((k, b.to_string())),
                                GeneratedSchema::Integer(i) => Ok((k, i.to_string())),
                                _ => {
                                    if serialize_non_strings
                                        .or(schema.options()?.serialize_non_strings)
                                        .unwrap_or(false)
                                    {
                                        Ok((k, serde_json::to_string(v.as_ref())?))
                                    } else {
                                        Err(GenerateError::new(
                                            &schema,
                                            &format!("Cannot format non-string value '{k}', which is of type {}", v.name()))
                                            .into()
                                        )
                                    }
                                }
                            })
                            .collect::<anyhow::Result<HashMap<_, _>>>()?;

                        Ok(GeneratedSchema::String(hbs.render("template", &data)?).into())
                    }
                    _ => Err(anyhow!("Cannot format non-object")),
                },
                ToStringTransform::Default => serde_json::to_string(&value)
                    .map(GeneratedSchema::String)
                    .context("Failed to serialize generated schema to string")
                    .map(Into::into),
            }
        }
    }
}

#[cfg(feature = "validate-schema")]
pub mod validate {
    use crate::transform::to_string::ToStringTransform;
    use crate::validation::path::ValidationPath;
    use crate::validation::result::{ValidationErrors, ValidationResult};
    use crate::validation::validate::Validate;
    use handlebars::Handlebars;

    impl Validate for ToStringTransform {
        fn validate(&self, path: &ValidationPath) -> ValidationResult {
            match self {
                ToStringTransform::Format { format, .. } => {
                    if format.is_empty() {
                        return Err(ValidationErrors::single(
                            "format must not be empty",
                            &path,
                            None,
                            None,
                        ));
                    }

                    let mut hbs = Handlebars::new();
                    if let Err(e) = hbs.register_template_string("template", format) {
                        return Err(ValidationErrors::single(
                            "invalid handlebars template",
                            &path,
                            Some(e.into()),
                            None,
                        ));
                    }
                }
                ToStringTransform::Default => {}
            }

            Ok(())
        }
    }
}
