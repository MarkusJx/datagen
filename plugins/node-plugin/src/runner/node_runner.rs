use crate::classes::node_plugin::NodePlugin;
use crate::util::napi::run_napi;
use anyhow::{anyhow, Context};
use datagen_rs::plugins::plugin::Plugin;
use datagen_rs::schema::schema_definition::{PluginInitArgs, Schema};
use napi::threadsafe_function::ThreadsafeFunction;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use napi::{CallContext, Env, JsError, JsFunction, JsObject, JsUnknown, Ref, ValueType};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct NodePluginArgs {
    name: String,
    path: String,
    args: Value,
}

pub struct NodeRunner {
    drop_refs: DropRefsTsfn,
}

type DropRefsTsfn = ThreadsafeFunction<()>;
type PluginMap = HashMap<String, Box<dyn Plugin>>;
type PluginMapResult = anyhow::Result<(PluginMap, DropRefsTsfn)>;

impl NodeRunner {
    pub fn init(schema: &Schema) -> anyhow::Result<(Self, PluginMap)> {
        let node_plugins = schema
            .options
            .as_ref()
            .and_then(|o| o.plugins.as_ref())
            .map(|plugins| {
                plugins
                    .iter()
                    .filter(|(name, args)| Self::is_node_plugin(name, args))
                    .map(|(name, args)| Self::map_to_args(name, args))
                    .collect::<Vec<_>>()
            });

        let (sender, receiver) = channel::<PluginMapResult>();

        std::thread::spawn(move || {
            run_napi(|env, _| {
                sender.send(Self::load_plugins(env, node_plugins)).unwrap();

                Ok(())
            })
            .context("Failed to initialize Node.js plugins")
            .unwrap();
        });

        let (res, drop_refs) = receiver
            .recv()
            .context("Failed to initialize Node.js plugins")??;

        Ok((Self { drop_refs }, res))
    }

    fn load_plugins(env: Env, node_plugins: Option<Vec<NodePluginArgs>>) -> PluginMapResult {
        let console = env
            .get_global()?
            .get_named_property::<JsObject>("console")?;

        let log = console.get_named_property::<JsFunction>("log")?;
        log.call(Some(&console), &[env.create_string("test")?])?;

        let require = env
            .get_global()?
            .get_named_property::<JsFunction>("require")?;

        let mut refs = Vec::new();

        let res = node_plugins
            .into_iter()
            .flatten()
            .map(|plugin| {
                let imported = require
                    .call(None, &[env.create_string(&plugin.path)?])
                    .context(anyhow!(
                        "Failed to import plugin '{}' at '{}'",
                        plugin.name,
                        plugin.path
                    ))?;

                let import_res = if imported.get_type()? == ValueType::Function {
                    unsafe { imported.cast::<JsFunction>() }
                        .call(None, &[env.to_js_value(&plugin.args)?])
                        .context(anyhow!(
                            "Failed to call init function of plugin '{}'",
                            plugin.name
                        ))?
                } else {
                    let imported = imported.coerce_to_object()?;
                    if imported.has_property("default")? {
                        imported
                            .get_named_property::<JsFunction>("default")
                            .context(anyhow!(
                                "Failed to get property 'default' of plugin '{}'",
                                plugin.name
                            ))?
                            .call(None, &[env.to_js_value(&plugin.args)?])
                            .context(anyhow!(
                                "Failed to call init function of plugin '{}'",
                                plugin.name
                            ))?
                    } else {
                        return Err(anyhow!(
                            "Plugin '{}' at '{}' does not have a default export",
                            plugin.name,
                            plugin.path
                        ));
                    }
                }
                .coerce_to_object()?;

                let import_ref = Arc::new(Mutex::new(env.create_reference(import_res)?));
                let generate_ref = import_ref.clone();
                let generate = env.create_function_from_closure("generate", move |ctx| {
                    if ctx.length < 2 {
                        return if ctx.length == 1
                            && ctx.get::<JsUnknown>(0)?.get_type()? != ValueType::Null
                        {
                            Err(napi::Error::from(ctx.get::<JsUnknown>(0)?))
                        } else {
                            Err(napi::Error::from_reason(
                                "Function 'generate' expects 2 arguments",
                            ))
                        };
                    }

                    let callback: JsFunction = ctx.get(1)?;
                    if let Err(e) = Self::generate_callback(&ctx, &generate_ref) {
                        callback.call(None, &[JsError::from(e).into_unknown(env)])?;
                    }

                    Ok(())
                })?;

                refs.push(import_ref);

                let transform = env.create_function_from_closure("transform", |_ctx| Ok(()))?;
                let serialize = env.create_function_from_closure("serialize", |_ctx| Ok(()))?;

                Ok((
                    plugin.name.clone(),
                    NodePlugin::new(plugin.name.clone(), generate, transform, serialize, env)
                        .map(|p| Box::new(p) as Box<dyn Plugin>)
                        .context(anyhow!("Failed to create plugin '{}'", plugin.name))?,
                ))
            })
            .collect::<anyhow::Result<HashMap<_, _>>>();

        let drop_refs = env.create_function_from_closure("dropRefs", move |ctx| {
            println!("Dropping Node.js plugins");
            refs.iter().for_each(|r| {
                let mut r = r.lock().unwrap();
                r.unref(*ctx.env).unwrap();
            });

            Ok(())
        })?;

        Ok((
            res?,
            env.create_threadsafe_function(&drop_refs, 1, |_| Ok(Vec::<()>::new()))?,
        ))
    }

