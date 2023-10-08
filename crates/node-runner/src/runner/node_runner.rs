use crate::classes::node_plugin::NodePlugin;
use crate::runner::node_plugin_args::NodePluginArgs;
use crate::runner::types::{DropRefsTsfn, PluginMap, PluginMapResult, RefArc};
use crate::util::napi::run_napi;
use anyhow::{anyhow, Context};
use datagen_rs::plugins::plugin::Plugin;
use datagen_rs::schema::schema_definition::{PluginInitArgs, Schema};
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use napi::{CallContext, Env, JsError, JsFunction, JsObject, JsUnknown, Ref, ValueType};
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};

pub struct NodeRunner {
    drop_refs: DropRefsTsfn,
}

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
                    .map(|(name, args)| NodePluginArgs::from_args(name, args))
                    .collect::<Vec<_>>()
            });

        let (sender, receiver) = channel::<PluginMapResult>();

        std::thread::spawn(move || {
            run_napi(|env| {
                sender
                    .send(Self::load_plugins(env, node_plugins))
                    .context("Failed to send result of Node.js plugin initialization")
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
        let require = env
            .get_global()?
            .get_named_property::<JsFunction>("require")?;

        let mut refs = Vec::new();

        let res = node_plugins
            .into_iter()
            .flatten()
            .map(|plugin| {
                let imported = plugin.import_plugin(env, &require)?;
                refs.push(imported.clone());

                let generate =
                    Self::create_plugin_closure(env, "generate", imported.clone(), false)?;
                let transform =
                    Self::create_plugin_closure(env, "transform", imported.clone(), true)?;
                let serialize = Self::create_plugin_closure(env, "serialize", imported, false)?;

                Ok((
                    plugin.name.clone(),
                    NodePlugin::new(plugin.name.clone(), generate, transform, serialize, env)
                        .map(|p| Box::new(p) as Box<dyn Plugin>)
                        .context(anyhow!("Failed to create plugin '{}'", plugin.name))?,
                ))
            })
            .collect::<anyhow::Result<HashMap<_, _>>>();

        let drop_refs = env.create_function_from_closure("dropRefs", move |ctx| {
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

    fn create_plugin_closure(
        env: Env,
        name: &str,
        imported_class: RefArc,
        third_arg: bool,
    ) -> napi::Result<JsFunction> {
        let func_name = name.to_string();
        env.create_function_from_closure(name, move |ctx| {
            if ctx.length < 2 {
                return if ctx.length == 1 && ctx.get::<JsUnknown>(0)?.get_type()? != ValueType::Null
                {
                    Err(napi::Error::from(ctx.get::<JsUnknown>(0)?))
                } else {
                    Err(napi::Error::from_reason(format!(
                        "Function '{func_name}' expects at least 2 arguments"
                    )))
                };
            }

            let callback: JsFunction = ctx.get(1)?;
            if let Err(e) = Self::create_callback(&func_name, &ctx, &imported_class, third_arg) {
                callback.call(None, &[JsError::from(e).into_unknown(env)])?;
            }

            Ok(())
        })
    }

    fn create_callback(
        name: &str,
        ctx: &CallContext,
        imported_class: &RefArc,
        third_arg: bool,
    ) -> napi::Result<()> {
        let imported_class = imported_class.lock().unwrap();
        let imported = ctx.env.get_reference_value::<JsObject>(&imported_class)?;
        let err: JsUnknown = ctx.get(0)?;
        let callback: JsFunction = ctx.get(1)?;
        let arg1: JsUnknown = ctx.get(2)?;
        let arg2: JsUnknown = ctx.get(3)?;

        if err.get_type()? != ValueType::Null {
            return Err(napi::Error::from(err));
        } else if !imported.has_property(name)? {
            return Err(napi::Error::from_reason(format!(
                "Plugin does not have a '{name}' function"
            )));
        }

        let mut args = vec![arg1, arg2];
        if third_arg {
            args.push(ctx.get(4)?);
        }

        let generate = imported.get_named_property::<JsFunction>(name)?;
        let generated: JsUnknown = generate.call(Some(&imported), args.as_slice())?;
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

        Ok(())
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
