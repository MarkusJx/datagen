use crate::classes::node_plugin::NodePlugin;
use crate::runner::node_plugin_args::NodePluginArgs;
use crate::runner::types::{DropRefsTsfn, PluginMap, PluginMapResult, RefArc};
use crate::util::traits::IntoNapiResult;
use anyhow::{anyhow, Context};
use datagen_rs::plugins::plugin_list::PluginList;
use datagen_rs::schema::schema_definition::{PluginInitArgs, Schema};
use log::debug;
use napi::threadsafe_function::{
    ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode,
};
use napi::{CallContext, Env, JsError, JsFunction, JsObject, JsUnknown, Ref, ValueType};
use nodejs::run_napi;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

type RefsVec = Arc<Mutex<Vec<RefArc>>>;

pub struct NodePluginLoader {
    args: Vec<NodePluginArgs>,
    sender: Sender<anyhow::Result<(PluginMap, Vec<RefArc>)>>,
}

pub struct NodeRunner {
    refs: RefsVec,
    drop_refs: DropRefsTsfn,
    load_plugins: Mutex<ThreadsafeFunction<NodePluginLoader>>,
    thread: Option<JoinHandle<()>>,
}

impl NodeRunner {
    /// Initialize the Node.js plugins from a schema.
    /// This method spawns a new Node.js instance and loads the plugins.
    /// This method can only be called once per process.
    /// If you want to load new plugins, use [`Self::load_new_plugins`].
    pub fn init(schema: &Schema) -> anyhow::Result<(Option<Self>, PluginMap)> {
        debug!("Initializing Node.js plugins");
        let node_plugins = Self::find_plugins(schema)?;
        if node_plugins.is_empty() {
            debug!("No Node.js plugins found");
            return Ok((None, PluginMap::new()));
        }

        let (sender, receiver) = channel::<PluginMapResult>();

        let refs = Arc::new(Mutex::new(Vec::new()));
        let refs_copy = refs.clone();
        let thread = std::thread::spawn(move || {
            run_napi(|env| {
                sender
                    .send(Self::load_plugins(env, node_plugins, &refs_copy))
                    .context("Failed to send result of Node.js plugin initialization")
                    .into_napi()
            })
            .context("Failed to initialize Node.js plugins")
            .unwrap();
        });

        let (res, drop_refs, load_plugins) = receiver
            .recv()
            .context("Failed to initialize Node.js plugins")??;

        Ok((
            Some(Self {
                drop_refs,
                load_plugins,
                refs,
                thread: Some(thread),
            }),
            res,
        ))
    }

    pub fn load_new_plugins(&self, schema: &Schema) -> anyhow::Result<PluginMap> {
        let load_plugins = self.load_plugins.lock().unwrap();
        let (sender, receiver) = channel();

        load_plugins.call(
            Ok(NodePluginLoader {
                sender,
                args: Self::find_plugins(schema).unwrap_or_default(),
            }),
            ThreadsafeFunctionCallMode::NonBlocking,
        );

        let (plugins, new_refs) = receiver
            .recv()
            .context("Failed to receive loaded plugins")??;

        let mut refs = self.refs.lock().unwrap();
        refs.extend(new_refs);

        Ok(plugins)
    }

    fn find_plugins(schema: &Schema) -> anyhow::Result<Vec<NodePluginArgs>> {
        Ok(
            PluginList::find_and_map_plugins(schema, None, |name, args, path| {
                if !name.starts_with("node:") && !path.starts_with("node:") {
                    return Ok(None);
                }

                let args = PluginInitArgs::Args {
                    path,
                    args: Some(args),
                };

                Ok(Some((
                    name.clone(),
                    NodePluginArgs::from_args(&name, &args),
                )))
            })?
            .into_values()
            .collect(),
        )
    }

    fn load_plugins(
        env: Env,
        node_plugins: Vec<NodePluginArgs>,
        refs: &RefsVec,
    ) -> PluginMapResult {
        let mut refs_guard = refs.lock().unwrap();
        let res = Self::load_plugin_list(env, node_plugins, &mut refs_guard);

        let refs_copy = refs.clone();
        let drop_refs = env.create_function_from_closure("dropRefs", move |ctx| {
            let refs = refs_copy.lock().unwrap();
            refs.iter().for_each(|r| {
                let mut r = r.lock().unwrap();
                r.unref(*ctx.env).unwrap();
            });

            Ok(())
        })?;

        let load_plugins = env.create_function_from_closure("loadPlugins", |ctx| {
            let loader: &NodePluginLoader = ctx.env.unwrap(&ctx.get(1)?)?;
            let mut refs = vec![];

            loader
                .sender
                .send(
                    Self::load_plugin_list(*ctx.env, loader.args.clone(), &mut refs)
                        .map(|r| (r, refs)),
                )
                .context("Failed to send result of Node.js plugin initialization")
                .into_napi()
        })?;

        Ok((
            res?,
            env.create_threadsafe_function(&drop_refs, 1, |_| Ok(Vec::<()>::new()))?,
            Mutex::new(env.create_threadsafe_function(
                &load_plugins,
                1,
                |ctx: ThreadSafeCallContext<NodePluginLoader>| {
                    let mut obj = ctx.env.create_object()?;
                    ctx.env.wrap(&mut obj, ctx.value)?;

                    Ok(vec![obj])
                },
            )?),
        ))
    }

    fn load_plugin_list(
        env: Env,
        args: Vec<NodePluginArgs>,
        refs: &mut Vec<RefArc>,
    ) -> anyhow::Result<PluginMap> {
        let require = env
            .get_global()?
            .get_named_property::<JsFunction>("require")?;

        args.into_iter()
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
                        .map(|p| Arc::new(p) as _)
                        .context(anyhow!("Failed to create plugin '{}'", plugin.name))?,
                ))
            })
            .collect::<anyhow::Result<_>>()
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
}

impl Drop for NodeRunner {
    fn drop(&mut self) {
        self.drop_refs
            .call(Ok(()), ThreadsafeFunctionCallMode::Blocking);
        let _ = self.thread.take().and_then(|t| t.join().ok());
    }
}
