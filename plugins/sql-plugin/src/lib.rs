#[cfg(test)]
mod test;

use anyhow::anyhow;
#[cfg(feature = "plugin-lib")]
use datagen_rs::declare_plugin;
use datagen_rs::generate::generated_schema::GeneratedSchema;
#[cfg(feature = "plugin-lib")]
use datagen_rs::init_plugin_logger;
use datagen_rs::plugins::plugin::PluginOptions;
use datagen_rs::plugins::plugin::{Plugin, PluginConstructor, PluginSerializeCallback};
use indexmap::IndexMap;
use log::debug;
use serde::Deserialize;
use serde_json::Value;
use sqlx::any::AnyPoolOptions;
#[cfg(test)]
use sqlx::AnyPool;
use std::future::Future;
use std::sync::{Arc, Mutex};
#[cfg(not(test))]
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};

static RUNTIME: Mutex<Option<Runtime>> = Mutex::new(None);

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SQLPluginArgs {
    pub url: String,
    pub max_chunk_size: Option<usize>,
    pub max_connections: Option<u32>,
    pub connect_timeout: Option<u64>,
    pub mappings: IndexMap<String, TableMapping>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableMapping {
    pub object_name: String,
    pub column_mappings: IndexMap<String, String>,
}

pub struct SQLPlugin {
    #[cfg(test)]
    pub pool: Option<AnyPool>,
}

pub fn run_sync<T, E, F>(fut: F) -> anyhow::Result<T>
where
    E: Into<anyhow::Error>,
    F: Future<Output = Result<T, E>>,
{
    let mut lock = RUNTIME
        .lock()
        .map_err(|e| anyhow!("Failed to lock runtime mutex: {e}"))?;

    if lock.is_none() {
        lock.replace(Builder::new_current_thread().enable_time().build()?);
    }

    lock.as_ref()
        .ok_or_else(|| anyhow!("Failed to acquire tokio runtime"))?
        .block_on(fut)
        .map_err(Into::into)
}

impl SQLPlugin {
    #[cfg(test)]
    pub fn with_pool() -> anyhow::Result<Self> {
        sqlx::any::install_default_drivers();

        Ok(Self {
            pool: Some(run_sync(
                AnyPoolOptions::new()
                    .max_connections(1)
                    .min_connections(1)
                    .connect("sqlite::file:memdb1?mode=memory&cache=shared"),
            )?),
        })
    }

    fn get_values_from_object(
        obj: &IndexMap<String, Arc<GeneratedSchema>>,
        table_mapping: &TableMapping,
    ) -> anyhow::Result<String> {
        let values = table_mapping
            .column_mappings
            .values()
            .map(|value| {
                let value = obj
                    .get(value)
                    .ok_or_else(|| anyhow::anyhow!("Column '{value}' not found"))?;

                if matches!(value.as_ref(), GeneratedSchema::String(_)) {
                    Ok(format!("'{}'", value.to_string().replace('\'', "''")))
                } else {
                    Ok(value.to_string())
                }
            })
            .collect::<anyhow::Result<Vec<_>>>()?
            .join(", ");

        Ok(format!("({values})"))
    }

    fn get_values(
        value: &Arc<GeneratedSchema>,
        table_mapping: &TableMapping,
    ) -> anyhow::Result<Vec<String>> {
        match value.as_ref() {
            GeneratedSchema::Object(obj) => {
                Self::get_values_from_object(obj, table_mapping).map(|v| vec![v])
            }
            GeneratedSchema::Array(arr) => Ok(arr
                .iter()
                .map(|value| match value.as_ref() {
                    GeneratedSchema::Object(obj) => {
                        Self::get_values_from_object(obj, table_mapping)
                    }
                    item => Err(anyhow::anyhow!(
                        "Item in array schema is not an object. Actual type: {}",
                        item.name()
                    )),
                })
                .collect::<anyhow::Result<Vec<_>>>()?),
            item => Err(anyhow::anyhow!(
                "Generated schema is not an object. Actual type: {}",
                item.name()
            )),
        }
    }

    fn get_values_from_root(
        value: &Arc<GeneratedSchema>,
        table_mapping: &TableMapping,
        key: &str,
    ) -> anyhow::Result<Vec<String>> {
        match value.as_ref() {
            GeneratedSchema::Object(obj) => obj
                .get(key)
                .ok_or_else(|| anyhow::anyhow!("Key '{key}' not found"))
                .and_then(|value| Self::get_values(value, table_mapping)),
            item => Err(anyhow::anyhow!(
                "Root schema is not an object. Actual type: {}",
                item.name()
            )),
        }
    }

    fn count_values(
        value: &Arc<GeneratedSchema>,
        mappings: &IndexMap<String, TableMapping>,
    ) -> anyhow::Result<usize> {
        let obj = match value.as_ref() {
            GeneratedSchema::Object(obj) => obj,
            item => {
                return Err(anyhow::anyhow!(
                    "Root schema is not an object. Actual type: {}",
                    item.name()
                ))
            }
        };

        let mut count = 0;
        for table_name in mappings.keys() {
            let value = obj
                .get(table_name)
                .ok_or_else(|| anyhow::anyhow!("Table '{table_name}' not found in root schema"))?;

            match value.as_ref() {
                GeneratedSchema::Array(arr) => count += arr.len(),
                GeneratedSchema::Object(_) => count += 1,
                item => anyhow::bail!(
                    "Generated schema is not an object. Actual type: {}",
                    item.name()
                ),
            }
        }

        Ok(count)
    }

    async fn insert_into_db(
        &self,
        args: &SQLPluginArgs,
        value: &Arc<GeneratedSchema>,
        callback: &PluginSerializeCallback,
    ) -> anyhow::Result<()> {
        debug!("Connecting to database: {}", args.url);

        #[cfg(not(test))]
        let pool = AnyPoolOptions::new()
            .max_connections(args.max_connections.unwrap_or(5))
            .acquire_timeout(Duration::from_secs(args.connect_timeout.unwrap_or(10)))
            .connect(args.url.as_str())
            .await?;

        #[cfg(test)]
        let pool = self.pool.clone().unwrap();

        let chunk_size = args.max_chunk_size.unwrap_or(100);
        anyhow::ensure!(chunk_size > 0, "Chunk size must be greater than 0");

        let count = Self::count_values(value, &args.mappings)?;
        debug!("Inserting data into database");

        let mut i = 0;
        for (table_name, mapping) in args.mappings.iter() {
            let keys = mapping
                .column_mappings
                .keys()
                .cloned()
                .collect::<Vec<_>>()
                .join(", ");

            let values = Self::get_values_from_root(value, mapping, &mapping.object_name)?;
            for chunk in values.chunks(chunk_size) {
                let statement = format!(
                    "INSERT INTO {table_name} ({keys}) VALUES {}",
                    chunk.join(", ")
                );

                debug!("Executing statement: {statement}");
                sqlx::query(&statement).execute(&pool).await?;

                i += chunk_size;
                callback(i, count)?;
            }
        }

        debug!("Done");
        Ok(())
    }
}

impl Plugin for SQLPlugin {
    fn name(&self) -> String {
        "sql".into()
    }

    fn serialize(&self, value: &Arc<GeneratedSchema>, args: Value) -> anyhow::Result<String> {
        self.serialize_with_progress(value, args, Box::new(|_, _| Ok(())))
    }

    fn serialize_with_progress(
        &self,
        value: &Arc<GeneratedSchema>,
        args: Value,
        callback: PluginSerializeCallback,
    ) -> anyhow::Result<String> {
        let args: SQLPluginArgs = serde_json::from_value(args)?;

        debug!("Initializing default SQL drivers");
        sqlx::any::install_default_drivers();

        run_sync(self.insert_into_db(&args, value, &callback))?;
        Ok("".into())
    }
}

impl PluginConstructor for SQLPlugin {
    fn new(
        _args: Value,
        #[cfg(feature = "plugin-lib")] options: PluginOptions,
        #[cfg(not(feature = "plugin-lib"))] _options: PluginOptions,
    ) -> anyhow::Result<Self> {
        #[cfg(feature = "plugin-lib")]
        init_plugin_logger!(options);

        debug!("Initializing default SQL drivers");
        sqlx::any::install_default_drivers();

        Ok(Self {
            #[cfg(test)]
            pool: None,
        })
    }
}

#[cfg(feature = "plugin-lib")]
declare_plugin!(SQLPlugin);
