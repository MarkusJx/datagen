use crate::schema::reference::Reference;
use crate::transform::choose_from_array::ChooseFromArray;
use crate::transform::filter::FilterTransform;
use crate::transform::plugin_transform::PluginTransform;
use crate::transform::random_remove::RandomRemoveTransform;
use crate::transform::regex_filter::RegexFilter;
use crate::transform::remove_all::RemoveAllTransform;
use crate::transform::sort::SortTransform;
use crate::transform::string_case_transform::ToLowerCase;
use crate::transform::string_case_transform::ToUpperCase;
use crate::transform::to_string::ToStringTransform;
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
pub enum MaybeValidTransform {
    Valid(Transform),
    #[cfg_attr(
        feature = "schema",
        schemars(schema_with = "create_maybe_valid_transform_schema")
    )]
    Invalid(serde_json::Value),
}

#[cfg(feature = "schema")]
fn create_maybe_valid_transform_schema(gen: &mut SchemaGenerator) -> Schema {
    gen.subschema_for::<Transform>()
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serialize",
    serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)
)]
pub enum Transform {
    Filter(FilterTransform),
    RegexFilter(RegexFilter),
    /// Remove all null values from the array or object.
    /// This can only be used on arrays and objects.
    FilterNonNull,
    ToString(ToStringTransform),
    ToUpperCase(ToUpperCase),
    ToLowerCase(ToLowerCase),
    Sort(SortTransform),
    Plugin(PluginTransform),
    RandomRemove(RandomRemoveTransform),
    RemoveAll(RemoveAllTransform),
    ChooseFromArray(ChooseFromArray),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(untagged, deny_unknown_fields))]
pub enum ReferenceOrString {
    Reference(Box<Reference>),
    String(String),
}

#[cfg(feature = "map-schema")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGeneratedArc;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::schema::transform::{MaybeValidTransform, ReferenceOrString, Transform};
    use crate::util::traits::generate::{ResolveRef, TransformTrait};
    use anyhow::anyhow;
    use indexmap::IndexMap;
    use std::sync::Arc;

    impl ResolveRef for ReferenceOrString {
        fn resolve_ref(self, schema: &DatagenContextRef) -> anyhow::Result<Arc<GeneratedSchema>> {
            match self {
                ReferenceOrString::Reference(reference) => {
                    reference.into_generated_arc(schema.clone())
                }
                ReferenceOrString::String(string) => string.resolve_ref(schema),
            }
        }
    }

    impl TransformTrait for Transform {
        fn transform(
            self,
            schema: DatagenContextRef,
            value: Arc<GeneratedSchema>,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            match self {
                Transform::Filter(filter) => filter.transform(schema, value),
                Transform::FilterNonNull => match value.as_ref() {
                    GeneratedSchema::Array(array) => {
                        let array = array
                            .iter()
                            .filter(|item| item.as_ref() != &GeneratedSchema::None)
                            .cloned()
                            .collect::<Vec<_>>();
                        Ok(GeneratedSchema::Array(array).into())
                    }
                    GeneratedSchema::Object(map) => {
                        let map = map
                            .iter()
                            .filter(|(_, item)| item.as_ref() != &GeneratedSchema::None)
                            .map(|(key, value)| (key.clone(), value.clone()))
                            .collect::<IndexMap<_, _>>();
                        Ok(GeneratedSchema::Object(map).into())
                    }
                    _ => Err(anyhow!(
                        "FilterNonNull can only be used on arrays and objects"
                    )),
                },
                Transform::ToString(to_string) => to_string.transform(schema, value),
                Transform::ToLowerCase(to_lower_case) => to_lower_case.transform(schema, value),
                Transform::ToUpperCase(to_upper_case) => to_upper_case.transform(schema, value),
                Transform::RegexFilter(regex_filter) => regex_filter.transform(schema, value),
                Transform::Sort(sort) => sort.transform(schema, value),
                Transform::Plugin(plugin) => plugin.transform(schema, value),
                Transform::RandomRemove(random_remove) => random_remove.transform(schema, value),
                Transform::RemoveAll(remove_all) => remove_all.transform(schema, value),
                Transform::ChooseFromArray(choose_from_array) => {
                    choose_from_array.transform(schema, value)
                }
            }
        }
    }

    impl TransformTrait for MaybeValidTransform {
        fn transform(
            self,
            schema: DatagenContextRef,
            value: Arc<GeneratedSchema>,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            match self {
                MaybeValidTransform::Valid(transform) => transform.transform(schema, value),
                MaybeValidTransform::Invalid(err) => Err(anyhow!(
                    "Failed to parse transform schema at {}\nInvalid value was:{}",
                    schema.path()?.to_string(),
                    serde_json::to_string(&err).unwrap_or_default()
                )),
            }
        }
    }
}

#[cfg(feature = "validate-schema")]
pub mod validate {
    use crate::schema::transform::{MaybeValidTransform, ReferenceOrString, Transform};
    use crate::validation::path::ValidationPath;
    use crate::validation::result::{ValidationErrors, ValidationResult};
    use crate::validation::validate::Validate;

    impl Validate for Transform {
        fn validate(&self, path: &ValidationPath) -> ValidationResult {
            match self {
                Transform::Filter(filter) => filter.validate(path),
                Transform::FilterNonNull => Ok(()),
                Transform::ToString(to_string) => to_string.validate(path),
                Transform::ToLowerCase(to_lower_case) => to_lower_case.validate(path),
                Transform::ToUpperCase(to_upper_case) => to_upper_case.validate(path),
                Transform::RegexFilter(regex_filter) => regex_filter.validate(path),
                Transform::Sort(sort) => sort.validate(path),
                Transform::Plugin(plugin) => plugin.validate(path),
                Transform::RandomRemove(random_remove) => random_remove.validate(path),
                Transform::RemoveAll(remove_all) => remove_all.validate(path),
                Transform::ChooseFromArray(_) => Ok(()),
            }
        }
    }

    impl Validate for MaybeValidTransform {
        fn validate(&self, path: &ValidationPath) -> ValidationResult {
            match self {
                MaybeValidTransform::Valid(transform) => transform.validate(path),
                MaybeValidTransform::Invalid(err) => Err(ValidationErrors::single(
                    "Failed to parse transform schema",
                    path,
                    None,
                    Some(err.clone()),
                )),
            }
        }
    }

    impl Validate for ReferenceOrString {
        fn validate(&self, path: &ValidationPath) -> ValidationResult {
            match self {
                ReferenceOrString::Reference(reference) => reference.validate(path),
                ReferenceOrString::String(_) => Ok(()),
            }
        }
    }
}
