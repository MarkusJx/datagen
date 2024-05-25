mod auth;
mod objects;
#[cfg(test)]
mod tests;

use crate::objects::upload_args::UploadArgs;
#[cfg(feature = "plugin-lib")]
use datagen_rs::declare_plugin;
use datagen_rs::generate::generated_schema::GeneratedSchema;
#[cfg(feature = "plugin-lib")]
use datagen_rs::init_plugin_logger;
use datagen_rs::plugins::plugin::{
    Plugin, PluginConstructor, PluginOptions, PluginSerializeCallback,
};
use serde_json::{from_value, Value};
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct UploadPlugin;

impl Plugin for UploadPlugin {
    fn name(&self) -> String {
        "upload-plugin".into()
    }

    fn serialize(&self, value: &Arc<GeneratedSchema>, args: Value) -> anyhow::Result<String> {
        self.serialize_with_progress(value, args, Box::new(|_current, _total| Ok(())))
    }

    fn serialize_with_progress(
        &self,
        value: &Arc<GeneratedSchema>,
        args: Value,
        callback: PluginSerializeCallback,
    ) -> anyhow::Result<String> {
        let args: UploadArgs = from_value(args).map_err(anyhow::Error::new)?;
        args.upload_data(value, callback)?;

        if args.return_null.unwrap_or_default() {
            Ok("".into())
        } else {
            args.serialize_generated(value)
        }
    }
}

impl PluginConstructor for UploadPlugin {
    fn new(
        _args: Value,
        #[cfg(feature = "plugin-lib")] options: PluginOptions,
        #[cfg(not(feature = "plugin-lib"))] _options: PluginOptions,
    ) -> anyhow::Result<Self> {
        #[cfg(feature = "plugin-lib")]
        init_plugin_logger!(options);

        Ok(Self)
    }
}

#[cfg(feature = "plugin-lib")]
declare_plugin!(UploadPlugin);
