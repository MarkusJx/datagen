use crate::plugins::transform::filter::FilterTransform;
use crate::plugins::transform::plugin_transform::PluginTransform;
use crate::plugins::transform::regex_filter::RegexFilter;
use crate::plugins::transform::sort::SortTransform;
use crate::plugins::transform::string_case_transform::ToLowerCase;
use crate::plugins::transform::string_case_transform::ToUpperCase;
use crate::plugins::transform::to_string::ToStringTransform;
use crate::schema::reference::Reference;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

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
    use crate::generate::current_schema::CurrentSchemaRef;
    use crate::generate::generated_schema::generate::IntoGeneratedArc;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::schema::transform::{ReferenceOrString, Transform};
    use crate::util::traits::generate::{ResolveRef, TransformTrait};
    use crate::util::types::Result;
    use indexmap::IndexMap;
    use std::sync::Arc;

    impl ResolveRef for ReferenceOrString {
        fn resolve_ref(self, schema: &CurrentSchemaRef) -> Result<Arc<GeneratedSchema>> {
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
            schema: CurrentSchemaRef,
            value: Arc<GeneratedSchema>,
        ) -> Result<Arc<GeneratedSchema>> {
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
                    _ => Err("FilterNonNull can only be used on arrays and objects".into()),
                },
                Transform::ToString(to_string) => to_string.transform(schema, value),
                Transform::ToLowerCase(to_lower_case) => to_lower_case.transform(schema, value),
                Transform::ToUpperCase(to_upper_case) => to_upper_case.transform(schema, value),
                Transform::RegexFilter(regex_filter) => regex_filter.transform(schema, value),
                Transform::Sort(sort) => sort.transform(schema, value),
                Transform::Plugin(plugin) => plugin.transform(schema, value),
            }
        }
    }
}
