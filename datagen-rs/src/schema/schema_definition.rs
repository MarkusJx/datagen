use crate::schema::any_value::AnyValue;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
pub struct SchemaOptions {
    pub plugins: Option<BTreeMap<String, Value>>,
    pub ignore_not_found_local_refs: Option<bool>,
    pub max_ref_cache_size: Option<usize>,
    pub serialize_refs: Option<bool>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Schema {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub value: AnyValue,
    pub options: Option<SchemaOptions>,
}
