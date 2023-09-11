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

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::current_schema::CurrentSchemaRef;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::plugins::transform::to_string::ToStringTransform;
    use crate::util::traits::TransformTrait;
    use crate::util::types::Result;
    use handlebars::Handlebars;
    use std::collections::HashMap;
    use std::sync::Arc;

    impl TransformTrait for ToStringTransform {
        fn transform(
            self,
            schema: CurrentSchemaRef,
            value: Arc<GeneratedSchema>,
        ) -> Result<Arc<GeneratedSchema>> {
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
                                        .or(schema.options().serialize_non_strings)
                                        .unwrap_or(false)
                                    {
                                        Ok((k, serde_json::to_string(v.as_ref())?))
                                    } else {
                                        Err("Cannot format non-string".into())
                                    }
                                }
                            })
                            .collect::<Result<HashMap<_, _>>>()?;

                        Ok(GeneratedSchema::String(hbs.render("template", &data)?).into())
                    }
                    _ => Err("Cannot format non-object".into()),
                },
                ToStringTransform::Default => serde_json::to_string(&value)
                    .map(GeneratedSchema::String)
                    .map_err(Into::into)
                    .map(Into::into),
            }
        }
    }
}
