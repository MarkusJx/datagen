use crate::generate::current_schema::CurrentSchemaRef;
use crate::generate::generated_schema::GeneratedSchema;
use crate::plugins::plugin::{Plugin, PluginInitResult};
use crate::util::plugin_error::MapPluginError;
use crate::util::types::Result;
use libloading::{Library, Symbol};
use serde_json::Value;
use std::ffi::{c_char, CString, OsStr};
use std::fmt::Display;
use std::rc::Rc;
use std::sync::Arc;

type InitFn = unsafe extern "C" fn(args: *mut Value) -> PluginInitResult;
type VersionFn = unsafe extern "C" fn() -> *mut c_char;

#[derive(Debug)]
pub(crate) struct PluginData {
    plugin: Box<dyn Plugin>,
    _lib: Library,
}

#[derive(Debug)]
pub struct ImportedPlugin(Rc<PluginData>);

impl ImportedPlugin {
    pub fn load<T: AsRef<OsStr> + Display + Clone>(path: T, args: Value) -> Result<Self> {
        let lib = unsafe { Library::new(path.clone()) }?;
        let constructor: Symbol<InitFn> = unsafe { lib.get(b"_plugin_create\0") }?;
        let version_fn: Symbol<VersionFn> = unsafe { lib.get(b"_plugin_version\0") }?;

        let version = unsafe { CString::from_raw(version_fn()) };
        let version_str = version.to_str()?;

        if version_str != "1.0.0" {
            Err(format!("Unsupported plugin version: {version_str}").into())
        } else {
            let args_raw = Box::into_raw(Box::new(args));
            match unsafe { constructor(args_raw) } {
                PluginInitResult::Ok(new_res) => {
                    let plugin = unsafe { Box::from_raw(new_res) };
                    Ok(Self(Rc::new(PluginData { plugin, _lib: lib })))
                }
                PluginInitResult::Err(err) => {
                    let err = unsafe { CString::from_raw(err) };
                    Err(format!("Failed to initialize plugin '{path}': {}", err.to_str()?).into())
                }
            }
        }
    }

    pub(crate) fn get_data(&self) -> Rc<PluginData> {
        self.0.clone()
    }
}

impl Plugin for ImportedPlugin {
    fn name(&self) -> &'static str {
        self.0.plugin.name()
    }

    fn generate(&self, schema: CurrentSchemaRef, args: Value) -> Result<Arc<GeneratedSchema>> {
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
    ) -> Result<Arc<GeneratedSchema>> {
        self.0
            .plugin
            .transform(schema, value, args)
            .map_plugin_error(self, "transform")
    }

    fn serialize(&self, value: &Arc<GeneratedSchema>, args: Value) -> Result<String> {
        self.0
            .plugin
            .serialize(value, args)
            .map_plugin_error(self, "serialize")
    }
}
