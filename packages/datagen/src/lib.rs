#![deny(clippy::all)]

mod util;

#[macro_use]
extern crate napi_derive;

use crate::util::helpers::generate_random_data_with_progress;
use datagen_rs::schema::schema_definition::Schema;
use datagen_rs::util::helpers::get_schema_value;
use datagen_rs_node_runner::classes::node_plugin::NodePlugin;
use datagen_rs_node_runner::util::traits::IntoNapiResult;
use datagen_rs_progress_plugin::{PluginWithSchemaResult, ProgressPlugin};
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
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
    #[napi(
        ts_arg_type = "((err: Error | null, value: GenerateProgress) => void) | null | undefined"
    )]
    generate_progress: Option<ThreadsafeFunction<GenerateProgress>>,
    #[napi(
        ts_arg_type = "((err: Error | null, value: GenerateProgress) => void) | null | undefined"
    )]
    serialize_progress: Option<ThreadsafeFunction<GenerateProgress>>,
    additional_plugins: HashMap<String, &NodePlugin>,
) -> napi::Result<String> {
    let (schema, mut plugins) = if let Some(callback) = generate_progress {
        let PluginWithSchemaResult { schema, plugins } =
            ProgressPlugin::with_schema(parse_schema(schema)?, move |current, total| {
                let status = callback.call(
                    Ok(GenerateProgress {
                        current: current as _,
                        total: total as _,
                    }),
                    ThreadsafeFunctionCallMode::NonBlocking,
                );

                println!("status: {:?}", status);
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

    generate_random_data_with_progress(
        schema,
        serialize_progress.map(|p| {
            move |current, total| {
                p.call(
                    Ok(GenerateProgress {
                        current: current as _,
                        total: total as _,
                    }),
                    ThreadsafeFunctionCallMode::NonBlocking,
                );
            }
        }),
        plugins,
    )
    .into_napi()
}
