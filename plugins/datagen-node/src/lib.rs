#![deny(clippy::all)]

pub mod classes;

#[macro_use]
extern crate napi_derive;

use crate::classes::node_plugin::NodePlugin;
use datagen_rs::plugins::plugin::Plugin;
use datagen_rs::util::helpers::get_schema_value;
use serde_json::Value;
use std::collections::HashMap;

#[napi]
pub fn get_schema() -> napi::Result<Value> {
    get_schema_value().map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub async fn get_schema_async() -> napi::Result<Value> {
    get_schema_value().map_err(|e| napi::Error::from_reason(e.to_string()))
}

pub fn generate_random_data(
    schema: Value,
    additional_plugins: HashMap<String, &NodePlugin>,
) -> napi::Result<String> {
    let schema =
        serde_json::from_value(schema).map_err(|e| napi::Error::from_reason(e.to_string()))?;

    let mut plugins: Option<HashMap<String, Box<dyn Plugin>>> = None;
    if !additional_plugins.is_empty() {
        plugins = Some(HashMap::new());
        for (name, plugin) in additional_plugins {
            plugins
                .as_mut()
                .unwrap()
                .insert(name, Box::new(plugin.clone()));
        }
    }

    datagen_rs::util::helpers::generate_random_data(schema, plugins)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn generate_random_data_internal(
    schema: Value,
    plugins: HashMap<String, &NodePlugin>,
) -> napi::Result<String> {
    generate_random_data(schema, plugins)
}

#[napi]
pub async fn generate_random_data_internal_async(
    schema: Value,
    plugins: HashMap<String, &NodePlugin>,
) -> napi::Result<String> {
    generate_random_data(schema, plugins)
}
