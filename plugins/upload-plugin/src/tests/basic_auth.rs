use crate::tests::create_schema;
use mockito::Matcher;
use serde_json::json;

#[test]
fn test_basic_auth() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/")
        .with_status(201)
        .match_header("Content-Type", "application/json")
        .match_header("Authorization", "Basic dGVzdDp0ZXN0")
        .match_body(Matcher::Json(json!("test")))
        .create()
        .expect(5);

    let res = create_schema(json!({
        "url": server.url(),
        "returnNull": true,
        "splitTopLevelArray": true,
        "auth": {
            "type": "basic",
            "username": "test",
            "password": "test"
        }
    }))
    .unwrap();

    mock.assert();
    assert_eq!(res, "".to_string());
}

#[test]
fn test_basic_auth_no_password() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/")
        .with_status(201)
        .match_header("Content-Type", "application/json")
        .match_header("Authorization", "Basic dGVzdDo=")
        .match_body(Matcher::Json(json!("test")))
        .create()
        .expect(5);

    let res = create_schema(json!({
        "url": server.url(),
        "returnNull": true,
        "splitTopLevelArray": true,
        "auth": {
            "type": "basic",
            "username": "test",
        }
    }))
    .unwrap();

    mock.assert();
    assert_eq!(res, "".to_string());
}
