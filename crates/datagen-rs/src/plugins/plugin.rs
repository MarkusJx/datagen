use crate::generate::current_schema::CurrentSchemaRef;
use crate::generate::generated_schema::GeneratedSchema;
use crate::util::types::Result;
use serde_json::Value;
use std::fmt::Debug;
use std::sync::Arc;

/// A plugin initialization result.
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

/// A `datagen` plugin.
/// Plugins are used to generate data for a schema, transform generated data,
/// and serialize generated data. Plugins are loaded dynamically at runtime.
/// Plugins are implemented in Rust and compiled to a shared library.
/// Plugins are loaded using the `libloading` crate.
///
/// You can use the [`datagen_rs::declare_plugin`] macro to export the plugin.
///
/// # Example
/// ```
/// use datagen_rs::plugins::plugin::Plugin;
/// use datagen_rs::generate::current_schema::CurrentSchemaRef;
/// use datagen_rs::generate::generated_schema::GeneratedSchema;
/// use datagen_rs::util::types::Result;
/// use serde_json::Value;
/// use std::sync::Arc;
///  
/// #[derive(Debug, Default)]
/// struct MyPlugin;
///  
/// impl Plugin for MyPlugin {
///     fn name(&self) -> String {
///         "my-plugin".into()
///     }
///  
///     // Optional: Implement the `transform` function
///     // if the plugin provides a transformer.
///     fn transform(
///         &self,
///         schema: CurrentSchemaRef,
///         value: Arc<GeneratedSchema>,
///         args: Value,
///     ) -> Result<Arc<GeneratedSchema>> {
///         // ...
///         Ok(value)
///     }
///  
///     // Optional: Implement the `generate` function
///     // if the plugin provides a generator.
///     fn generate(
///         &self,
///         schema: CurrentSchemaRef,
///         args: Value
///     ) -> Result<Arc<GeneratedSchema>> {
///         // ...
///         Ok(Arc::new(GeneratedSchema::None))
///     }
///  
///     // Optional: Implement the `serialize` function
///     // if the plugin provides a serializer.
///     fn serialize(
///         &self,
///         value: &Arc<GeneratedSchema>,
///         args: Value
///     ) -> Result<String> {
///         // ...
///         Ok("".into())
///     }
/// }
/// ```
pub trait Plugin: Debug {
    /// Returns the name of the plugin.
    /// The name of the plugin is used to
    /// identify the plugin if an error is thrown.
    fn name(&self) -> String;

    /// Generate random data with the given arguments.
    /// The `generate` function is optional.
    /// If the `generate` function is not implemented,
    /// the plugin will not be able to generate data.
    ///
    /// # Arguments
    /// * `schema` - The current schema.
    /// * `args` - The arguments to use when generating data.
    ///
    /// # Returns
    /// The generated data.
    #[allow(unused_variables)]
    fn generate(&self, schema: CurrentSchemaRef, args: Value) -> Result<Arc<GeneratedSchema>> {
        Err("Operation 'generate' is not supported".into())
    }

    /// Transform generated data with the given arguments.
    /// The `transform` function is optional.
    /// If the `transform` function is not implemented,
    /// the plugin will not be able to transform data.
    ///
    /// # Arguments
    /// * `schema` - The current schema.
    /// * `value` - The generated data to transform.
    /// * `args` - The arguments to use when transforming data.
    ///
    /// # Returns
    /// The transformed data.
    #[allow(unused_variables)]
    fn transform(
        &self,
        schema: CurrentSchemaRef,
        value: Arc<GeneratedSchema>,
        args: Value,
    ) -> Result<Arc<GeneratedSchema>> {
        Err("Operation 'transform' is not supported".into())
    }

    /// Serialize generated data to a string with the given arguments.
    /// The `serialize` function is optional.
    /// If the `serialize` function is not implemented,
    /// the plugin will not be able to serialize data.
    ///
    /// # Arguments
    /// * `value` - The generated data to serialize.
    /// * `args` - The arguments to use when serializing data.
    ///
    /// # Returns
    /// The serialized data.
    #[allow(unused_variables)]
    fn serialize(&self, value: &Arc<GeneratedSchema>, args: Value) -> Result<String> {
        Err("Operation 'serialize' is not supported".into())
    }
}

/// A plugin constructor.
/// Plugin constructors are used to create a plugin instance.
///
/// # Example
/// ```
/// use datagen_rs::plugins::plugin::{Plugin, PluginConstructor};
/// use datagen_rs::util::types::Result;
///
/// #[derive(Debug, Default)]
/// struct MyPlugin;
///
/// impl Plugin for MyPlugin {
///     fn name(&self) -> String {
///         "my-plugin".into()
///     }
/// }
///
/// impl PluginConstructor for MyPlugin {
///     fn new(args: serde_json::Value) -> Result<Self> {
///         Ok(Self)
///     }
/// }
/// ```
pub trait PluginConstructor: Plugin + Sized {
    /// Create a new plugin instance with the given arguments.
    fn new(args: Value) -> Result<Self>;
}

/// Declare a plugin.
/// This macro declares the necessary functions to export a plugin.
/// The plugin must implement the [`Plugin`] trait.
/// if the plugin implements the [`PluginConstructor`] trait,
/// no additional arguments are required. Otherwise,
/// the path to the plugin constructor must be provided.
/// The plugin constructor has the signature `fn() -> Result<Self>`.
///
/// # Example
/// ## Plugin with constructor
/// ```
/// use datagen_rs::plugins::plugin::{Plugin, PluginConstructor};
/// use datagen_rs::util::types::Result;
/// use serde_json::Value;
///
/// #[derive(Debug, Default)]
/// struct MyPlugin;
///
/// impl Plugin for MyPlugin {
///     fn name(&self) -> String {
///         "my-plugin".into()
///     }
/// }
///
/// impl PluginConstructor for MyPlugin {
///     fn new(args: serde_json::Value) -> Result<Self> {
///         Ok(Self)
///     }
/// }
///
/// datagen_rs::declare_plugin!(MyPlugin);
/// ```
///
/// ## Plugin without constructor
/// ```
/// use datagen_rs::plugins::plugin::{Plugin, PluginConstructor};
/// use datagen_rs::util::types::Result;
/// use datagen_rs::declare_plugin;
/// use serde_json::Value;
///
/// #[derive(Debug, Default)]
/// struct MyPlugin;
///
/// impl Plugin for MyPlugin {
///     fn name(&self) -> String {
///         "my-plugin".into()
///     }
/// }
///
/// declare_plugin!(MyPlugin, MyPlugin::default);
/// ```
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
