use crate::schema::any_of::AnyOf;
use crate::schema::array::Array;
use crate::schema::bool::Bool;
use crate::schema::counter::Counter;
use crate::schema::file::File;
use crate::schema::flatten::Flatten;
use crate::schema::include::Include;
use crate::schema::integer::Integer;
use crate::schema::number::Number;
use crate::schema::object::Object;
use crate::schema::plugin::Plugin;
use crate::schema::reference::Reference;
use crate::schema::string::StringSchema;
use crate::schema::transform::MaybeValidTransform;
use crate::util::traits::GetTransform;
#[cfg(feature = "schema")]
use schemars::gen::SchemaGenerator;
#[cfg(feature = "schema")]
use schemars::schema::Schema;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum MaybeValidAny {
    Valid(Any),
    #[cfg_attr(
        feature = "schema",
        schemars(schema_with = "create_maybe_valid_any_schema")
    )]
    Invalid(serde_json::Value),
}

#[cfg(feature = "schema")]
fn create_maybe_valid_any_schema(gen: &mut SchemaGenerator) -> Schema {
    gen.subschema_for::<Any>()
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type", rename_all = "camelCase"))]
pub enum Any {
    Number(Number),
    Integer(Integer),
    Counter(Counter),
    Bool(Bool),
    String(StringSchema),
    AnyOf(AnyOf),
    Reference(Reference),
    Plugin(Plugin),
    Array(Box<Array>),
    Object(Box<Object>),
    Flatten(Flatten),
    File(File),
    Include(Include),
}

impl GetTransform for MaybeValidAny {
    fn get_transform(&self) -> Option<Vec<MaybeValidTransform>> {
        match self {
            MaybeValidAny::Valid(inner) => inner.get_transform(),
            MaybeValidAny::Invalid(_) => None,
        }
    }
}

impl GetTransform for Any {
    fn get_transform(&self) -> Option<Vec<MaybeValidTransform>> {
        None
    }
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGeneratedArc;
    use crate::generate::generated_schema::{GeneratedSchema, IntoRandom};
    use crate::schema::any::{Any, MaybeValidAny};
    use std::sync::Arc;

    impl IntoGeneratedArc for Any {
        fn into_generated_arc(
            self,
            schema: DatagenContextRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            match self {
                Any::Number(number) => number.into_random(schema),
                Any::Integer(integer) => integer.into_random(schema),
                Any::Counter(counter) => counter.into_random(schema),
                Any::Bool(bool) => bool.into_random(schema),
                Any::String(string) => string.into_random(schema),
                Any::AnyOf(any_of) => any_of.into_random(schema),
                Any::Reference(reference) => reference.into_random(schema),
                Any::Plugin(plugin) => plugin.into_random(schema),
                Any::Array(array) => array.into_random(schema),
                Any::Object(object) => object.into_random(schema),
                Any::Flatten(flatten) => flatten.into_random(schema),
                Any::File(file) => file.into_random(schema),
                Any::Include(include) => include.into_random(schema),
            }
        }
    }

    impl IntoGeneratedArc for MaybeValidAny {
        fn into_generated_arc(
            self,
            schema: DatagenContextRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            self.into_inner(&schema)?.into_generated_arc(schema)
        }
    }

    impl MaybeValidAny {
        pub fn into_inner(self, schema: &DatagenContextRef) -> anyhow::Result<Any> {
            match self {
                MaybeValidAny::Valid(inner) => Ok(inner),
                MaybeValidAny::Invalid(err) => Err(anyhow::anyhow!(
                    "Failed to parse schema at {}\nInvalid value was:\n{}",
                    schema.path()?.to_string(),
                    serde_json::to_string_pretty(&err).unwrap_or_default()
                )),
            }
        }
    }
}

#[cfg(feature = "validate-schema")]
pub mod validate {
    use crate::schema::any::{Any, MaybeValidAny};
    use crate::validation::path::ValidationPath;
    use crate::validation::result::{IterValidate, ValidationResult};
    use crate::validation::validate::{Validate, ValidateGenerateSchema};

    impl ValidateGenerateSchema for MaybeValidAny {
        fn validate_generate_schema(&self, path: &ValidationPath) -> ValidationResult {
            match self {
                MaybeValidAny::Valid(inner) => inner.validate_generate_schema(path),
                MaybeValidAny::Invalid(err) => ValidationResult::single(
                    "Failed to parse schema",
                    path,
                    None,
                    Some(err.clone()),
                ),
            }
        }
    }

    impl ValidateGenerateSchema for Any {
        fn validate_generate_schema(&self, path: &ValidationPath) -> ValidationResult {
            match self {
                Any::Number(number) => number.validate(path),
                Any::Integer(integer) => integer.validate(path),
                Any::Counter(counter) => counter.validate(path),
                Any::Bool(bool) => bool.validate(path),
                Any::String(string) => string.validate(path),
                Any::AnyOf(any_of) => any_of.validate(path),
                Any::Reference(reference) => reference.validate(path),
                Any::Plugin(plugin) => plugin.validate(path),
                Any::Array(array) => array.as_ref().validate(path),
                Any::Object(object) => object.as_ref().validate(path),
                Any::Flatten(flatten) => flatten.validate(path),
                Any::File(file) => file.validate(path),
                Any::Include(include) => include.validate(path),
            }
        }
    }
}
