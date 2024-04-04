use crate::generate::datagen_context::DatagenContextRef;
use crate::generate::generated_schema::GeneratedSchema;
use crate::plugins::abi::{IntoAnyhow, JsonValue, PluginAbiBox};
use crate::plugins::plugin::{Plugin, PluginLibRef};
use abi_stable::library::RootModule;
use anyhow::{anyhow, Context};
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
        let lib = Self::try_load(path.to_string())?;
        let plugin = lib.new_plugin()(&mut JsonValue::read_from(args)?).into_anyhow()?;
        Ok(Self(PluginData { plugin, _lib: lib }.into()))
    }

    pub(crate) fn get_data(&self) -> Arc<PluginData> {
        self.0.clone()
    }

    fn try_load(path: String) -> anyhow::Result<PluginLibRef> {
        let err = match PluginLibRef::load_from_file(Path::new(&path)) {
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

        Err(anyhow::Error::new(err).context(anyhow!(
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
            if let Ok(lib) = PluginLibRef::load_from_file(Path::new(&path)) {
                return Ok(lib);
            }

            tried_paths.push(path.clone());
        }

        if !path.ends_with(LIB_EXTENSION) {
            path = format!("{}{}", path, LIB_EXTENSION);
        }

        if let Ok(lib) = PluginLibRef::load_from_file(Path::new(&path)) {
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
        PluginLibRef::load_from_file(Path::new(&path)).map_err(Into::into)
    }
}

impl Plugin for ImportedPlugin {
    fn name(&self) -> String {
        self.0.plugin.name().to_string()
    }

    fn generate(
        &self,
        schema: DatagenContextRef,
        args: Value,
    ) -> anyhow::Result<Arc<GeneratedSchema>> {
        self.0
            .plugin
            .generate(schema.into(), JsonValue::read_from(args)?)
            .into_anyhow()
            .and_then(TryInto::try_into)
            .context(format!(
                "Failed to call method 'generate' in plugin '{}'",
                self.name()
            ))
    }

    fn transform(
        &self,
        schema: DatagenContextRef,
        value: Arc<GeneratedSchema>,
        args: Value,
    ) -> anyhow::Result<Arc<GeneratedSchema>> {
        self.0
            .plugin
            .transform(
                schema.into(),
                value.try_into()?,
                JsonValue::read_from(args)?,
            )
            .into_anyhow()
            .and_then(TryInto::try_into)
            .context(format!(
                "Failed to call method 'transform' in plugin '{}'",
                self.name()
            ))
    }

    fn serialize(&self, value: &Arc<GeneratedSchema>, args: Value) -> anyhow::Result<String> {
        self.0
            .plugin
            .serialize(value.try_into()?, JsonValue::read_from(args)?)
            .map(Into::into)
            .into_anyhow()
            .context(format!(
                "Failed to call method 'transform' in plugin '{}'",
                self.name()
            ))
    }
}
