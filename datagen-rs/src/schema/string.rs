use crate::generate::current_schema::CurrentSchemaRef;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::{GeneratedSchema, IntoGenerated};
#[cfg(feature = "generate")]
use crate::generate::generated_schema::{IntoGeneratedArc, IntoRandom};
use crate::schema::reference::Reference;
use crate::schema::transform::AnyTransform;
#[cfg(feature = "generate")]
use crate::util::types::Result;
#[cfg(feature = "generate")]
use fake::faker::address::en::{
    CityName, CountryCode, CountryName, Latitude, Longitude, StateName, StreetName, ZipCode,
};
#[cfg(feature = "generate")]
use fake::faker::internet::en::{FreeEmail, Username};
#[cfg(feature = "generate")]
use fake::faker::name::en::{FirstName, LastName, Name};
#[cfg(feature = "generate")]
use fake::faker::phone_number::en::PhoneNumber;
#[cfg(feature = "generate")]
use fake::uuid::UUIDv4;
#[cfg(feature = "generate")]
use fake::Fake;
#[cfg(feature = "generate")]
use handlebars::Handlebars;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[cfg(feature = "generate")]
use std::collections::HashMap;
#[cfg(feature = "generate")]
use std::sync::Arc;

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
        transform: Option<Vec<AnyTransform>>,
    },
    Constant {
        value: String,
        transform: Option<Vec<AnyTransform>>,
    },
}

#[cfg(feature = "generate")]
impl IntoGeneratedArc for StringSchema {
    fn into_generated_arc(self, schema: CurrentSchemaRef) -> Result<Arc<GeneratedSchema>> {
        match self {
            StringSchema::Constant { value, .. } => schema.resolve_ref(value)?.into_random(),
            StringSchema::Generated { generator, .. } => generator.into_random(schema),
        }
    }

    fn get_transform(&self) -> Option<Vec<AnyTransform>> {
        match self {
            StringSchema::Constant { transform, .. } => transform.clone(),
            StringSchema::Generated { transform, .. } => transform.clone(),
        }
    }
}

#[cfg(feature = "generate")]
impl IntoGenerated for StringGenerator {
    fn into_generated(self, schema: CurrentSchemaRef) -> Result<GeneratedSchema> {
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
            StringGenerator::Latitude => GeneratedSchema::Number(Latitude().fake::<f64>().into()),
            StringGenerator::Longitude => GeneratedSchema::Number(Longitude().fake::<f64>().into()),
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
                    .map(|(name, arg)| -> Result<(String, Arc<GeneratedSchema>)> {
                        Ok((
                            name,
                            match arg {
                                FormatArg::Number(num) => {
                                    GeneratedSchema::String(num.to_string()).into()
                                }
                                FormatArg::Integer(num) => {
                                    GeneratedSchema::String(num.to_string()).into()
                                }
                                FormatArg::String(str) => schema.resolve_ref(str)?.into_random()?,
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
                    })
                    .collect::<Result<HashMap<_, _>>>()?
                    .into_iter()
                    .map(|(name, arg)| -> Result<(String, String)> {
                        if let GeneratedSchema::String(str) = arg.as_ref() {
                            Ok((name, str.clone()))
                        } else if serialize_non_strings
                            .or(schema.options().serialize_non_strings)
                            .unwrap_or(false)
                        {
                            Ok((name, serde_json::to_string(&arg)?))
                        } else {
                            Err(format!(
                                "Unable to format non-string value: {}",
                                serde_json::to_string(&arg)?
                            )
                            .into())
                        }
                    })
                    .collect::<Result<HashMap<_, _>>>()?;

                GeneratedSchema::String(hbs.render("template", &data)?)
            }
        })
    }

    fn get_transform(&self) -> Option<Vec<AnyTransform>> {
        None
    }

    fn should_finalize(&self) -> bool {
        false
    }
}
