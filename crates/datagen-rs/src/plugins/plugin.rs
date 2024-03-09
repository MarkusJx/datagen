use crate::generate::current_schema::CurrentSchemaRef;
use crate::generate::generated_schema::GeneratedSchema;
use anyhow::anyhow;
use serde_json::Value;
use std::fmt::Debug;
use std::sync::Arc;

/// The version of the plugin API.
/// All loaded plugins must match this version.
pub const SUPPORTED_PLUGIN_VERSION: &str = "1.1.0";

/// A plugin initialization result.
#[repr(C)]
pub enum PluginInitResult {
    Ok(*mut dyn Plugin),
    Err(*mut std::ffi::c_char),
}

impl<T: Plugin + 'static> From<anyhow::Result<T>> for PluginInitResult {
    fn from(result: anyhow::Result<T>) -> Self {
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
///     // Optional: Implement the `generate` function
///     // if the plugin provides a generator.
///     fn generate(
///         &self,
///         schema: CurrentSchemaRef,
///         args: Value
///     ) -> anyhow::Result<Arc<GeneratedSchema>> {
///         // ...
///         Ok(Arc::new(GeneratedSchema::None))
///     }
///
///     // Optional: Implement the `transform` function
///     // if the plugin provides a transformer.
///     fn transform(
///         &self,
///         schema: CurrentSchemaRef,
///         value: Arc<GeneratedSchema>,
///         args: Value,
///     ) -> anyhow::Result<Arc<GeneratedSchema>> {
///         // ...
///         Ok(value)
///     }
///  
///     // Optional: Implement the `serialize` function
///     // if the plugin provides a serializer.
///     fn serialize(
///         &self,
///         value: &Arc<GeneratedSchema>,
///         args: Value
///     ) -> anyhow::Result<String> {
///         // ...
///         Ok("".into())
///     }
/// }
/// ```
pub trait Plugin: Debug + Send + Sync {
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
    fn generate(
        &self,
        schema: CurrentSchemaRef,
        args: Value,
    ) -> anyhow::Result<Arc<GeneratedSchema>> {
        Err(anyhow!("Operation 'generate' is not supported"))
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
    ) -> anyhow::Result<Arc<GeneratedSchema>> {
        Err(anyhow!("Operation 'transform' is not supported"))
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
    fn serialize(&self, value: &Arc<GeneratedSchema>, args: Value) -> anyhow::Result<String> {
        Err(anyhow!("Operation 'serialize' is not supported"))
    }

    /// Serialize generated data to a string with the given arguments and a progress callback.
    /// The `serialize_with_progress` function is optional and will call
    /// [`serialize`] by default.
    ///
    /// # Arguments
    /// * `value` - The generated data to serialize.
    /// * `args` - The arguments to use when serializing data.
    /// * `callback` - A `fn(current: usize, total: usize) -> ()` callback to call with the current progress.
    ///
    /// # Returns
    /// The serialized data.
    #[allow(unused_variables)]
    fn serialize_with_progress(
        &self,
        value: &Arc<GeneratedSchema>,
        args: Value,
        callback: &dyn Fn(usize, usize),
    ) -> anyhow::Result<String> {
        self.serialize(value, args)
    }
}

/// A plugin constructor.
/// Plugin constructors are used to create a plugin instance.
///
/// # Example
/// ```
/// use datagen_rs::plugins::plugin::{Plugin, PluginConstructor};
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
///     fn new(args: serde_json::Value) -> anyhow::Result<Self> {
///         Ok(Self)
///     }
/// }
/// ```
pub trait PluginConstructor: Plugin + Sized {
    /// Create a new plugin instance with the given arguments.
    fn new(args: Value) -> anyhow::Result<Self>;
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
///     fn new(args: Value) -> anyhow::Result<Self> {
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
        impl datagen_rs::plugins::plugin::PluginConstructor for $plugin_type {
            fn new(args: serde_json::Value) -> anyhow::Result<Self> {
                Ok($constructor())
            }
        }

        declare_plugin!($plugin_type);
    };
    ($plugin_type:ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn _plugin_create(
            args: *mut serde_json::Value,
        ) -> datagen_rs::plugins::plugin::PluginInitResult {
            use datagen_rs::plugins::plugin::PluginConstructor;

            // make sure the constructor is the correct type.
            let constructor: fn(args: serde_json::Value) -> anyhow::Result<$plugin_type> =
                <$plugin_type>::new;

            let args = Box::from_raw(args);
            constructor(*args).into()
        }

        #[no_mangle]
        pub extern "C" fn _plugin_version() -> *mut std::ffi::c_char {
            std::ffi::CString::new(datagen_rs::plugins::plugin::SUPPORTED_PLUGIN_VERSION)
                .unwrap()
                .into_raw()
        }
    };
}
