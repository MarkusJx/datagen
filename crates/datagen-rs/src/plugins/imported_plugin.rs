use crate::generate::datagen_context::DatagenContextRef;
use crate::generate::generated_schema::GeneratedSchema;
use crate::plugins::abi::{IntoAnyhow, JsonValue, PluginAbiBox};
use crate::plugins::plugin::{Plugin, PluginLibRef, PluginOptions};
use abi_stable::library::lib_header_from_path;
use anyhow::{anyhow, Context};
use log::debug;
use serde_json::Value;
use std::env;
use std::ffi::OsStr;
use std::fmt::{Debug, Display};
use std::path::Path;
use std::sync::Arc;

pub(crate) struct PluginData {
    plugin: PluginAbiBox,
    _lib: PluginLibRef,
}

unsafe impl Send for PluginData {}
unsafe impl Sync for PluginData {}

#[cfg(target_os = "windows")]
const LIB_EXTENSION: &str = ".dll";
#[cfg(target_os = "linux")]
const LIB_EXTENSION: &str = ".so";
#[cfg(target_os = "macos")]
const LIB_EXTENSION: &str = ".dylib";

pub struct ImportedPlugin(Arc<PluginData>);

impl Debug for ImportedPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImportedPlugin")
            .field("name", &self.0.plugin.name())
            .finish()
    }
}

impl ImportedPlugin {
    pub fn load<T: AsRef<OsStr> + Display + Clone>(path: T, args: Value) -> anyhow::Result<Self> {
        debug!("Loading plugin from '{}' with args {}", path, args);
        let lib = Self::try_load(path.to_string())?;
        let plugin = lib.new_plugin()(
            &mut JsonValue::read_from(args).context("Failed to serialize plugin arguments")?,
            &mut JsonValue::read_from(PluginOptions::default())?,
        )
        .into_anyhow()
        .context("Failed to initialize plugin")?;

        debug!("Successfully loaded plugin '{}'", plugin.name());
        Ok(Self(PluginData { plugin, _lib: lib }.into()))
    }

    pub(crate) fn get_data(&self) -> Arc<PluginData> {
        self.0.clone()
    }

    fn load_from_path(path: &str) -> anyhow::Result<PluginLibRef> {
        lib_header_from_path(Path::new(path))?
            .init_root_module()
            .map_err(Into::into)
    }

    fn try_load(path: String) -> anyhow::Result<PluginLibRef> {
        debug!("Trying to load plugin from '{}'", path);
        let err = match Self::load_from_path(&path) {
            Ok(lib) => return Ok(lib),
            Err(err) => err,
        };

        let mut tried_paths = vec![path.clone()];
        if let Ok(lib) = Self::try_load_with_prefix(None, path.clone(), &mut tried_paths) {
            return Ok(lib);
        }

        if let Ok(prefix) = env::var("DATAGEN_PLUGIN_DIR") {
            if let Ok(lib) =
                Self::try_load_with_prefix(Some(prefix), path.clone(), &mut tried_paths)
            {
                return Ok(lib);
            }
        }

        if let Ok(Some(cur)) =
            env::current_exe().map(|p| p.parent().map(|p| p.to_str().unwrap().to_string()))
        {
            if let Ok(lib) = Self::try_load_with_prefix(Some(cur), path.clone(), &mut tried_paths) {
                return Ok(lib);
            }
        }

        Err(err.context(anyhow!(
            "Failed to load plugin '{}'. Tried paths: {}",
            tried_paths[0],
            tried_paths.join(", ")
        )))
    }

    fn try_load_with_prefix(
        prefix: Option<String>,
        mut path: String,
        tried_paths: &mut Vec<String>,
    ) -> anyhow::Result<PluginLibRef> {
        if let Some(prefix) = prefix {
            path = format!("{}/{}", prefix, path);
            debug!("Trying to load plugin from '{}'", path);
            if let Ok(lib) = Self::load_from_path(&path) {
                return Ok(lib);
            }

            tried_paths.push(path.clone());
        }

        if !path.ends_with(LIB_EXTENSION) {
            path = format!("{}{}", path, LIB_EXTENSION);
        }

        debug!("Trying to load plugin from '{}'", path);
        if let Ok(lib) = Self::load_from_path(&path) {
            return Ok(lib);
        }

        tried_paths.push(path.clone());
        if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
            let p = Path::new(&path);
            if let Some(file) = p.file_name() {
                let file = format!(
                    "lib{}",
                    file.to_str().ok_or(anyhow!("Failed to convert filename"))?
                );

                if let Some(parent) = p.parent() {
                    let parent = parent
                        .to_str()
                        .ok_or(anyhow!("Failed to convert parent path"))?;
                    if parent.is_empty() {
                        path = file;
                    } else {
                        path = format!("{}/{}", parent, file);
                    }
                } else {
                    path = file;
                }
            }
        }

        tried_paths.push(path.clone());
        debug!("Trying to load plugin from '{}'", path);
        Self::load_from_path(&path)
    }
}

impl Plugin for ImportedPlugin {
    fn name(&self) -> String {
        Plugin::name(&self.0.plugin)
    }

    fn generate(
        &self,
        schema: DatagenContextRef,
        args: Value,
    ) -> anyhow::Result<Arc<GeneratedSchema>> {
        Plugin::generate(&self.0.plugin, schema, args)
    }

    fn transform(
        &self,
        schema: DatagenContextRef,
        value: Arc<GeneratedSchema>,
        args: Value,
    ) -> anyhow::Result<Arc<GeneratedSchema>> {
        Plugin::transform(&self.0.plugin, schema, value, args)
    }

    fn serialize(&self, value: &Arc<GeneratedSchema>, args: Value) -> anyhow::Result<String> {
        Plugin::serialize(&self.0.plugin, value, args)
    }

    fn serialize_with_progress(
        &self,
        value: &Arc<GeneratedSchema>,
        args: Value,
        callback: &dyn Fn(usize, usize),
    ) -> anyhow::Result<String> {
        Plugin::serialize_with_progress(&self.0.plugin, value, args, callback)
    }
}
