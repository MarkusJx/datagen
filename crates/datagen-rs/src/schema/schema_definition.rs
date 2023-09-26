use crate::schema::any_value::AnyValue;
use indexmap::IndexMap;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
pub struct SchemaOptions {
    /// Additional plugins to load.
    /// The key is the name of the plugin.
    /// The value is the arguments to pass to the plugin.
    pub plugins: Option<IndexMap<String, PluginInitArgs>>,
    pub ignore_not_found_local_refs: Option<bool>,
    /// The maximum number of items to keep in the reference cache.
    /// If not specified, the default is infinite.
    /// If the value is 0, the reference cache will be disabled.
    pub max_ref_cache_size: Option<usize>,
    /// Whether to serialize references to strings when
    /// referencing non-strings in a string schema.
    /// If not specified, the default is false and an
    /// error will be thrown if a non-string is referenced
    /// in a string schema.
    pub serialize_non_strings: Option<bool>,
    /// The serializer to use when serializing the generated data.
    /// If not specified, the default is JSON.
    pub serializer: Option<Serializer>,
}

/// Arguments to initialize a plugin.
/// The arguments are either a path to the plugin
/// or a value to pass to the plugin.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serialize",
    serde(untagged, deny_unknown_fields, rename_all = "camelCase")
)]
pub enum PluginInitArgs {
    /// Arguments for a plugin.
    /// These contain a path to the plugin and the arguments to pass to the plugin.
    Args {
        /// The path to the plugin.
        path: String,
        /// The arguments to pass to the plugin.
        args: Option<Value>,
    },
    /// Arguments for a plugin.
    Value(Value),
}

/// The serializer to use when serializing the generated data.
/// If not specified, the default is JSON.
/// The serializer is specified in the schema options.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase", tag = "type"))]
pub enum Serializer {
    /// The JSON serializer.
    Json {
        /// Whether to pretty print the JSON.
        /// If not specified, the default is false.
        pretty: Option<bool>,
    },
    /// The YAML serializer.
    Yaml,
    /// The XML serializer.
    /// The root element must be specified.
    #[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
    Xml {
        /// The name of the root element.
        root_element: String,
        /// Whether to pretty print the XML.
        /// If not specified, the default is false.
        pretty: Option<bool>,
    },
    /// A plugin serializer.
    #[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
    Plugin {
        /// The name of the plugin.
        plugin_name: String,
        /// The arguments to pass to the plugin.
        args: Option<Value>,
    },
}

impl Default for Serializer {
    fn default() -> Self {
        Serializer::Json { pretty: None }
    }
}

impl Default for &Serializer {
    fn default() -> Self {
        &Serializer::Json { pretty: None }
    }
}

/// A schema definition.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Schema {
    /// The schema value.
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub value: AnyValue,
    /// The schema options.
    pub options: Option<SchemaOptions>,
}
