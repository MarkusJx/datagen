use crate::tests::create_schema;
use crate::tests::util::oidc_auth_helpers::{create_oidc_schema, create_request_mock, OidcMock};
use openidconnect::core::CoreGrantType;
use serde_json::json;

#[test]
fn test_oidc_auth() {
    let mut server = mockito::Server::new();
    let oidc_mock = OidcMock::new(&mut server, 3600, 7200, CoreGrantType::ClientCredentials);
    let mock = create_request_mock(&mut server, &oidc_mock.access_token);

    let res = create_oidc_schema(&server);

    mock.assert();
    oidc_mock.assert();
    assert!(
        res.is_ok(),
        "{}",
        res.err().map(|e| e.to_string()).unwrap_or_default()
    );
    assert_eq!(res.unwrap(), "".to_string());
}

#[test]
fn test_oidc_auth_refresh() {
    let mut server = mockito::Server::new();
    let mut oidc_mock = OidcMock::new(&mut server, 8, 7200, CoreGrantType::ClientCredentials);
    oidc_mock.create_refresh_token_mock(&mut server, 3600);
    let mock = create_request_mock(&mut server, &oidc_mock.access_token);

    let res = create_oidc_schema(&server);

    oidc_mock.assert();
    mock.assert();
    assert!(
        res.is_ok(),
        "{}",
        res.err().map(|e| e.to_string()).unwrap_or_default()
    );
    assert_eq!(res.unwrap(), "".to_string());
}

#[test]
fn test_oidc_auth_password() {
    let mut server = mockito::Server::new();
    let oidc_mock = OidcMock::new(&mut server, 3600, 7200, CoreGrantType::Password);
    let mock = create_request_mock(&mut server, &oidc_mock.access_token);

    let res = create_schema(json!({
        "url": server.url(),
        "returnNull": true,
        "splitTopLevelArray": true,
        "auth": {
            "type": "oidc",
            "clientId": "client",
            "clientSecret": "client-secret",
            "discoveryUrl": server.url(),
            "method": {
                "type": "password",
                "username": "user",
                "password": "password"
            }
        }
    }));

    mock.assert();
    oidc_mock.assert();
    assert!(
        res.is_ok(),
        "{}",
        res.err().map(|e| e.to_string()).unwrap_or_default()
    );
    assert_eq!(res.unwrap(), "".to_string());
}
