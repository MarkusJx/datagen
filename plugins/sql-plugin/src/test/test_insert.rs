use crate::{run_sync, SQLPlugin};
use datagen_rs::generate::current_schema::CurrentSchema;
use datagen_rs::generate::generated_schema::IntoRandom;
use datagen_rs::plugins::plugin::Plugin;
use datagen_rs::plugins::plugin_list::PluginList;
use datagen_rs::util::helpers::generate_random_data;
use serde_json::{json, Value};
use sqlx::Row;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

fn items() -> Value {
    json!({
        "type": "array",
        "length": {
            "value": 10
        },
        "items": {
            "type": "object",
            "properties": {
                "id": {
                    "type": "integer"
                },
                "name": {
                    "type": "string",
                    "generator": {
                        "type": "fullName"
                    }
                },
                "email": {
                    "type": "string",
                    "generator": {
                        "type": "email"
                    }
                }
            }
        }
    })
}

fn generate_data(schema: Value, plugin: Arc<SQLPlugin>) {
    let generated = generate_random_data(
        serde_json::from_value(schema).unwrap(),
        Some(
            vec![("sql".into(), plugin.clone() as Arc<dyn Plugin>)]
                .into_iter()
                .collect(),
        ),
    )
    .unwrap();
    assert_eq!("", generated);

    let count = run_sync(
        sqlx::query("SELECT COUNT(*) FROM users").fetch_one(plugin.as_ref().pool.as_ref().unwrap()),
    )
    .unwrap()
    .get::<i64, _>(0);

    assert_eq!(10, count);
}

#[test]
fn test_insert() {
    let plugin = Arc::new(SQLPlugin::with_pool().unwrap());

    run_sync(
        sqlx::query("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)")
            .execute(plugin.as_ref().pool.as_ref().unwrap()),
    )
    .unwrap();

    let schema = json!({
        "options": {
            "serializer": {
                "type": "plugin",
                "pluginName": "sql",
                "args": {
                    "url": "sqlite::memory:",
                    "mappings": {
                        "users": {
                            "objectName": "users",
                            "columnMappings": {
                                "id": "id",
                                "name": "name",
                                "email": "email"
                            }
                        }
                    }
                }
            }
        },
        "type": "object",
        "properties": {
            "users": items()
        }
    });

    generate_data(schema, plugin.clone());
}

#[test]
fn test_insert_multiple() {
    let plugin = Arc::new(SQLPlugin::with_pool().unwrap());

    run_sync(
        sqlx::query("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)")
            .execute(plugin.as_ref().pool.as_ref().unwrap()),
    )
    .unwrap();
    run_sync(
        sqlx::query("CREATE TABLE companies (id INTEGER PRIMARY KEY, name TEXT, email TEXT)")
            .execute(plugin.as_ref().pool.as_ref().unwrap()),
    )
    .unwrap();

    let schema = json!({
        "options": {
            "serializer": {
                "type": "plugin",
                "pluginName": "sql",
                "args": {
                    "url": "sqlite::memory:",
                    "mappings": {
                        "users": {
                            "objectName": "users",
                            "columnMappings": {
                                "id": "id",
                                "name": "name",
                                "email": "email"
                            }
                        },
                        "companies": {
                            "objectName": "companies",
                            "columnMappings": {
                                "id": "id",
                                "name": "name",
                                "email": "email"
                            }
                        }
                    }
                }
            }
        },
        "type": "object",
        "properties": {
            "users": items(),
            "companies": items()
        }
    });

    generate_data(schema, plugin.clone());

    let count = run_sync(
        sqlx::query("SELECT COUNT(*) FROM companies")
            .fetch_one(plugin.as_ref().pool.as_ref().unwrap()),
    )
    .unwrap()
    .get::<i64, _>(0);

    assert_eq!(10, count);
}

#[test]
fn test_serialize_with_progress() {
    let plugin = Arc::new(SQLPlugin::with_pool().unwrap());

    run_sync(
        sqlx::query("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)")
            .execute(plugin.as_ref().pool.as_ref().unwrap()),
    )
    .unwrap();
    run_sync(
        sqlx::query("CREATE TABLE companies (id INTEGER PRIMARY KEY, name TEXT, email TEXT)")
            .execute(plugin.as_ref().pool.as_ref().unwrap()),
    )
    .unwrap();

    let schema = json!({
        "options": {
            "serializer": {
                "type": "plugin",
                "pluginName": "sql",
                "args": {
                    "url": "sqlite::memory:",
                    "maxChunkSize": 1,
                    "mappings": {
                        "users": {
                            "objectName": "users",
                            "columnMappings": {
                                "id": "id",
                                "name": "name",
                                "email": "email"
                            }
                        },
                        "companies": {
                            "objectName": "companies",
                            "columnMappings": {
                                "id": "id",
                                "name": "name",
                                "email": "email"
                            }
                        }
                    }
                }
            }
        },
        "type": "object",
        "properties": {
            "users": items(),
            "companies": items()
        }
    });

    let mut schema = serde_json::from_value(schema).unwrap();
    let plugins = PluginList::from_schema(
        &mut schema,
        Some(
            vec![("sql".into(), plugin.clone() as Arc<dyn Plugin>)]
                .into_iter()
                .collect(),
        ),
    )
    .unwrap();
    let options = Arc::new(schema.options.unwrap_or_default());
    let root = CurrentSchema::root(options.clone(), plugins.clone());
    let generated = schema.value.into_random(root.into()).unwrap();

    let count = Arc::new(AtomicI32::new(0));
    let count_clone = count.clone();

    options
        .serializer
        .as_ref()
        .unwrap_or_default()
        .serialize_generated_with_progress(
            generated,
            Some(plugins),
            Box::new(move |current, total| {
                count.fetch_add(1, Ordering::Relaxed);
                assert_eq!(count.load(Ordering::Relaxed), current as i32);
                assert_eq!(20, total);

                Ok(())
            }),
        )
        .unwrap();

    assert_eq!(20, count_clone.load(Ordering::Relaxed));
}
