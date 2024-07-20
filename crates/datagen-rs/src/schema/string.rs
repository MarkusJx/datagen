use crate::schema::reference::Reference;
use crate::schema::transform::MaybeValidTransform;
use crate::util::traits::GetTransform;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serialize",
    serde(deny_unknown_fields, tag = "type", rename_all = "camelCase")
)]
pub enum StringGenerator {
    Uuid,
    Email,
    FirstName,
    LastName,
    FullName,
    Username,
    City,
    Country,
    CountryCode,
    Street,
    State,
    ZipCode,
    Latitude,
    Longitude,
    Phone,
    #[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
    Format {
        format: String,
        args: BTreeMap<String, FormatArg>,
        serialize_non_strings: Option<bool>,
    },
    #[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
    DateTime {
        /// The format of the resulting date and time string.
        /// If not specified, the result will be in RFC 3339 format.
        /// Example: "%Y-%m-%d %H:%M:%S"
        format: Option<String>,
        /// The minimum date and time in RFC 3339 format.
        /// Example: "1996-12-19T16:39:57-08:00"
        from: Option<String>,
        /// The maximum date and time in RFC 3339 format.
        /// This date must be at lease one minute after the minimum date.
        /// Example: "1996-12-19T16:39:57-08:00"
        to: Option<String>,
    },
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(untagged, deny_unknown_fields))]
pub enum FormatArg {
    String(String),
    StringSchema(StringSchema),
    Integer(i32),
    Number(f64),
    Reference(Reference),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serialize",
    serde(untagged, deny_unknown_fields, rename = "string")
)]
pub enum StringSchema {
    Generated {
        generator: StringGenerator,
        transform: Option<Vec<MaybeValidTransform>>,
    },
    Constant {
        value: String,
        transform: Option<Vec<MaybeValidTransform>>,
    },
}

impl GetTransform for StringSchema {
    fn get_transform(&self) -> Option<Vec<MaybeValidTransform>> {
        match self {
            StringSchema::Constant { transform, .. } => transform.clone(),
            StringSchema::Generated { transform, .. } => transform.clone(),
        }
    }
}

impl GetTransform for StringGenerator {
    fn get_transform(&self) -> Option<Vec<MaybeValidTransform>> {
        None
    }
}

impl GetTransform for FormatArg {
    fn get_transform(&self) -> Option<Vec<MaybeValidTransform>> {
        None
    }
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::{IntoGenerated, IntoGeneratedArc};
    use crate::generate::generated_schema::{GeneratedSchema, IntoRandom};
    use crate::schema::string::{FormatArg, StringGenerator, StringSchema};
    use anyhow::{anyhow, Context};
    use chrono::{DateTime, SecondsFormat, Timelike, Utc};
    use fake::faker::address::en::{
        CityName, CountryCode, CountryName, Latitude, Longitude, StateName, StreetName, ZipCode,
    };
    use fake::faker::chrono::en::{DateTime, DateTimeAfter, DateTimeBefore, DateTimeBetween};
    use fake::faker::internet::en::{FreeEmail, Username};
    use fake::faker::name::en::{FirstName, LastName, Name};
    use fake::faker::phone_number::en::PhoneNumber;
    use fake::uuid::UUIDv4;
    use fake::Fake;
    use handlebars::Handlebars;
    use std::collections::HashMap;
    use std::sync::Arc;

    impl IntoGeneratedArc for StringSchema {
        fn into_generated_arc(
            self,
            schema: DatagenContextRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            match self {
                StringSchema::Constant { value, .. } => schema.resolve_ref(&value)?.into_random(),
                StringSchema::Generated { generator, .. } => generator.into_random(schema),
            }
        }
    }

