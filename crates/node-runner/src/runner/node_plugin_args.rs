use crate::runner::types::RefArc;
use anyhow::{anyhow, Context};
use datagen_rs::schema::schema_definition::PluginInitArgs;
use napi::{Env, JsFunction, ValueType};
use serde_json::Value;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct NodePluginArgs {
    pub name: String,
    pub path: String,
    pub args: Value,
}

impl NodePluginArgs {
    pub fn from_args(name: &str, args: &PluginInitArgs) -> Self {
        match args.clone() {
            PluginInitArgs::Args { path, args } => NodePluginArgs {
                name: name.to_string(),
                path: Self::normalize_path(&path),
                args: args.unwrap_or_default(),
            },
            PluginInitArgs::Value(value) => NodePluginArgs {
                name: name.to_string(),
                path: Self::normalize_path(name),
                args: value,
            },
        }
    }

    pub fn import_plugin(&self, env: Env, require: &JsFunction) -> anyhow::Result<RefArc> {
        let imported = require
            .call(None, &[env.create_string(&self.path)?])
            .context(anyhow!(
                "Failed to import plugin '{}' at '{}'",
                self.name,
                self.path
            ))?;

        let import_res = if imported.get_type()? == ValueType::Function {
            unsafe { imported.cast::<JsFunction>() }
                .call(None, &[env.to_js_value(&self.args)?])
                .context(anyhow!(
                    "Failed to call init function of plugin '{}'",
                    self.name
                ))?
        } else {
            let imported = imported.coerce_to_object()?;
            if imported.has_property("default")? {
                imported
                    .get_named_property::<JsFunction>("default")
                    .context(anyhow!(
                        "Failed to get property 'default' of plugin '{}'",
                        self.name
                    ))?
                    .call(None, &[env.to_js_value(&self.args)?])
                    .context(anyhow!(
                        "Failed to call init function of plugin '{}'",
                        self.name
                    ))?
            } else {
                return Err(anyhow!(
                    "Plugin '{}' at '{}' does not have a default export",
                    self.name,
                    self.path
                ));
            }
        }
        .coerce_to_object()?;

        Ok(Arc::new(Mutex::new(env.create_reference(import_res)?)))
    }

    fn normalize_path(path: &str) -> String {
        path.strip_prefix("node:").unwrap_or(path).to_string()
    }
}
