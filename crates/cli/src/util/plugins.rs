use datagen_rs::plugins::plugin::{Plugin, PluginConstructor, PluginOptions};
use datagen_rs::schema::schema_definition::Schema;
use openaddresses_plugin::OpenAddressesPlugin;
use serde_json::Value;
use sql_plugin::SQLPlugin;
use std::collections::HashMap;
use std::sync::Arc;
use upload_plugin::UploadPlugin;

pub fn load_plugins(schema: &Schema) -> anyhow::Result<HashMap<String, Arc<dyn Plugin>>> {
    let mut result = HashMap::new();

    if let Some(options) = schema.options.as_ref() {
        if let Some(plugins) = options.plugins.as_ref() {
            if let Some(openaddresses_plugin) = plugins.get("openaddresses-plugin") {
                let plugin = OpenAddressesPlugin::new(
                    serde_json::to_value(openaddresses_plugin.clone())?,
                    PluginOptions::default(),
                )?;
                result.insert("openaddresses-plugin".into(), Arc::new(plugin) as Arc<_>);
            }
        }
    }

    result.insert("upload-plugin".into(), Arc::new(UploadPlugin) as Arc<_>);
    result.insert(
        "sql-plugin".into(),
        Arc::new(SQLPlugin::new(Value::Null, PluginOptions::default())?) as Arc<_>,
    );

    Ok(result)
}
