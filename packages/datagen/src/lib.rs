#![deny(clippy::all)]

mod util;

#[macro_use]
extern crate napi_derive;

use datagen_rs::schema::schema_definition::Schema;
use datagen_rs::util::helpers::{generate_random_data, get_schema_value};
use datagen_rs_node_runner::classes::node_plugin::NodePlugin;
use datagen_rs_node_runner::util::traits::IntoNapiResult;
use datagen_rs_progress_plugin::{PluginWithSchemaResult, ProgressPlugin};
use napi::threadsafe_function::ThreadsafeFunction;
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunctionCallMode};
use serde_json::Value;
use std::collections::HashMap;

#[napi]
pub fn get_schema() -> napi::Result<Value> {
    get_schema_value().into_napi()
}

#[napi]
pub async fn get_schema_async() -> napi::Result<Value> {
    get_schema_value().into_napi()
}

fn parse_schema(schema: Value) -> napi::Result<Schema> {
    serde_json::from_value(schema).map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi(object)]
pub struct GenerateProgress {
    pub current: u32,
    pub total: u32,
}

#[napi]
pub async fn generate_random_data_internal(
    schema: Value,
    #[napi(ts_arg_type = "((progress: GenerateProgress) => void) | null | undefined")]
    callback: Option<ThreadsafeFunction<GenerateProgress, ErrorStrategy::Fatal>>,
    additional_plugins: HashMap<String, &NodePlugin>,
) -> napi::Result<String> {
    let (schema, mut plugins) = if let Some(callback) = callback {
        let PluginWithSchemaResult { schema, plugins } =
            ProgressPlugin::with_schema(parse_schema(schema)?, move |current, total| {
                callback.call(
                    GenerateProgress {
                        current: current as _,
                        total: total as _,
                    },
                    ThreadsafeFunctionCallMode::NonBlocking,
                );
            })
            .into_napi()?;

        (schema, Some(plugins))
    } else {
        (parse_schema(schema)?, None)
    };

    if !additional_plugins.is_empty() {
        let plugins = plugins.get_or_insert_with(HashMap::new);

        for (name, plugin) in additional_plugins {
            plugins.insert(name, Box::new(plugin.clone()));
        }
    }

    generate_random_data(schema, plugins).into_napi()
}
