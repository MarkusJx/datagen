#[cfg(feature = "generate")]
use crate::generate::current_schema::CurrentSchema;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::IntoRandom;
#[cfg(feature = "generate")]
use crate::plugins::plugin::Plugin;
#[cfg(feature = "generate")]
use crate::plugins::plugin_list::PluginList;
#[cfg(any(feature = "schema", any(feature = "serialize", feature = "generate")))]
use crate::schema::schema_definition::Schema;
#[cfg(feature = "generate")]
use crate::schema::schema_definition::Serializer;
#[cfg(any(feature = "schema", feature = "serialize"))]
use crate::util::types::Result;
#[cfg(feature = "schema")]
use schemars::schema_for;
#[cfg(feature = "schema")]
use serde_json::Value;
#[cfg(feature = "generate")]
use std::collections::HashMap;
#[cfg(any(feature = "schema", feature = "serialize"))]
use std::fs::File;
#[cfg(feature = "generate")]
use std::io::Read;
#[cfg(any(feature = "schema", feature = "serialize"))]
use std::path::Path;
#[cfg(feature = "generate")]
use std::sync::Arc;
#[cfg(feature = "generate")]
use xml::{EmitterConfig, ParserConfig};

#[cfg(feature = "schema")]
#[allow(unused)]
pub fn get_schema_value() -> Result<Value> {
    let schema = schema_for!(Schema);
    serde_json::to_value(schema).map_err(|e| e.into())
}

#[cfg(feature = "schema")]
pub fn write_json_schema<P: AsRef<Path>>(path: P) -> Result<()> {
    let file = File::create(path)?;
    let schema = schema_for!(Schema);

    serde_json::to_writer_pretty(file, &schema).map_err(|e| e.into())
}

#[cfg(feature = "serialize")]
pub fn read_schema<P: AsRef<Path>>(path: P) -> Result<Schema> {
    let file = File::open(path)?;
    let schema: Schema = serde_json::from_reader(file)?;

    Ok(schema)
}

#[cfg(feature = "generate")]
fn format_xml<R: Read>(src: R) -> Result<String> {
    let mut dest = Vec::new();
    let reader = ParserConfig::new()
        .trim_whitespace(true)
        .ignore_comments(false)
        .create_reader(src);
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .normalize_empty_elements(false)
        .autopad_comments(false)
        .create_writer(&mut dest);

    for event in reader {
        if let Some(event) = event?.as_writer_event() {
            writer.write(event)?;
        }
    }

    String::from_utf8(dest).map_err(Into::into)
}

#[cfg(feature = "generate")]
pub fn generate_random_data(
    schema: Schema,
    additional_plugins: Option<HashMap<String, Box<dyn Plugin>>>,
) -> Result<String> {
    let plugins = PluginList::from_schema(&schema, additional_plugins)?;
    let options = Arc::new(schema.options.unwrap_or_default());
    let root = CurrentSchema::root(options.clone(), plugins.clone());
    let generated = schema.value.into_random(root)?;

    match options.serializer.as_ref().unwrap_or_default() {
        Serializer::Json { pretty } => pretty
            .unwrap_or(false)
            .then(|| serde_json::to_string_pretty(&generated))
            .unwrap_or_else(|| serde_json::to_string(&generated))
            .map_err(Into::into),
        Serializer::Yaml => serde_yaml::to_string(&generated).map_err(Into::into),
        Serializer::Xml {
            root_element,
            pretty,
        } => {
            let res = quick_xml::se::to_string_with_root(root_element, &generated)
                .map_err(|e| e.to_string())?;

            if pretty.unwrap_or(false) {
                format_xml(res.as_bytes())
            } else {
                Ok(res)
            }
        }
        Serializer::Plugin { plugin_name, args } => plugins
            .get(plugin_name)?
            .serialize(&generated, args.clone().unwrap_or_default()),
    }
}
