use crate::json_string;
use crate::runner::node_runner::NodeRunner;
use crate::tests::assert_error_eq;
use datagen_rs::plugins::plugin_list::PluginMap;
use datagen_rs::schema::schema_definition::Schema;
use datagen_rs::util::helpers::generate_random_data;
use once_cell::sync::Lazy;
use serde_json::{json, Value};

type RunnerPlugins = (NodeRunner, PluginMap);

static SCHEMA: Lazy<RunnerPlugins> = Lazy::new(|| {
    napi::__private::register_class("CurrentSchema", None, "CurrentSchema\0", vec![]);
    let mut schema = serde_json::from_value(json!({
        "options": {
            "plugins": {
                "test": {
                    "path": "node:../../packages/test-plugin/dist/TestPlugin.js",
                    "args": {
                        "init": "test",
                    }
                },
                "empty": {
                    "path": "node:../../packages/test-plugin/dist/EmptyPlugin.js",
                    "args": null,
                }
            },
        },
        "type": "string",
        "value": "test",
    }))
    .unwrap();

    let (runner, plugins) = NodeRunner::init(&mut schema).unwrap();
    (runner.unwrap(), plugins)
});

fn get_schema(mut args: Value) -> Schema {
    args.as_object_mut().unwrap().insert(
        "options".into(),
        json!({
            "plugins": {
                "test": {
                    "path": "node:../../packages/test-plugin/dist/TestPlugin.js",
                    "args": {
                        "init": "test",
                    }
                },
                "empty": {
                    "path": "node:../../packages/test-plugin/dist/EmptyPlugin.js",
                    "args": null,
                }
            }
        }),
    );

    serde_json::from_value(args).unwrap()
}

#[test]
fn test_generate() {
    let mut schema = get_schema(json!({
        "type": "plugin",
        "pluginName": "test",
        "args": {
            "foo": "bar"
        }
    }));

    let plugins = SCHEMA.0.load_new_plugins(&mut schema).unwrap();
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
    let mut schema = get_schema(json!({
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

    let plugins = SCHEMA.0.load_new_plugins(&mut schema).unwrap();
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
    let mut schema = serde_json::from_value(json!({
        "options": {
            "plugins": {
                "test": {
                    "path": "node:../../packages/test-plugin/dist/TestPlugin.js",
                    "args": {
                        "init": "test",
                    }
                },
                "empty": {
                    "path": "node:../../packages/test-plugin/dist/EmptyPlugin.js",
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

    let plugins = SCHEMA.0.load_new_plugins(&mut schema).unwrap();
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

#[test]
fn test_generate_not_implemented() {
    let mut schema = get_schema(json!({
        "type": "plugin",
        "pluginName": "empty",
        "args": {
            "foo": "bar"
        }
    }));

    let plugins = SCHEMA.0.load_new_plugins(&mut schema).unwrap();
    let err = generate_random_data(schema, Some(plugins)).unwrap_err();

    assert_error_eq(
        err,
        anyhow::Error::msg("Plugin does not have a 'generate' function")
            .context("Could not receive result from function")
            .context("Failed to call function 'generate' on plugin 'empty'"),
    );
}

#[test]
fn test_transform_not_implemented() {
    let mut schema = get_schema(json!({
        "type": "object",
        "properties": {
            "test": "value"
        },
        "transform": [
            {
                "type": "plugin",
                "name": "empty",
                "args": {
                    "foo": "transform"
                }
            }
        ]
    }));

    let plugins = SCHEMA.0.load_new_plugins(&mut schema).unwrap();
    let err = generate_random_data(schema, Some(plugins)).unwrap_err();

    assert_error_eq(
        err,
        anyhow::Error::msg("Plugin does not have a 'transform' function")
            .context("Could not receive result from function")
            .context("Failed to call function 'transform' on plugin 'empty'"),
    );
}

#[test]
fn test_serialize_not_implemented() {
    let mut schema = serde_json::from_value(json!({
        "options": {
            "plugins": {
                "test": {
                    "path": "node:../../packages/test-plugin/dist/TestPlugin.js",
                    "args": {
                        "init": "test",
                    }
                },
                "empty": {
                    "path": "node:../../packages/test-plugin/dist/EmptyPlugin.js",
                    "args": null,
                }
            },
            "serializer": {
                "type": "plugin",
                "pluginName": "empty",
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

    let plugins = SCHEMA.0.load_new_plugins(&mut schema).unwrap();
    let err = generate_random_data(schema, Some(plugins)).unwrap_err();

    assert_error_eq(
        err,
        anyhow::Error::msg("Plugin does not have a 'serialize' function")
            .context("Could not receive result from function")
            .context("Failed to call function 'serialize' on plugin 'empty'"),
    );
}