    impl IntoGenerated for StringGenerator {
        fn into_generated(self, schema: DatagenContextRef) -> anyhow::Result<GeneratedSchema> {
            Ok(match self {
                StringGenerator::Uuid => GeneratedSchema::String(UUIDv4.fake()),
                StringGenerator::Email => GeneratedSchema::String(FreeEmail().fake()),
                StringGenerator::FirstName => GeneratedSchema::String(FirstName().fake()),
                StringGenerator::LastName => GeneratedSchema::String(LastName().fake()),
                StringGenerator::FullName => GeneratedSchema::String(Name().fake()),
                StringGenerator::Username => GeneratedSchema::String(Username().fake()),
                StringGenerator::City => GeneratedSchema::String(CityName().fake()),
                StringGenerator::Country => GeneratedSchema::String(CountryName().fake()),
                StringGenerator::CountryCode => GeneratedSchema::String(CountryCode().fake()),
                StringGenerator::Street => GeneratedSchema::String(StreetName().fake()),
                StringGenerator::State => GeneratedSchema::String(StateName().fake()),
                StringGenerator::ZipCode => GeneratedSchema::String(ZipCode().fake()),
                StringGenerator::Latitude => {
                    GeneratedSchema::Number(Latitude().fake::<f64>().into())
                }
                StringGenerator::Longitude => {
                    GeneratedSchema::Number(Longitude().fake::<f64>().into())
                }
                StringGenerator::Phone => GeneratedSchema::String(PhoneNumber().fake()),
                StringGenerator::Format {
                    format,
                    args,
                    serialize_non_strings,
                } => {
                    let mut hbs = Handlebars::new();
                    hbs.register_template_string("template", format)?;

                    let data = args
                        .into_iter()
                        .map(
                            |(name, arg)| -> anyhow::Result<(String, Arc<GeneratedSchema>)> {
                                Ok((
                                    name,
                                    match arg {
                                        FormatArg::Number(num) => {
                                            GeneratedSchema::String(num.to_string()).into()
                                        }
                                        FormatArg::Integer(num) => {
                                            GeneratedSchema::String(num.to_string()).into()
                                        }
                                        FormatArg::String(str) => {
                                            schema.resolve_ref(&str)?.into_random()?
                                        }
                                        FormatArg::StringSchema(str) => {
                                            let res = str.into_generated_arc(schema.clone())?;
                                            match res.as_ref() {
                                                GeneratedSchema::Number(num) => {
                                                    GeneratedSchema::String(num.to_string()).into()
                                                }
                                                GeneratedSchema::Integer(num) => {
                                                    GeneratedSchema::String(num.to_string()).into()
                                                }
                                                _ => res,
                                            }
                                        }
                                        FormatArg::Reference(reference) => {
                                            reference.into_generated_arc(schema.clone())?
                                        }
                                    },
                                ))
                            },
                        )
                        .collect::<anyhow::Result<HashMap<_, _>>>()?
                        .into_iter()
                        .map(|(name, arg)| -> anyhow::Result<(String, String)> {
                            if let GeneratedSchema::String(str) = arg.as_ref() {
                                Ok((name, str.clone()))
                            } else if serialize_non_strings
                                .or(schema.options()?.serialize_non_strings)
                                .unwrap_or(false)
                            {
                                Ok((name, serde_json::to_string(&arg)?))
                            } else {
                                Err(anyhow!(
                                    "Unable to format non-string value: {}",
                                    serde_json::to_string(&arg)?
                                ))
                            }
                        })
                        .collect::<anyhow::Result<HashMap<_, _>>>()?;

                    GeneratedSchema::String(hbs.render("template", &data)?)
                }
                StringGenerator::DateTime { format, from, to } => {
                    let date: DateTime<Utc> = if from.is_some() && to.is_some() {
                        let min: DateTime<Utc> = DateTime::parse_from_rfc3339(&from.unwrap())
                            .context("Failed to parse 'to' date")?
                            .into();
                        let max: DateTime<Utc> = DateTime::parse_from_rfc3339(&to.unwrap())
                            .context("Failed to parse 'to' date")?
                            .into();

                        if min
                            .with_second(0)
                            .ok_or(anyhow!("Failed to set seconds of 'from' date"))?
                            >= max
                                .with_second(0)
                                .ok_or(anyhow!("Failed to set seconds of 'to' date"))?
                        {
                            return Err(anyhow!(
                                "'From' date must be at least one minute before the 'to' date"
                            ));
                        }

                        DateTimeBetween(min, max).fake()
                    } else if let Some(min) = from {
                        DateTimeAfter(
                            DateTime::parse_from_rfc3339(&min)
                                .context("Failed to parse 'from' date")?
                                .into(),
                        )
                        .fake()
                    } else if let Some(max) = to {
                        DateTimeBefore(
                            DateTime::parse_from_rfc3339(&max)
                                .context("Failed to parse 'to' date")?
                                .into(),
                        )
                        .fake()
                    } else {
                        DateTime().fake()
                    };

                    if let Some(format) = format {
                        GeneratedSchema::String(date.format(&format).to_string())
                    } else {
                        GeneratedSchema::String(date.to_rfc3339_opts(SecondsFormat::Secs, true))
                    }
                }
            })
        }

        fn should_finalize(&self) -> bool {
            false
        }
    }
}

#[cfg(feature = "validate-schema")]
pub mod validate {
    use crate::schema::string::{FormatArg, StringGenerator, StringSchema};
    use crate::validation::path::ValidationPath;
    use crate::validation::result::{IterValidate, ValidationResult};
    use crate::validation::validate::{Validate, ValidateGenerateSchema};

    impl ValidateGenerateSchema for StringSchema {
        fn validate_generate_schema(&self, path: &ValidationPath) -> ValidationResult {
            match self {
                StringSchema::Generated { generator, .. } => generator.validate(path),
                StringSchema::Constant { .. } => Ok(()),
            }
        }
    }

    impl ValidateGenerateSchema for StringGenerator {
        fn validate_generate_schema(&self, path: &ValidationPath) -> ValidationResult {
            match self {
                StringGenerator::Format { format, args, .. } => ValidationResult::ensure(
                    !format.is_empty(),
                    "format must not be empty",
                    &path.append_single("format"),
                )
                .concat(ValidationResult::validate(args.iter(), |_, (name, arg)| {
                    arg.validate(&path.append("args", name))
                })),
                StringGenerator::DateTime { from, to, format } => from
                    .as_ref()
                    .map_or(Ok(()), |from| {
                        ValidationResult::ensure_ok(
                            chrono::DateTime::parse_from_rfc3339(from),
                            "from must be a valid RFC 3339 date",
                            &path.append_single("from"),
                            Some(serde_json::Value::String(from.clone())),
                        )
                    })
                    .concat(to.as_ref().map_or(Ok(()), |to| {
                        ValidationResult::ensure(
                            chrono::DateTime::parse_from_rfc3339(to).is_ok(),
                            "to must be a valid RFC 3339 date",
                            &path.append_single("to"),
                        )
                    }))
                    .concat(format.as_ref().map_or(Ok(()), |format| {
                        ValidationResult::ensure(
                            format.is_empty()
                                || chrono::DateTime::parse_from_str("2021-01-01 00:00:00", format)
                                    .is_ok(),
                            "format must be a valid date format",
                            &path.append_single("format"),
                        )
                    })),
                _ => Ok(()),
            }
        }
    }

    impl ValidateGenerateSchema for FormatArg {
        fn validate_generate_schema(&self, path: &ValidationPath) -> ValidationResult {
            match self {
                FormatArg::StringSchema(schema) => schema.validate(path),
                FormatArg::Reference(reference) => reference.validate(path),
                _ => Ok(()),
            }
        }
    }
}
