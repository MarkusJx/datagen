use crate::tests::create_schema;
use mockito::Matcher;
use serde_json::json;

#[test]
fn test_bearer_auth() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/")
        .with_status(201)
        .match_header("Content-Type", "application/json")
        .match_header("Authorization", "Bearer test")
        .match_body(Matcher::Json(json!("test")))
        .create()
        .expect(5);

    let res = create_schema(json!({
        "url": server.url(),
        "returnNull": true,
        "splitTopLevelArray": true,
        "auth": {
            "type": "bearer",
            "token": "test",
        }
    }))
    .unwrap();

    mock.assert();
    assert_eq!(res, "".to_string());
}

#[test]
fn test_bearer_auth_invalid_token() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/")
        .with_status(401)
        .match_header("Content-Type", "application/json")
        .match_header("Authorization", "Bearer invalid")
        .match_body(Matcher::Json(json!("test")))
        .create()
        .expect(5);

    let err = create_schema(json!({
        "url": server.url(),
        "returnNull": true,
        "splitTopLevelArray": true,
        "auth": {
            "type": "bearer",
            "token": "invalid",
        }
    }))
    .unwrap_err();

    mock.assert();
    assert!(
        format!("{:?}", err).contains("Returned status was not ok: 401 Unauthorized"),
        "{:?}",
        err
    );
}
