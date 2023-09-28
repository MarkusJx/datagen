use crate::auth::keycloak_auth::KeycloakAuthResponse;
use crate::tests::create_schema;
use datagen_rs::util::types::Result;
use mockito::{Matcher, Mock, ServerGuard};
use serde_json::{json, to_string};

fn create_fetch_mock(server: &mut ServerGuard, expires_in: u64, refresh_expires_in: u64) -> Mock {
    server
        .mock("POST", "/test/realms/realm/protocol/openid-connect/token")
        .match_header("Content-Type", "application/x-www-form-urlencoded")
        .match_header("Accept", "application/json")
        .match_body("grant_type=password&client_id=client&username=testuser&password=testpass")
        .with_body(
            to_string(&KeycloakAuthResponse {
                access_token: "access".to_string(),
                expires_in,
                refresh_expires_in,
                refresh_token: "refresh".to_string(),
            })
            .unwrap(),
        )
        .create()
}

fn create_request_mock(server: &mut ServerGuard) -> Mock {
    server
        .mock("POST", "/")
        .with_status(201)
        .match_header("Content-Type", "application/json")
        .match_header("Authorization", "Bearer access")
        .match_body(Matcher::Json(json!("test")))
        .create()
        .expect(5)
}

fn create_keycloak_schema(server: &ServerGuard) -> Result<String> {
    create_schema(json!({
        "url": server.url(),
        "returnNull": true,
        "splitTopLevelArray": true,
        "auth": {
            "type": "keycloak",
            "realm": "realm",
            "username": "testuser",
            "password": "testpass",
            "clientId": "client",
            "host": format!("{}/test", server.url()),
        }
    }))
}

#[test]
fn test_keycloak_auth() {
    let mut server = mockito::Server::new();
    let mock = create_request_mock(&mut server);
    let fetch_mock = create_fetch_mock(&mut server, 100, 100);

    let res = create_keycloak_schema(&server).unwrap();

    mock.assert();
    fetch_mock.assert();
    assert_eq!(res, "".to_string());
}

#[test]
fn test_refresh_token() {
    let mut server = mockito::Server::new();
    let mock = create_request_mock(&mut server);
    let fetch_mock = create_fetch_mock(&mut server, 0, 100);
    let refresh_mock = server
        .mock("POST", "/test/realms/realm/protocol/openid-connect/token")
        .match_header("Content-Type", "application/x-www-form-urlencoded")
        .match_header("Accept", "application/json")
        .match_body("grant_type=refresh_token&client_id=client&refresh_token=refresh")
        .with_body(
            to_string(&KeycloakAuthResponse {
                access_token: "access".to_string(),
                expires_in: 100,
                refresh_expires_in: 100,
                refresh_token: "refresh".to_string(),
            })
            .unwrap(),
        )
        .create();

    let res = create_keycloak_schema(&server).unwrap();

    mock.assert();
    fetch_mock.assert();
    refresh_mock.assert();
    assert_eq!(res, "".to_string());
}
