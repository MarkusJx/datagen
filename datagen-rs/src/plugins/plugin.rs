use crate::generate::current_schema::CurrentSchemaRef;
use crate::generate::generated_schema::GeneratedSchema;
use crate::util::types::Result;
use serde_json::Value;
use std::fmt::Debug;
use std::sync::Arc;

#[repr(C)]
pub enum PluginInitResult {
    Ok(*mut dyn Plugin),
    Err(*mut std::ffi::c_char),
}

impl<T: Plugin + 'static> From<Result<T>> for PluginInitResult {
    fn from(result: Result<T>) -> Self {
        match result {
            Ok(value) => PluginInitResult::Ok(Box::into_raw(Box::new(value))),
            Err(err) => {
                PluginInitResult::Err(std::ffi::CString::new(err.to_string()).unwrap().into_raw())
            }
        }
    }
}

pub trait Plugin: Debug {
    fn name(&self) -> &'static str;

    #[allow(unused_variables)]
    fn generate(&self, schema: CurrentSchemaRef, args: Value) -> Result<Arc<GeneratedSchema>> {
        Err("Operation 'generate' is not supported".into())
    }

    #[allow(unused_variables)]
    fn transform(
        &self,
        schema: CurrentSchemaRef,
        value: Arc<GeneratedSchema>,
        args: Value,
    ) -> Result<Arc<GeneratedSchema>> {
        Err("Operation 'transform' is not supported".into())
    }

    #[allow(unused_variables)]
    fn serialize(&self, value: &Arc<GeneratedSchema>, args: Value) -> Result<String> {
        Err("Operation 'serialize' is not supported".into())
    }
}

pub trait PluginConstructor: Plugin + Sized {
    fn new(args: Value) -> Result<Self>;
}

#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor: path) => {
        impl PluginConstructor for $plugin_type {
            fn new(args: Value) -> Result<Self> {
                Ok($constructor())
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
            let constructor: fn(args: Value) -> Result<$plugin_type> = <$plugin_type>::new;

            let args = Box::from_raw(args);
            constructor(*args).into()
        }

        #[no_mangle]
        pub extern "C" fn _plugin_version() -> *mut std::ffi::c_char {
            std::ffi::CString::new("1.0.0").unwrap().into_raw()
        }
    };
}
