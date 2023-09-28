mod auth;
mod objects;

use crate::objects::upload_args::UploadArgs;
use datagen_rs::declare_plugin;
use datagen_rs::generate::generated_schema::GeneratedSchema;
use datagen_rs::plugins::plugin::Plugin;
use datagen_rs::util::types::Result;
use serde_json::{from_value, Value};
use std::sync::Arc;

#[derive(Debug, Default)]
struct UploadPlugin;

impl Plugin for UploadPlugin {
    fn name(&self) -> String {
        "upload-plugin".into()
    }

    fn serialize(&self, value: &Arc<GeneratedSchema>, args: Value) -> Result<String> {
        let args: UploadArgs = from_value(args).map_err(|e| e.to_string())?;
        args.upload_data(value)?;

        if args.return_null.unwrap_or_default() {
            Ok("".into())
        } else {
            args.serialize_generated(value)
        }
    }
}

declare_plugin!(UploadPlugin, UploadPlugin::default);
