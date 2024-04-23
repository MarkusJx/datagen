use crate::generate::datagen_context::DatagenContextRef;
use crate::generate::generated_schema::GeneratedSchema;
#[cfg(feature = "plugin-abi")]
use crate::plugins::abi::{
    CurrentSchemaAbiBox, GeneratedSchemaAbi, JsonValue, PluginAbi, PluginAbiBox, PluginResult,
    SerializeCallback, WrapResult,
};
#[cfg(feature = "plugin-abi")]
use abi_stable::library::RootModule;
#[cfg(feature = "plugin-abi")]
use abi_stable::sabi_types::VersionStrings;
#[cfg(feature = "plugin-abi")]
use abi_stable::std_types::RString;
#[cfg(feature = "plugin-abi")]
use abi_stable::{package_version_strings, StableAbi};
use anyhow::anyhow;
#[cfg(feature = "plugin")]
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;
use std::sync::Arc;

/// A plugin serialize callback.
/// The callback is used to report progress when serializing data.
pub type PluginSerializeCallback =
    Box<dyn (Fn(usize, usize) -> anyhow::Result<()>) + Send + Sync + 'static>;

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
/// use datagen_rs::generate::generated_schema::GeneratedSchema;
/// use serde_json::Value;
/// use std::sync::Arc;
/// use datagen_rs::generate::datagen_context::DatagenContextRef;
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
///         schema: DatagenContextRef,
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
///         schema: DatagenContextRef,
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
pub trait Plugin: Send + Sync {
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
        schema: DatagenContextRef,
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
        schema: DatagenContextRef,
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
        callback: PluginSerializeCallback,
    ) -> anyhow::Result<String> {
        self.serialize(value, args)
    }
}

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = PluginLibRef)))]
#[sabi(missing_field(option))]
#[cfg(feature = "plugin-abi")]
pub struct PluginLib {
    #[sabi(last_prefix_field)]
    pub new_plugin: extern "C" fn(&mut JsonValue, &mut JsonValue) -> PluginResult<PluginAbiBox>,
}

#[cfg(feature = "plugin-abi")]
impl RootModule for PluginLibRef {
    abi_stable::declare_root_module_statics! {PluginLibRef}

    const BASE_NAME: &'static str = "datagen_plugin";
    const NAME: &'static str = "datagen_plugin";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}

#[cfg(feature = "plugin-abi")]
pub struct PluginContainer {
    plugin: Arc<dyn Plugin>,
}

#[cfg(feature = "plugin-abi")]
impl PluginContainer {
    pub fn new<T: Plugin + 'static>(plugin: T) -> Self {
        Self {
            plugin: Arc::new(plugin),
        }
    }

    pub fn from_arc(plugin: Arc<dyn Plugin>) -> Self {
        Self { plugin }
    }
}

#[cfg(feature = "plugin-abi")]
impl PluginAbi for PluginContainer {
    fn name(&self) -> RString {
        self.plugin.name().into()
    }

    fn generate(
        &self,
        schema: CurrentSchemaAbiBox,
        args: JsonValue,
    ) -> PluginResult<GeneratedSchemaAbi> {
        PluginResult::wrap(|| {
            self.plugin
                .generate(schema.clone().into(), args.parse_into()?)
                .and_then(TryInto::try_into)
        })
    }

    fn transform(
        &self,
        schema: CurrentSchemaAbiBox,
        value: GeneratedSchemaAbi,
        args: JsonValue,
    ) -> PluginResult<GeneratedSchemaAbi> {
        PluginResult::wrap(|| {
            self.plugin
                .transform(
                    schema.clone().into(),
                    value.clone().try_into()?,
                    args.parse_into()?,
                )
                .and_then(TryInto::try_into)
        })
    }

    fn serialize(&self, value: GeneratedSchemaAbi, args: JsonValue) -> PluginResult<RString> {
        PluginResult::wrap(|| {
            self.plugin
                .serialize(&value.clone().try_into()?, args.parse_into()?)
                .map(Into::into)
        })
    }

    fn serialize_with_progress(
        &self,
        value: GeneratedSchemaAbi,
        args: JsonValue,
        callback: SerializeCallback,
    ) -> PluginResult<RString> {
        PluginResult::wrap(move || {
            let callback_copy = callback.clone();

            self.plugin
                .serialize_with_progress(
                    &value.clone().try_into()?,
                    args.parse_into()?,
                    Box::new(move |current, total| callback_copy.call(current, total)),
                )
                .map(Into::into)
        })
    }
}

/// Plugin options
#[cfg_attr(feature = "plugin", derive(Serialize, Deserialize))]
pub struct PluginOptions {
    /// The current log level of the application
    log_level: String,
}

impl PluginOptions {
    /// Get the log level of the plugin.
    pub fn log_level(&self) -> log::LevelFilter {
        log::LevelFilter::from_str(&self.log_level).expect("Failed to parse log level")
    }
}

impl Default for PluginOptions {
    fn default() -> Self {
        Self {
            log_level: log::max_level().to_string(),
        }
    }
}

/// A plugin constructor.
/// Plugin constructors are used to create a plugin instance.
///
/// # Example
/// ```
/// use datagen_rs::plugins::plugin::{Plugin, PluginConstructor, PluginOptions};
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
///     fn new(args: serde_json::Value, options: PluginOptions) -> anyhow::Result<Self> {
///         Ok(Self)
///     }
/// }
/// ```
pub trait PluginConstructor: Plugin + Sized {
    /// Create a new plugin instance with the given arguments.
    fn new(args: Value, options: PluginOptions) -> anyhow::Result<Self>;
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
/// use datagen_rs::plugins::plugin::{Plugin, PluginConstructor, PluginOptions};
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
///     fn new(args: Value, options: PluginOptions) -> anyhow::Result<Self> {
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
            fn new(
                _args: serde_json::Value,
                _options: datagen_rs::plugins::plugin::PluginOptions,
            ) -> anyhow::Result<Self> {
                Ok($constructor())
            }
        }

        declare_plugin!($plugin_type);
    };
    ($plugin_type:ty) => {
        #[abi_stable::export_root_module]
        pub fn get_library() -> datagen_rs::plugins::plugin::PluginLibRef {
            use abi_stable::prefix_type::PrefixTypeTrait;

            datagen_rs::plugins::plugin::PluginLib { new_plugin }.leak_into_prefix()
        }

        #[abi_stable::sabi_extern_fn]
        pub fn new_plugin(
            value: &mut datagen_rs::plugins::abi::JsonValue,
            options: &mut datagen_rs::plugins::abi::JsonValue,
        ) -> datagen_rs::plugins::abi::PluginResult<datagen_rs::plugins::abi::PluginAbiBox> {
            use datagen_rs::plugins::abi::WrapResult;
            use datagen_rs::plugins::plugin::PluginConstructor;

            datagen_rs::plugins::abi::PluginResult::wrap(|| {
                Ok(datagen_rs::plugins::abi::PluginAbi_TO::from_value(
                    datagen_rs::plugins::plugin::PluginContainer::new(<$plugin_type>::new(
                        value.parse_into()?,
                        options.parse_into()?,
                    )?),
                    abi_stable::sabi_trait::TD_Opaque,
                ))
            })
        }
    };
}
