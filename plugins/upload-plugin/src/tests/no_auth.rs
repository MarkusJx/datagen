use crate::tests::{check_array, create_schema};
use mockito::{Matcher, Mock, ServerGuard};
use serde_json::json;

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
    assert_eq!(res.to_string(), "Expected status code 201, got 200 OK");
}
