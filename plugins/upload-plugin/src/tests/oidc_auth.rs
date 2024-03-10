use crate::tests::util::oidc_auth_helpers::{create_oidc_schema, create_request_mock, OidcMock};

#[test]
fn test_oidc_auth() {
    let mut server = mockito::Server::new();
    let oidc_mock = OidcMock::new(&mut server, 3600, 7200);
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
