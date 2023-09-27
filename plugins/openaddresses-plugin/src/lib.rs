mod backends;
mod objects;
#[cfg(test)]
mod tests;

use crate::backends::backend::{Backend, BackendConstructor};
use crate::backends::memory_backend::MemoryBackend;
#[cfg(feature = "sqlite")]
use crate::backends::sqlite_backend::SQLiteBackend;
use crate::objects::args::{BackendType, IntoGenerated, PluginArgs, StringOrVec};
use crate::objects::call_args::CallArgs;
use datagen_rs::declare_plugin;
use datagen_rs::generate::current_schema::CurrentSchemaRef;
use datagen_rs::generate::generated_schema::GeneratedSchema;
use datagen_rs::plugins::plugin::{Plugin, PluginConstructor};
use datagen_rs::util::types::Result;
#[cfg(feature = "log")]
use log::LevelFilter;
#[cfg(feature = "log")]
use log4rs::append::console::ConsoleAppender;
#[cfg(feature = "log")]
use log4rs::config::{Appender, Root};
#[cfg(feature = "log")]
use log4rs::Config;
use serde_json::Value;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

#[cfg(feature = "sqlite")]
include!(concat!(env!("OUT_DIR"), "/build_vars.rs"));

/// A plugin for generating random addresses from the OpenAddresses dataset.
///
/// # Example
/// ```no_run
/// use datagen_rs::generate::generated_schema::GeneratedSchema;
/// use datagen_rs::generate::current_schema::CurrentSchemaRef;
/// use datagen_rs::plugins::plugin::Plugin;
/// use datagen_rs::plugins::openaddresses_plugin::OpenAddressesPlugin;
/// use datagen_rs::util::types::Result;
/// use serde_json::json;
/// use std::sync::Arc;
///
/// OpenAddressesPlugin::new(json!({
///     "files": "tests/data/openaddresses/us/ny/albany.geojson",
///      "backend": {
///         "type": "sqlite",
///         "databaseName": "albany.db",
///         "batchSize": 1000,
///         "cacheSize": 1000
///      }
/// })).unwrap();
/// ```
#[derive(Debug)]
pub struct OpenAddressesPlugin {
    backend: Mutex<Box<dyn Backend>>,
}

impl Plugin for OpenAddressesPlugin {
    fn name(&self) -> String {
        "openaddresses".into()
    }

    fn generate(&self, schema: CurrentSchemaRef, args: Value) -> Result<Arc<GeneratedSchema>> {
        let args: CallArgs = serde_json::from_value(args)?;
        let feature = self.backend.lock().unwrap().get_random_feature()?;

        args.into_generated(&schema, &feature)
    }
}

impl PluginConstructor for OpenAddressesPlugin {
    /// Create a new [`OpenAddressesPlugin`] from the given arguments.
    ///
    /// # Arguments
    /// * `args` - A JSON object which will be converted into a [`PluginArgs`] struct.
    ///
    /// # Example
    /// ```no_run
    /// use datagen_rs::plugins::plugin::PluginConstructor;
    /// use datagen_rs::plugins::openaddresses_plugin::OpenAddressesPlugin;
    /// use serde_json::json;
    /// use std::sync::Arc;
    ///
    /// let plugin = OpenAddressesPlugin::new(json!({
    ///     "files": "albany.geojson",
    ///     "backend": {
    ///         "type": "memory",
    ///     }
    /// })).unwrap();
    /// ```
    fn new(args: Value) -> Result<Self> {
        let args: PluginArgs = serde_json::from_value(args)?;
        let paths = match args.files.clone() {
            StringOrVec::Single(path) => vec![path],
            StringOrVec::Multiple(paths) => paths,
        };
        #[cfg(feature = "log")]
        log4rs::init_config(
            Config::builder()
                .appender(
                    Appender::builder()
                        .build("stdout", Box::new(ConsoleAppender::builder().build())),
                )
                .build(Root::builder().appender("stdout").build(LevelFilter::Debug))?,
        )?;

        #[cfg(feature = "log")]
        log::debug!("Initializing plugin 'openaddress'");

        let backend: Box<dyn Backend> = match &args.backend.clone().unwrap_or_default() {
            #[cfg(feature = "sqlite")]
            BackendType::SQLite { .. } => Box::new(SQLiteBackend::new(paths, args)?),
            #[cfg(not(feature = "sqlite"))]
            BackendType::SQLite { .. } => {
                return Err("The SQLite backend is not enabled in this build".into())
            }
            BackendType::Memory => Box::new(MemoryBackend::new(paths, args)?),
        };

        Ok(Self {
            backend: Mutex::new(backend),
        })
    }
}

declare_plugin!(OpenAddressesPlugin);
