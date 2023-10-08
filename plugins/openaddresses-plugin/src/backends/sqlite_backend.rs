use crate::backends::backend::{Backend, BackendConstructor};
use crate::objects::args::{BackendType, PluginArgs};
use crate::objects::geo_data::GeoFeature;
use crate::SQLITE_MAX_VARIABLE_NUMBER;
use anyhow::anyhow;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use rusqlite::types::Type;
use rusqlite::{params_from_iter, Connection};
use serde_json::Value;
#[cfg(test)]
use std::any::Any;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub(crate) struct SQLiteBackend {
    #[doc(hidden)]
    pub(crate) db: Connection,
    data_cache: HashMap<String, Vec<GeoFeature>>,
    num_cached: u32,
}

unsafe impl Send for SQLiteBackend {}
unsafe impl Sync for SQLiteBackend {}

impl SQLiteBackend {
    #[doc(hidden)]
    pub(crate) fn table_exists(db: &Connection, table_name: &String) -> bool {
        db.query_row(
            "SELECT name FROM sqlite_master WHERE type='table' AND name=?1",
            [&table_name],
            |_| Ok(()),
        )
        .is_ok()
    }

    fn create_table(db: &Connection, table_name: &String) -> anyhow::Result<()> {
        #[cfg(feature = "log")]
        log::debug!("Creating table '{table_name}'");

        db.execute(
            &format!(
                "CREATE TABLE {table_name} (id INTEGER PRIMARY KEY AUTOINCREMENT, feature JSON)"
            ),
            [],
        )?;

        Ok(())
    }

    fn insert_multiple_values(
        db: &Connection,
        table_name: &String,
        buf: &mut Vec<Value>,
    ) -> anyhow::Result<()> {
        if buf.is_empty() {
            return Ok(());
        }

        #[cfg(feature = "log")]
        log::debug!("Inserting {} values into table '{table_name}'", buf.len());

        let values = buf.iter().map(|_| "(?)").collect::<Vec<_>>().join(", ");
        db.execute(
            &format!("INSERT INTO {table_name} (feature) VALUES {values}"),
            params_from_iter(buf.iter()),
        )?;
        buf.clear();

        Ok(())
    }

    fn str_to_feature(str: &str) -> anyhow::Result<Value> {
        serde_json::to_value(serde_json::from_str::<GeoFeature>(str)?).map_err(Into::into)
    }

    fn fill_cache(&mut self, table_name: &String) -> anyhow::Result<&mut Vec<GeoFeature>> {
        let mut stmt = self.db.prepare(&format!(
            "select feature from {table_name} order by random() limit ?1"
        ))?;

        let data = self.data_cache.get_mut(table_name).unwrap();
        data.extend(
            stmt.query_map([self.num_cached], |row| {
                serde_json::from_value::<GeoFeature>(row.get::<usize, Value>(0)?)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, Type::Text, e.into()))
            })?
            .map(|e| e.map_err(Into::into))
            .collect::<anyhow::Result<Vec<_>>>()?,
        );

        #[cfg(feature = "log")]
        log::debug!(
            "Re-filled cache for table '{table_name}' with {} items",
            data.len()
        );

        Ok(data)
    }
}

impl Backend for SQLiteBackend {
    fn get_random_feature(&mut self) -> anyhow::Result<GeoFeature> {
        let table_name = {
            let (table_name, data) = self
                .data_cache
                .iter_mut()
                .choose(&mut thread_rng())
                .ok_or(anyhow!("The data cache is empty"))?;

            if let Some(feature) = data.pop() {
                return Ok(feature);
            }

            table_name.clone()
        };

        let data = self.fill_cache(&table_name)?;
        data.pop().ok_or(anyhow!("Failed to find data"))
    }

    #[cfg(test)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[cfg(test)]
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl BackendConstructor for SQLiteBackend {
    fn new(paths: Vec<String>, args: PluginArgs) -> anyhow::Result<Self> {
        let BackendType::SQLite {
            database_name,
            batch_size,
            cache_size,
        } = args.backend.unwrap_or_default()
        else {
            return Err(anyhow!(
                "Unable to create SQLite backend: The selected backend type is not SQLite"
            ));
        };

        #[cfg(feature = "log")]
        log::debug!("Initializing SQLite backend");

        let db = Connection::open(database_name)?;
        let num_rows = batch_size.unwrap_or(SQLITE_MAX_VARIABLE_NUMBER);
        if num_rows > SQLITE_MAX_VARIABLE_NUMBER {
            return Err(anyhow!(
                "batchSize cannot be greater than {}",
                SQLITE_MAX_VARIABLE_NUMBER
            )
            .into());
        }

        #[cfg(feature = "log")]
        log::debug!("Using batch size of {}", num_rows);

        let mut buf = Vec::with_capacity(num_rows);
        let mut data_cache = HashMap::new();

        for path in &paths {
            let table_name = path.replace(['-', '_', '.', '/', '\\'], "");
            data_cache.insert(table_name.clone(), Vec::new());
            if Self::table_exists(&db, &table_name) {
                #[cfg(feature = "log")]
                log::debug!("Table '{table_name}' already exists, skipping creation");
                continue;
            }

            Self::create_table(&db, &table_name)?;

            let reader = BufReader::new(File::open(path)?);
            for (i, line) in reader.lines().enumerate() {
                if i > 1 && i % num_rows == 0 {
                    Self::insert_multiple_values(&db, &table_name, &mut buf)?;
                }

                buf.push(Self::str_to_feature(&line?)?);
            }

            Self::insert_multiple_values(&db, &table_name, &mut buf)?;
        }

        Ok(Self {
            db,
            num_cached: cache_size.unwrap_or(1000),
            data_cache,
        })
    }
}
