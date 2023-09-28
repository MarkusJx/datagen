use crate::UploadPlugin;
use datagen_rs::plugins::plugin::Plugin;
use datagen_rs::schema::schema_definition::Schema;
use datagen_rs::util::helpers::generate_random_data;
use datagen_rs::util::types::Result;
use serde_json::{from_str, from_value, json, Value};

mod basic_auth;
mod bearer_auth;
mod keycloak_auth;
mod no_auth;

fn create_schema(plugin_args: Value) -> Result<String> {
    let schema: Schema = from_value(json!({
        "type": "array",
        "length": {
            "value": 5
        },
        "items": {
            "type": "string",
            "value": "test"
        },
        "options": {
            "serializer": {
                "type": "plugin",
                "pluginName": "upload-plugin",
                "args": plugin_args
            }
        }
    }))
    .unwrap();

    let plugin: Box<dyn Plugin> = Box::<UploadPlugin>::default();
    generate_random_data(
        schema,
        Some(vec![("upload-plugin".into(), plugin)].into_iter().collect()),
    )
}

fn check_array(data: String) {
    let data: Vec<Value> = from_str(&data).unwrap();
    assert_eq!(data.len(), 5);
    for value in data {
        assert_eq!(value, "test");
    }
}
