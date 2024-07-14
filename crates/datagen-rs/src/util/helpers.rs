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
#[cfg(feature = "serialize")]
use anyhow::Context;
#[cfg(feature = "schema")]
use schemars::schema_for;
#[cfg(feature = "schema")]
use serde_json::Value;
#[cfg(feature = "generate")]
use std::collections::HashMap;
#[cfg(any(feature = "schema", feature = "serialize"))]
use std::fs::File;
#[cfg(any(feature = "schema", feature = "serialize"))]
use std::path::Path;
#[cfg(feature = "generate")]
use std::sync::Arc;

#[cfg(feature = "schema")]
/// Get the JSON schema as a [`Value`].
#[allow(unused)]
pub fn get_schema_value() -> anyhow::Result<Value> {
    let schema = schema_for!(Schema);
    serde_json::to_value(schema).map_err(|e| e.into())
}

#[cfg(feature = "schema")]
/// Write the JSON schema to a file.
/// The schema is generated using the [`schemars`] crate.
/// The schema is used to validate the schema definition.
///
/// # Arguments
/// * `path` - The path to the JSON file.
///
/// # Example
/// ```no_run
/// use datagen_rs::util::helpers::write_json_schema;
///
/// write_json_schema("schema.json").unwrap();
/// ```
pub fn write_json_schema<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::create(path)?;
    let schema = schema_for!(Schema);

    serde_json::to_writer_pretty(file, &schema).map_err(|e| e.into())
}

#[cfg(feature = "serialize")]
/// Read a [`Schema`] from a JSON file.
///
/// # Arguments
/// * `path` - The path to the JSON file.
///
/// # Example
/// ```no_run
/// use datagen_rs::util::helpers::read_schema;
///
/// let schema = read_schema("schema.json").unwrap();
/// ```
pub fn read_schema<P: AsRef<Path>>(path: P) -> anyhow::Result<Schema> {
    let file = File::open(&path).context("Failed to read schema file")?;
    let res = crate::util::json_deserialize::from_reader(file);

    match res {
        Ok(schema) => Ok(schema),
        Err(e) => {
            let deserializer = &mut serde_json::Deserializer::from_reader(File::open(&path)?);
            serde_path_to_error::deserialize(deserializer).context(e)
        }
    }
}

#[cfg(feature = "generate")]
/// Generate random data from a [`Schema`].
///
/// # Arguments
/// * `schema` - The schema to generate data from.
/// * `additional_plugins` - Additional plugins to use when generating data.
///
/// # Example
/// ```
/// use datagen_rs::util::helpers::{generate_random_data, read_schema};
/// use serde_json::{json, from_value};
///
/// let schema_json = json!({
///    "type": "object",
///     "properties": {
///         "name": {
///             "type": "string",
///             "generator": {
///                 "type": "firstName",
///             },
///         },
///     },
/// });
///
/// let schema = from_value(schema_json).unwrap();
/// let data = generate_random_data(schema, None).unwrap();
/// println!("{}", data);
/// ```
pub fn generate_random_data(
    mut schema: Schema,
    additional_plugins: Option<HashMap<String, Arc<dyn Plugin>>>,
) -> anyhow::Result<String> {
    let plugins = PluginList::from_schema(&mut schema, additional_plugins)?;
    let options = Arc::new(schema.options.unwrap_or_default());
    let root = CurrentSchema::root(options.clone(), plugins.clone());
    let generated = schema.value.into_random(root.into())?;

    options
        .serializer
        .as_ref()
        .unwrap_or_default()
        .serialize_generated(generated, Some(plugins))
}
