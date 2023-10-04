use crate::generate::current_schema::CurrentSchemaRef;
use crate::generate::generated_schema::GeneratedSchema;
use crate::plugins::plugin::{Plugin, PluginInitResult};
use crate::util::plugin_error::native::MapPluginError;
use anyhow::anyhow;
use libloading::{Library, Symbol};
use serde_json::Value;
use std::env;
use std::ffi::{c_char, CString, OsStr};
use std::fmt::Display;
use std::path::Path;
use std::sync::Arc;

#[allow(improper_ctypes_definitions)]
type InitFn = unsafe extern "C" fn(args: *mut Value) -> PluginInitResult;
type VersionFn = unsafe extern "C" fn() -> *mut c_char;

#[derive(Debug)]
pub(crate) struct PluginData {
    plugin: Box<dyn Plugin>,
    _lib: Library,
}

unsafe impl Send for PluginData {}
unsafe impl Sync for PluginData {}

#[cfg(target_os = "windows")]
const LIB_EXTENSION: &str = ".dll";
#[cfg(target_os = "linux")]
const LIB_EXTENSION: &str = ".so";
#[cfg(target_os = "macos")]
const LIB_EXTENSION: &str = ".dylib";

#[derive(Debug)]
pub struct ImportedPlugin(Arc<PluginData>);

impl ImportedPlugin {
    pub fn load<T: AsRef<OsStr> + Display + Clone>(path: T, args: Value) -> anyhow::Result<Self> {
        let lib = Self::try_load(path.to_string())?;
        let constructor: Symbol<InitFn> = unsafe { lib.get(b"_plugin_create\0") }?;
        let version_fn: Symbol<VersionFn> = unsafe { lib.get(b"_plugin_version\0") }?;

        let version = unsafe { CString::from_raw(version_fn()) };
        let version_str = version.to_str()?;

        if version_str != "1.0.0" {
            Err(anyhow!("Unsupported plugin version: {version_str}"))
        } else {
            let args_raw = Box::into_raw(Box::new(args));
            match unsafe { constructor(args_raw) } {
                PluginInitResult::Ok(new_res) => {
                    let plugin = unsafe { Box::from_raw(new_res) };
                    Ok(Self(Arc::new(PluginData { plugin, _lib: lib })))
                }
                PluginInitResult::Err(err) => {
                    let err = unsafe { CString::from_raw(err) };
                    Err(anyhow!("Failed to initialize plugin '{path}'")
                        .context(anyhow!(err.to_str()?.to_string())))
                }
            }
        }
    }

    pub(crate) fn get_data(&self) -> Arc<PluginData> {
        self.0.clone()
    }

    fn try_load(path: String) -> anyhow::Result<Library> {
        let err = match unsafe { Library::new(&path) } {
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
    ) -> anyhow::Result<Library> {
        if let Some(prefix) = prefix {
            path = format!("{}/{}", prefix, path);
            if let Ok(lib) = unsafe { Library::new(&path) } {
                return Ok(lib);
            }

            tried_paths.push(path.clone());
        }

        if !path.ends_with(LIB_EXTENSION) {
            path = format!("{}{}", path, LIB_EXTENSION);
        }

        if let Ok(lib) = unsafe { Library::new(&path) } {
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
        unsafe { Library::new(path) }.map_err(Into::into)
    }
}

impl Plugin for ImportedPlugin {
    fn name(&self) -> String {
        self.0.plugin.name()
    }

    fn generate(
        &self,
        schema: CurrentSchemaRef,
        args: Value,
    ) -> anyhow::Result<Arc<GeneratedSchema>> {
        self.0
            .plugin
            .generate(schema, args)
            .map_plugin_error(self, "generate")
    }

    fn transform(
        &self,
        schema: CurrentSchemaRef,
        value: Arc<GeneratedSchema>,
        args: Value,
    ) -> anyhow::Result<Arc<GeneratedSchema>> {
        self.0
            .plugin
            .transform(schema, value, args)
            .map_plugin_error(self, "transform")
    }

    fn serialize(&self, value: &Arc<GeneratedSchema>, args: Value) -> anyhow::Result<String> {
        self.0
            .plugin
            .serialize(value, args)
            .map_plugin_error(self, "serialize")
    }
}
