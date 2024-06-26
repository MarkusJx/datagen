#![deny(clippy::all)]

mod util;

#[macro_use]
extern crate napi_derive;

use crate::util::helpers::{generate_random_data_with_progress, EnvExt};
use datagen_rs::schema::schema_definition::Schema;
use datagen_rs::util::helpers::get_schema_value;
use datagen_rs_node_runner::classes::node_plugin::NodePlugin;
use datagen_rs_node_runner::util::traits::IntoNapiResult;
use datagen_rs_progress_plugin::{PluginWithSchemaResult, ProgressPlugin};
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use napi::{Env, JsFunction, JsObject};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[napi]
pub fn get_schema() -> napi::Result<Value> {
    get_schema_value().into_napi()
}

#[napi]
pub async fn get_schema_async() -> napi::Result<Value> {
    get_schema_value().into_napi()
}

fn parse_schema(schema: Value) -> napi::Result<Schema> {
    datagen_rs::util::json_deserialize::from_value(schema)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi(object)]
pub struct GenerateProgress {
    pub current: u32,
    pub total: u32,
}

#[napi(ts_return_type = "Promise<string>")]
pub fn generate_random_data_internal(
    env: Env,
    schema: Value,
    #[napi(
        ts_arg_type = "((err: Error | null, value: GenerateProgress) => void) | null | undefined"
    )]
    generate_progress: Option<JsFunction>,
    #[napi(
        ts_arg_type = "((err: Error | null, value: GenerateProgress) => void) | null | undefined"
    )]
    serialize_progress: Option<JsFunction>,
    additional_plugins: HashMap<String, &NodePlugin>,
) -> napi::Result<JsObject> {
    let generate_progress = env.create_tsfn(generate_progress)?;
    let serialize_progress = env.create_tsfn(serialize_progress)?;

    let additional_plugins = additional_plugins
        .into_iter()
        .map(|(name, plugin)| (name, plugin.clone()))
        .collect::<HashMap<_, _>>();

    env.execute_tokio_future(
        async move {
            let (schema, mut plugins) = if let Some(callback) = generate_progress {
                let PluginWithSchemaResult { schema, plugins } =
                    ProgressPlugin::with_schema(parse_schema(schema)?, move |current, total| {
                        callback.call(
                            Ok(GenerateProgress {
                                current: current as _,
                                total: total as _,
                            }),
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
                    plugins.insert(name, Arc::new(plugin.clone()));
                }
            }

            generate_random_data_with_progress(
                schema,
                serialize_progress.map(|func| {
                    move |current, total| {
                        let status = func.call(
                            Ok(GenerateProgress {
                                current: current as _,
                                total: total as _,
                            }),
                            ThreadsafeFunctionCallMode::NonBlocking,
                        );

                        if status == napi::Status::Ok {
                            Ok(())
                        } else {
                            Err(anyhow::anyhow!(
                                "Failed to call progress callback: {}",
                                status
                            ))
                        }
                    }
                }),
                plugins,
            )
            .into_napi()
        },
        |env, result| env.create_string_from_std(result),
    )
}
