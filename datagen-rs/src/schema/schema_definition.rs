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
    pub plugins: Option<IndexMap<String, Value>>,
    pub ignore_not_found_local_refs: Option<bool>,
    pub max_ref_cache_size: Option<usize>,
    /// Whether to serialize references to strings when
    /// referencing non-strings in a string schema.
    /// If not specified, the default is false and an
    /// error will be thrown if a non-string is referenced
    /// in a string schema.
    pub serialize_refs: Option<bool>,
    /// The serializer to use when serializing the generated data.
    /// If not specified, the default is JSON.
    pub serializer: Option<Serializer>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase", tag = "type"))]
pub enum Serializer {
    Json {
        pretty: Option<bool>,
    },
    Yaml {
        pretty: Option<bool>,
    },
    #[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
    Xml {
        root_element: String,
    },
    #[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
    Plugin {
        plugin_name: String,
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

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Schema {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub value: AnyValue,
    pub options: Option<SchemaOptions>,
}
