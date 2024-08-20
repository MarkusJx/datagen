use crate::tests::{check_array, create_schema};
use crate::UploadPlugin;
use datagen_rs::plugins::plugin::Plugin;
use datagen_rs::schema::schema_definition::Schema;
use datagen_rs::util::helpers::generate_random_data;
use mockito::{Matcher, Mock, ServerGuard};
use serde_json::{from_value, json, Value};
use std::sync::Arc;
use std::vec;

fn create_mock(server: &mut ServerGuard, method: &str, status: usize) -> Mock {
    server
        .mock(method, "/")
        .with_status(status)
        .match_header("Content-Type", "application/json")
        .match_body(Matcher::Json(json!([
            "test", "test", "test", "test", "test"
        ])))
        .create()
        .expect(1)
}

fn create_object_schema(plugin_args: Value) -> anyhow::Result<String> {
    let schema: Schema = from_value(json!({
        "type": "object",
        "properties": {
            "test": {
                "type": "string",
                "value": "test"
            },
            "other": {
                "type": "integer",
                "value": 2
            }
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

    let plugin: Arc<dyn Plugin> = Arc::<UploadPlugin>::default();
    generate_random_data(
        schema,
        Some(vec![("upload-plugin".into(), plugin)].into_iter().collect()),
    )
}

#[test]
fn test_upload_single() {
    let mut server = mockito::Server::new();

    let mock = create_mock(&mut server, "POST", 201);
    let res = create_schema(json!({
        "url": server.url(),
    }))
    .unwrap();

    mock.assert();
    check_array(res);
}

#[test]
fn test_upload_multi() {
    let mut server = mockito::Server::new();

    let mock = server
        .mock("POST", "/")
        .with_status(201)
        .match_header("Content-Type", "application/json")
        .match_body(Matcher::Json(json!("test")))
        .create()
        .expect(5);

    let res = create_schema(json!({
        "url": server.url(),
        "splitTopLevelArray": true,
    }))
    .unwrap();

    mock.assert();
    check_array(res);
}

#[test]
fn test_upload_xml() {
    let mut server = mockito::Server::new();

    let mock = server
        .mock("POST", "/")
        .with_status(201)
        .match_header("Content-Type", "application/xml")
        .match_body(Matcher::Exact("<test>test</test>".into()))
        .create()
        .expect(5);

    let res = create_schema(json!({
        "url": server.url(),
        "splitTopLevelArray": true,
        "serializer": {
            "type": "xml",
            "rootElement": "test"
        }
    }))
    .unwrap();

    mock.assert();
    assert_eq!(
        res,
        "<test>test</test><test>test</test><test>test</test><test>test</test><test>test</test>"
            .to_string()
    );
}

#[test]
fn test_upload_yaml() {
    let mut server = mockito::Server::new();

    let mock = server
        .mock("POST", "/")
        .with_status(201)
        .match_header("Content-Type", "application/yaml")
        .match_body(Matcher::Exact("test\n".into()))
        .create()
        .expect(5);

    let res = create_schema(json!({
        "url": server.url(),
        "splitTopLevelArray": true,
        "serializer": {
            "type": "yaml",
        }
    }))
    .unwrap();

    mock.assert();
    assert_eq!(res, "- test\n- test\n- test\n- test\n- test\n".to_string());
}

#[test]
fn test_upload_return_null() {
    let mut server = mockito::Server::new();

    let mock = create_mock(&mut server, "POST", 201);
    let res = create_schema(json!({
        "url": server.url(),
        "returnNull": true,
    }))
    .unwrap();

    mock.assert();
    assert_eq!(res, "".to_string());
}

#[test]
fn test_upload_put() {
    let mut server = mockito::Server::new();

    let mock = create_mock(&mut server, "PUT", 201);
    let res = create_schema(json!({
        "url": server.url(),
        "returnNull": true,
        "method": "put"
    }))
    .unwrap();

    mock.assert();
    assert_eq!(res, "".to_string());
}

#[test]
fn test_upload_patch() {
    let mut server = mockito::Server::new();

    let mock = create_mock(&mut server, "PATCH", 201);
    let res = create_schema(json!({
        "url": server.url(),
        "returnNull": true,
        "method": "patch"
    }))
    .unwrap();

    mock.assert();
    assert_eq!(res, "".to_string());
}

#[test]
fn test_status_code() {
    let mut server = mockito::Server::new();

    let mock = create_mock(&mut server, "PATCH", 201);
    let res = create_schema(json!({
        "url": server.url(),
        "returnNull": true,
        "method": "patch",
        "expectedStatusCode": 201
    }))
    .unwrap();

    mock.assert();
    assert_eq!(res, "".to_string());
}

#[test]
fn test_status_code_not_matching() {
    let mut server = mockito::Server::new();

    let mock = create_mock(&mut server, "POST", 200);
    let res = create_schema(json!({
        "url": server.url(),
        "returnNull": true,
        "expectedStatusCode": 201
    }))
    .unwrap_err();

    mock.assert();
    assert!(format!("{:?}", res).contains("Expected status code 201, got 200 OK"), "{:?}", res);
}

#[test]
fn test_upload_with_headers() {
    let mut server = mockito::Server::new();

    let mock = server
        .mock("POST", "/")
        .with_status(201)
        .match_header("Content-Type", "application/yaml")
        .match_header("X-Test", "test")
        .match_header("X-Test2", "test2")
        .match_body(Matcher::Exact("test\n".into()))
        .create()
        .expect(5);

    let res = create_schema(json!({
        "url": server.url(),
        "splitTopLevelArray": true,
        "serializer": {
            "type": "yaml",
        },
        "headers": {
            "X-Test": "test",
            "X-Test2": "test2"
        }
    }))
    .unwrap();

    mock.assert();
    assert_eq!(res, "- test\n- test\n- test\n- test\n- test\n".to_string());
}

#[test]
fn test_upload_in_query() {
    let mut server = mockito::Server::new();

    let mock = server
        .mock("POST", "/")
        .with_status(201)
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("test".into(), "test".into()),
            Matcher::UrlEncoded("other".into(), "2".into()),
        ]))
        .create()
        .expect(1);

    let res = create_object_schema(json!({
        "url": server.url(),
        "splitTopLevelArray": true,
        "uploadIn": "query"
    }))
    .unwrap();

    mock.assert();
    assert_eq!(res, "{\"test\":\"test\",\"other\":2}".to_string());
}

#[test]
fn test_upload_in_form() {
    let mut server = mockito::Server::new();

    let mock = server
        .mock("POST", "/")
        .with_status(201)
        .match_header("Content-Type", "application/x-www-form-urlencoded")
        .match_body(Matcher::Exact("test=test&other=2".into()))
        .create()
        .expect(1);

    let res = create_object_schema(json!({
        "url": server.url(),
        "splitTopLevelArray": true,
        "uploadIn": "form"
    }))
    .unwrap();

    mock.assert();
    assert_eq!(res, "{\"test\":\"test\",\"other\":2}".to_string());
}