    fn generate_callback(
        ctx: &CallContext,
        imported_class: &Arc<Mutex<Ref<()>>>,
    ) -> napi::Result<JsUnknown> {
        let imported_class = imported_class.lock().unwrap();
        let imported = ctx.env.get_reference_value::<JsObject>(&imported_class)?;
        let err: JsUnknown = ctx.get(0)?;
        let callback: JsFunction = ctx.get(1)?;
        let schema: JsUnknown = ctx.get(2)?;
        let args: JsUnknown = ctx.get(3)?;

        if err.get_type()? != ValueType::Null {
            return Err(napi::Error::from(err));
        } else if !imported.has_property("generate")? {
            return Err(napi::Error::from_reason(
                "Plugin does not have a 'generate' function",
            ));
        }

        let generate = imported.get_named_property::<JsFunction>("generate")?;
        let generated: JsUnknown = generate.call(Some(&imported), &[schema, args])?;
        if generated.is_promise()? {
            let promise = generated.coerce_to_object()?;
            let callback_ref = Arc::new(Mutex::new(ctx.env.create_reference(callback)?));
            let ok_callback = Self::get_promise_callback(ctx.env, callback_ref.clone())?;
            let err_callback = Self::get_promise_callback(ctx.env, callback_ref)?;

            promise
                .get_named_property::<JsFunction>("then")?
                .call(Some(&promise), &[ok_callback, err_callback])?;
        } else {
            callback.call(None, &[generated])?;
        }

        Ok(ctx.env.get_undefined()?.into_unknown())
    }

    fn get_promise_callback(env: &Env, callback: Arc<Mutex<Ref<()>>>) -> napi::Result<JsFunction> {
        env.create_function_from_closure("callback", move |ctx| {
            let res: JsUnknown = ctx.get(0)?;
            let mut callback = callback.lock().unwrap();
            let callback_func: JsFunction = ctx.env.get_reference_value(&callback)?;

            let res = callback_func.call(None, &[res]);
            callback.unref(*ctx.env)?;

            res
        })
    }

    fn normalize_path(path: &str) -> String {
        path.strip_prefix("node:").unwrap_or(path).to_string()
    }

    fn map_to_args(name: &str, args: &PluginInitArgs) -> NodePluginArgs {
        match args.clone() {
            PluginInitArgs::Args { path, args } => NodePluginArgs {
                name: name.to_string(),
                path: Self::normalize_path(&path),
                args: args.unwrap_or_default(),
            },
            PluginInitArgs::Value(value) => NodePluginArgs {
                name: name.to_string(),
                path: Self::normalize_path(name),
                args: value,
            },
        }
    }

    fn is_node_plugin(name: &str, args: &PluginInitArgs) -> bool {
        if name.starts_with("node:") {
            true
        } else if let PluginInitArgs::Args { path, .. } = args {
            path.starts_with("node:")
        } else {
            false
        }
    }
}

impl Drop for NodeRunner {
    fn drop(&mut self) {
        self.drop_refs
            .call(Ok(()), ThreadsafeFunctionCallMode::Blocking);
    }
}
