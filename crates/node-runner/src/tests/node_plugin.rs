use crate::json_string;
use crate::runner::node_runner::NodeRunner;
use datagen_rs::plugins::plugin::Plugin;
use datagen_rs::schema::schema_definition::Schema;
use datagen_rs::util::helpers::generate_random_data;
use once_cell::sync::Lazy;
use serde_json::{json, Value};
use std::collections::HashMap;

static SCHEMA: Lazy<(NodeRunner, HashMap<String, Box<dyn Plugin>>)> = Lazy::new(|| {
    napi::__private::register_class("CurrentSchema", None, "CurrentSchema\0", vec![]);
    napi::__private::register_class("NodePluginLoader", None, "NodePluginLoader\0", vec![]);
    let schema = serde_json::from_value(json!({
        "options": {
            "plugins": {
                "test": {
                    "path": "node:../../../packages/test-plugin/dist/TestPlugin.js",
                    "args": {
                        "init": "test",
                    }
                },
                "empty": {
                    "path": "node:../../../packages/test-plugin/dist/EmptyPlugin.js",
                    "args": null,
                }
            },
        },
        "type": "string",
        "value": "test",
    }))
    .unwrap();

    NodeRunner::init(&schema).unwrap()
});

fn get_schema(mut args: Value) -> Schema {
    args.as_object_mut().unwrap().insert(
        "options".into(),
        json!({
            "plugins": {
                "test": {
                    "path": "node:../../../packages/test-plugin/dist/TestPlugin.js",
                    "args": {
                        "init": "test",
                    }
                },
                "empty": {
                    "path": "node:../../../packages/test-plugin/dist/EmptyPlugin.js",
                    "args": null,
                }
            }
        }),
    );

    serde_json::from_value(args).unwrap()
}

#[test]
fn test_generate() {
    let schema = get_schema(json!({
        "type": "plugin",
        "pluginName": "test",
        "args": {
            "foo": "bar"
        }
    }));

    let plugins = SCHEMA.0.load_new_plugins(&schema).unwrap();
    let generated = generate_random_data(schema, Some(plugins)).unwrap();

    assert_eq!(
        generated,
        json_string!({
            "foo": "bar",
            "init": "test",
        })
    );
}

#[test]
fn test_transform() {
    let schema = get_schema(json!({
        "type": "object",
        "properties": {
            "test": "value"
        },
        "transform": [
            {
                "type": "plugin",
                "name": "test",
                "args": {
                    "foo": "transform"
                }
            }
        ]
    }));

    let plugins = SCHEMA.0.load_new_plugins(&schema).unwrap();
    let generated = generate_random_data(schema, Some(plugins)).unwrap();

    assert_eq!(
        generated,
        json_string!({
            "foo": "transform",
            "init": "test",
            "test": "value"
        })
    );
}

#[test]
fn test_serialize() {
    let schema = serde_json::from_value(json!({
        "options": {
            "plugins": {
                "test": {
                    "path": "node:../../../packages/test-plugin/dist/TestPlugin.js",
                    "args": {
                        "init": "test",
                    }
                },
                "empty": {
                    "path": "node:../../../packages/test-plugin/dist/EmptyPlugin.js",
                    "args": null,
                }
            },
            "serializer": {
                "type": "plugin",
                "pluginName": "test",
                "args": {
                    "foo": "serialize"
                }
            }
        },
        "type": "object",
        "properties": {
            "test": "value"
        },
    }))
    .unwrap();

    let plugins = SCHEMA.0.load_new_plugins(&schema).unwrap();
    let generated = generate_random_data(schema, Some(plugins)).unwrap();

    assert_eq!(
        generated,
        json_string!({
            "foo": "serialize",
            "init": "test",
            "test": "value"
        })
    );
}
