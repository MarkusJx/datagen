use anyhow::Context;
use datagen_rs::plugins::plugin::{PluginConstructor, PluginOptions};
use datagen_rs::plugins::plugin_list::PluginMap;
use datagen_rs::schema::schema_definition::{PluginInitArgs, Schema};
use log::{debug, warn};
use openaddresses_plugin::OpenAddressesPlugin;
use serde_json::Value;
use sql_plugin::SQLPlugin;
use std::sync::Arc;
use upload_plugin::UploadPlugin;

#[derive(Default)]
struct PluginLoader(PluginMap);

impl PluginLoader {
    fn add_without_args<P: PluginConstructor + 'static>(
        mut self,
        name: &str,
    ) -> anyhow::Result<Self> {
        debug!("Loading plugin '{}'", name);
        self.0.insert(
            name.into(),
            Arc::new(
                P::new(Value::Null, PluginOptions::default())
                    .context("Failed to load bundled plugin without arguments")?,
            ) as Arc<_>,
        );

        Ok(self)
    }

    fn add_with_args<P: PluginConstructor + 'static>(
        mut self,
        name: &str,
        schema: &Schema,
    ) -> anyhow::Result<Self> {
        if let Some(options) = schema.options.as_ref() {
            if let Some(plugins) = options.plugins.as_ref() {
                if let Some(plugin) = plugins.get(name) {
                    let args = match plugin {
                        PluginInitArgs::Args { args, .. } => {
                            warn!("Using args with plugin path for the '{name}' plugin when the plugin is bundled in the binary. The plugin path will be ignored.");
                            args.clone().unwrap_or_default()
                        }
                        PluginInitArgs::Value(args) => args.clone(),
                    };

                    debug!("Loading plugin '{name}' with args: {}", args);
                    let plugin = P::new(args, PluginOptions::default())
                        .context("Failed to load bundled plugin with arguments")?;
                    self.0.insert(name.into(), Arc::new(plugin) as Arc<_>);
                }
            }
        }

        Ok(self)
    }

    fn build(self) -> anyhow::Result<PluginMap> {
        Ok(self.0)
    }
}

pub fn load_plugins(schema: &Schema) -> anyhow::Result<PluginMap> {
    PluginLoader::default()
        .add_without_args::<UploadPlugin>("upload-plugin")?
        .add_without_args::<SQLPlugin>("sql-plugin")?
        .add_with_args::<OpenAddressesPlugin>("openaddresses-plugin", schema)?
        .build()
}
