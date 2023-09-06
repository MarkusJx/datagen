use crate::generate::current_schema::CurrentSchema;
use crate::generate::generated_schema::GeneratedSchema;
use crate::util::types::Result;
use serde_json::Value;
use std::fmt::Debug;
use std::sync::Arc;

pub type PluginInitResult = std::result::Result<*mut dyn Plugin, *mut std::ffi::c_char>;

pub trait Plugin: Debug {
    fn name(&self) -> &'static str;
    fn generate(&self, schema: Arc<CurrentSchema>, args: Value) -> Result<Arc<GeneratedSchema>>;
    fn transform(
        &self,
        schema: Arc<CurrentSchema>,
        value: Arc<GeneratedSchema>,
        args: Value,
    ) -> Result<Arc<GeneratedSchema>>;
}

pub trait PluginConstructor: Plugin + Sized {
    fn new(args: Box<Value>) -> Result<Self>;
}

#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor: expr) => {
        impl PluginConstructor for $plugin_type {
            fn new(args: Box<Value>) -> Result<Self> {
                Ok($constructor)
            }
        }

        declare_plugin!($plugin_type);
    };
    ($plugin_type:ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn _plugin_create(
            args: *mut Value,
        ) -> datagen_rs::plugins::plugin::PluginInitResult {
            // make sure the constructor is the correct type.
            let constructor: fn(args: Box<Value>) -> Result<$plugin_type> = <$plugin_type>::new;

            match constructor(Box::from_raw(args)) {
                Ok(object) => {
                    let boxed: Box<dyn Plugin> = Box::new(object);
                    Ok(Box::into_raw(boxed))
                }
                Err(err) => Err(std::ffi::CString::new(err.to_string()).unwrap().into_raw()),
            }
        }

        #[no_mangle]
        pub extern "C" fn _plugin_version() -> *mut std::ffi::c_char {
            std::ffi::CString::new("1.0.0").unwrap().into_raw()
        }
    };
}
