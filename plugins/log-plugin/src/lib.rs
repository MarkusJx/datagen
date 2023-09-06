use datagen_rs::declare_plugin;
use datagen_rs::generate::current_schema::CurrentSchema;
use datagen_rs::generate::generated_schema::GeneratedSchema;
use datagen_rs::plugins::plugin::{Plugin, PluginConstructor};
use datagen_rs::util::types::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct LogPlugin;

#[derive(Debug, Serialize, Deserialize)]
struct Args {
    pub test: String,
}

impl Plugin for LogPlugin {
    fn name(&self) -> &'static str {
        println!("LogPlugin");
        "log"
    }

    fn generate(&self, schema: Arc<CurrentSchema>, args: Value) -> Result<Arc<GeneratedSchema>> {
        println!("args: {:?}", args);
        let args: Args = serde_json::from_value(args)?;
        println!("generate called with {:?} and args {:?}", schema, args);
        Ok(Arc::new(GeneratedSchema::String("logged value".into())))
    }

    fn transform(
        &self,
        _: Arc<CurrentSchema>,
        value: Arc<GeneratedSchema>,
        args: Value,
    ) -> Result<Arc<GeneratedSchema>> {
        println!("{:?}, args {:?}", value, args);
        if let GeneratedSchema::String(value) = value.as_ref() {
            Ok(Arc::new(GeneratedSchema::String(
                value.to_ascii_uppercase(),
            )))
        } else {
            Ok(value)
        }
    }
}

impl PluginConstructor for LogPlugin {
    fn new(args: Box<Value>) -> Result<Self> {
        println!("init");
        println!("new called with {:?}", args);
        Ok(Self::default())
    }
}

impl Drop for LogPlugin {
    fn drop(&mut self) {
        println!("drop called");
    }
}

declare_plugin!(LogPlugin);
