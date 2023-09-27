use crate::backends::sqlite_backend::SQLiteBackend;
use crate::tests::{generate_random, get_by_hash, TestAddress, ADDR_FILE_NAME};
use crate::OpenAddressesPlugin;
use datagen_rs::plugins::plugin::PluginConstructor;
use datagen_rs::util::types::Result;
use rand::random;
use serde_json::json;
use serial_test::serial;
use std::env;
use std::fs::remove_file;
use std::string::ToString;

#[derive(Debug)]
struct Cleanup {
    id: u32,
}

impl Cleanup {
    fn new(id: u32) -> Self {
        Self { id }
    }
}

impl Drop for Cleanup {
    fn drop(&mut self) {
        let path = env::current_dir()
            .unwrap()
            .join(format!("addresses-{}.db", self.id));
        if path.exists() {
            remove_file(path).unwrap();
        }
    }
}

fn create_database(file: Option<&str>) -> Result<(Cleanup, OpenAddressesPlugin)> {
    let id: u32 = random();

    let cleanup = Cleanup::new(id);
    OpenAddressesPlugin::new(json!({
        "files": file.unwrap_or(ADDR_FILE_NAME),
        "backend": {
            "type": "sqlite",
            "databaseName": format!("addresses-{}.db", id),
            "batchSize": 1000,
            "cacheSize": 1000
        }
    }))
    .map(|plugin| (cleanup, plugin))
}

const TABLE_NAME: &str = "srctestsaddressesgeojson";

#[test]
#[serial]
fn test_create_database() {
    let (_c, plugin) = create_database(None).unwrap();

    let backend_lock = plugin.backend.lock().unwrap();
    let backend = backend_lock
        .as_ref()
        .as_any()
        .downcast_ref::<SQLiteBackend>()
        .unwrap();

    assert!(
        SQLiteBackend::table_exists(&backend.db, &TABLE_NAME.to_string()),
        "Expected table '{}' to exist",
        TABLE_NAME
    );

    let count: i32 = backend
        .db
        .query_row(
            &format!("select count(*) from {}", TABLE_NAME),
            [],
            |count| count.get(0),
        )
        .unwrap();

    assert_eq!(count, 10);
}

#[test]
#[serial]
fn test_generate() {
    let (_c, plugin) = create_database(None).unwrap();

    let generated = generate_random(&plugin);
    assert_eq!(generated, get_by_hash(&generated.hash));
}

#[test]
#[serial]
fn test_get_random() {
    let (_c, plugin) = create_database(None).unwrap();

    let feature: TestAddress = plugin
        .backend
        .lock()
        .unwrap()
        .get_random_feature()
        .unwrap()
        .into();
    assert_eq!(feature, get_by_hash(&feature.hash));
}

#[test]
#[serial]
fn get_invalid_file() {
    let result = create_database(Some("invalid.geojson"));
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("The system cannot find the file specified.") || err.contains("No such file or directory"),
        "Expected error to contain 'The system cannot find the file specified.' or 'No such file or directory', but got: {}",
        err
    );
}
