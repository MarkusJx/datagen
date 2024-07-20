use crate::UploadPlugin;
use datagen_rs::plugins::plugin::Plugin;
use datagen_rs::schema::schema_definition::Schema;
use datagen_rs::util::helpers::generate_random_data;
use mockito::Matcher;
use serde_json::{from_value, json};
use std::sync::Arc;

#[test]
fn test_upload_by_object() {
    let mut server = mockito::Server::new();

    let mock_first = server
        .mock("POST", "/first")
        .with_status(201)
        .match_header("Content-Type", "application/json")
        .match_body(Matcher::Json(json!(["first", "first"])))
        .create()
        .expect(1);

    let mock_second = server
        .mock("POST", "/second")
        .with_status(201)
        .match_header("Content-Type", "application/json")
        .match_body(Matcher::Json(json!(["second", "second"])))
        .create()
        .expect(1);

    let schema: Schema = from_value(json!({
        "type": "object",
        "properties": {
            "first": {
                "type": "array",
                "length": 2,
                "items": {
                    "type": "string",
                    "value": "first"
                }
            },
            "second": {
                "type": "array",
                "length": 2,
                "items": {
                    "type": "string",
                    "value": "second"
                }
            }
        },
        "options": {
            "serializer": {
                "type": "plugin",
                "pluginName": "upload-plugin",
                "args": {
                    "url": {
                        "first": format!("{}/first", server.url()),
                        "second": format!("{}/second", server.url())
                    },
                    "splitTopLevelArray": false
                }
            }
        }
    }))
    .unwrap();

    let plugin: Arc<dyn Plugin> = Arc::<UploadPlugin>::default();
    let res = generate_random_data(
        schema,
        Some(vec![("upload-plugin".into(), plugin)].into_iter().collect()),
    )
    .unwrap();

    mock_first.assert();
    mock_second.assert();
    assert_eq!(
        json!({
            "first": ["first", "first"],
            "second": ["second", "second"]
        })
        .to_string(),
        res
    );
}

#[test]
fn test_upload_by_object_split_top_level_array() {
    let mut server = mockito::Server::new();

    let mock_first = server
        .mock("POST", "/first")
        .with_status(201)
        .match_header("Content-Type", "application/json")
        .match_body(Matcher::Json(json!("first")))
        .create()
        .expect(2);

    let mock_second = server
        .mock("POST", "/second")
        .with_status(201)
        .match_header("Content-Type", "application/json")
        .match_body(Matcher::Json(json!("second")))
        .create()
        .expect(2);

    let schema: Schema = from_value(json!({
        "type": "object",
        "properties": {
            "first": {
                "type": "array",
                "length": 2,
                "items": {
                    "type": "string",
                    "value": "first"
                }
            },
            "second": {
                "type": "array",
                "length": 2,
                "items": {
                    "type": "string",
                    "value": "second"
                }
            }
        },
        "options": {
            "serializer": {
                "type": "plugin",
                "pluginName": "upload-plugin",
                "args": {
                    "url": {
                        "first": format!("{}/first", server.url()),
                        "second": format!("{}/second", server.url())
                    },
                    "splitTopLevelArray": true
                }
            }
        }
    }))
    .unwrap();

    let plugin: Arc<dyn Plugin> = Arc::<UploadPlugin>::default();
    let res = generate_random_data(
        schema,
        Some(vec![("upload-plugin".into(), plugin)].into_iter().collect()),
    )
    .unwrap();

    mock_first.assert();
    mock_second.assert();
    assert_eq!(
        json!({
            "first": ["first", "first"],
            "second": ["second", "second"]
        })
        .to_string(),
        res
    );
}
