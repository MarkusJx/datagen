#![deny(clippy::all)]

pub mod classes;

#[macro_use]
extern crate napi_derive;

use crate::classes::node_plugin::NodePlugin;
use datagen_rs::plugins::plugin::Plugin;
use datagen_rs::schema::any::Any;
use datagen_rs::schema::any_value::AnyValue;
use datagen_rs::schema::schema_definition::Schema;
use datagen_rs::util::helpers::get_schema_value;
use napi::threadsafe_function::ThreadsafeFunction;
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunctionCallMode};
use progress_plugin::ProgressPlugin;
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
pub async fn generate_random_data_internal_async(
    schema: Value,
    plugins: HashMap<String, &NodePlugin>,
) -> napi::Result<String> {
    generate_random_data(schema, plugins)
}

#[napi(object)]
pub struct GenerateProgress {
    pub current: u32,
    pub total: u32,
}

#[napi]
pub async fn generate_random_data_with_progress_internal(
    schema: Value,
    callback: ThreadsafeFunction<GenerateProgress, ErrorStrategy::Fatal>,
    additional_plugins: HashMap<String, &NodePlugin>,
) -> napi::Result<String> {
    let progress: Box<dyn Plugin> = Box::new(ProgressPlugin::new(move |current, total| {
        callback.call(
            GenerateProgress {
                current: current as _,
                total: total as _,
            },
            ThreadsafeFunctionCallMode::NonBlocking,
        );
    }));

    let mut schema: Schema =
        serde_json::from_value(schema).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    schema.value = AnyValue::Any(Any::Plugin(datagen_rs::schema::plugin::Plugin {
        plugin_name: "progress".into(),
        args: Some(serde_json::to_value(schema.value)?),
        transform: None,
    }));

    let mut plugins: HashMap<String, Box<dyn Plugin>> =
        vec![("progress".into(), progress)].into_iter().collect();
    if !additional_plugins.is_empty() {
        for (name, plugin) in additional_plugins {
            plugins.insert(name, Box::new(plugin.clone()));
        }
    }

    datagen_rs::util::helpers::generate_random_data(schema, Some(plugins))
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}
