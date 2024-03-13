use crate::tests::create_schema;
use chrono::Utc;
use mockito::{Matcher, Mock, ServerGuard};
use oauth2::{AccessToken, AuthUrl, EmptyExtraTokenFields, RefreshToken, TokenUrl};
use openidconnect::core::{
    CoreGrantType, CoreIdToken, CoreIdTokenClaims, CoreIdTokenFields, CoreJsonWebKeySet,
    CoreJwsSigningAlgorithm, CoreProviderMetadata, CoreResponseType, CoreRsaPrivateSigningKey,
    CoreSubjectIdentifierType, CoreTokenResponse, CoreTokenType,
};
use openidconnect::{
    Audience, EmptyAdditionalClaims, EmptyAdditionalProviderMetadata, IssuerUrl, JsonWebKeyId,
    JsonWebKeySetUrl, PrivateSigningKey, ResponseTypes, StandardClaims, SubjectIdentifier,
};
use rsa::pkcs1::der::zeroize::Zeroizing;
use rsa::pkcs1::{EncodeRsaPrivateKey, LineEnding};
use rsa::RsaPrivateKey;
use serde_json::{json, to_string};
use std::time::Duration;

const CLIENT_CREDENTIALS_BODY: &str =
    "grant_type=client_credentials&client_id=client&client_secret=client-secret";
const PASSWORD_BODY: &str = "grant_type=password&username=user&password=password&client_id=client&client_secret=client-secret";

fn create_oidc_discovery_mock(server: &mut ServerGuard) -> Mock {
    let provider_metadata = CoreProviderMetadata::new(
        IssuerUrl::new(server.url()).unwrap(),
        AuthUrl::new("https://invalid.com/authorize".to_string()).unwrap(),
        JsonWebKeySetUrl::new(format!("{}/jwk", server.url())).unwrap(),
        vec![
            ResponseTypes::new(vec![CoreResponseType::Code]),
            ResponseTypes::new(vec![CoreResponseType::Token, CoreResponseType::IdToken]),
        ],
        vec![CoreSubjectIdentifierType::Pairwise],
        vec![CoreJwsSigningAlgorithm::RsaSsaPssSha256],
        EmptyAdditionalProviderMetadata {},
    )
    .set_token_endpoint(Some(
        TokenUrl::new(format!("{}/token", server.url())).unwrap(),
    ));

    server
        .mock("GET", "/.well-known/openid-configuration")
        .match_header("Accept", "application/json")
        .with_body(to_string(&provider_metadata).unwrap())
        .expect_at_least(1)
        .create()
}

fn create_private_key() -> Zeroizing<String> {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");

    private_key
        .to_pkcs1_pem(LineEnding::LF)
        .expect("failed to encode the key")
}

fn create_jwks_mock(server: &mut ServerGuard, private_key: &Zeroizing<String>) -> Mock {
    let jwks = CoreJsonWebKeySet::new(vec![CoreRsaPrivateSigningKey::from_pem(
        private_key.as_str(),
        Some(JsonWebKeyId::new("key1".to_string())),
    )
    .expect("Invalid RSA private key")
    .as_verification_key()]);

    server
        .mock("GET", "/jwk")
        .match_header("Accept", "application/json")
        .with_body(to_string(&jwks).unwrap())
        .expect_at_least(1)
        .create()
}

fn create_id_token(
    server: &ServerGuard,
    expires_in: u64,
    private_key: &Zeroizing<String>,
) -> CoreIdToken {
    CoreIdToken::new(
        CoreIdTokenClaims::new(
            IssuerUrl::new(server.url()).unwrap(),
            vec![Audience::new("client-id-123".to_string())],
            Utc::now() + Duration::from_secs(expires_in),
            Utc::now(),
            StandardClaims::new(SubjectIdentifier::new(
                "5f83e0ca-2b8e-4e8c-ba0a-f80fe9bc3632".to_string(),
            )),
            EmptyAdditionalClaims {},
        ),
        &CoreRsaPrivateSigningKey::from_pem(
            private_key.as_str(),
            Some(JsonWebKeyId::new("key1".to_string())),
        )
        .expect("Invalid RSA private key"),
        CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha256,
        None,
        None,
    )
    .unwrap()
}

fn create_token_mock(
    server: &mut ServerGuard,
    private_key: &Zeroizing<String>,
    expires_in: u64,
    refresh_expires_in: u64,
    grant_type: CoreGrantType,
) -> (Mock, CoreIdToken, CoreIdToken) {
    let access_token = create_id_token(server, expires_in, private_key);
    let refresh_token = create_id_token(server, refresh_expires_in, private_key);

    let mut response = CoreTokenResponse::new(
        AccessToken::new(access_token.to_string()),
        CoreTokenType::Bearer,
        CoreIdTokenFields::new(None, EmptyExtraTokenFields {}),
    );
    response.set_refresh_token(Some(RefreshToken::new(refresh_token.to_string())));

    (
        server
            .mock("POST", "/token")
            .match_header("Content-Type", "application/x-www-form-urlencoded")
            .match_body(match grant_type {
                CoreGrantType::ClientCredentials => CLIENT_CREDENTIALS_BODY,
                CoreGrantType::Password => PASSWORD_BODY,
                _ => panic!("Unsupported grant type: {grant_type:?}"),
            })
            .with_body(to_string(&response).unwrap())
            .create(),
        access_token,
        refresh_token,
    )
}

fn create_refresh_token_mock(
    server: &mut ServerGuard,
    private_key: &Zeroizing<String>,
    expires_in: u64,
    refresh_token: &CoreIdToken,
) -> (Option<Mock>, CoreIdToken) {
    let access_token = create_id_token(server, expires_in, private_key);
    let response = CoreTokenResponse::new(
        AccessToken::new(access_token.to_string()),
        CoreTokenType::Bearer,
        CoreIdTokenFields::new(None, EmptyExtraTokenFields {}),
    );

    (Some(server
        .mock("POST", "/token")
        .match_header("Content-Type", "application/x-www-form-urlencoded")
        .match_body(
            format!(
                "grant_type=refresh_token&refresh_token={}&client_id=client&client_secret=client-secret",
                refresh_token.to_string()
            )
            .as_str(),
        )
        .with_body(to_string(&response).unwrap())
        .create()), access_token)
}

pub(crate) fn create_request_mock(server: &mut ServerGuard, access_token: &CoreIdToken) -> Mock {
    server
        .mock("POST", "/")
        .with_status(201)
        .match_header("Content-Type", "application/json")
        .match_header(
            "Authorization",
            Matcher::Exact(format!("Bearer {}", access_token.to_string())),
        )
        .match_body(Matcher::Json(json!("test")))
        .create()
        .expect(5)
}

pub(crate) fn create_oidc_schema(server: &ServerGuard) -> anyhow::Result<String> {
    create_schema(json!({
        "url": server.url(),
        "returnNull": true,
        "splitTopLevelArray": true,
        "auth": {
            "type": "oidc",
            "clientId": "client",
            "clientSecret": "client-secret",
            "discoveryUrl": server.url(),
            "method": {
                "type": "clientCredentials"
            }
        }
    }))
}

pub(crate) struct OidcMock {
    discovery_mock: Mock,
    jwks_mock: Mock,
    token_mock: Mock,
    refresh_token_mock: Option<Mock>,
    pub access_token: CoreIdToken,
    refresh_token: CoreIdToken,
    private_key: Zeroizing<String>,
}

impl OidcMock {
    pub(crate) fn new(
        server: &mut ServerGuard,
        expires_in: u64,
        refresh_expires_in: u64,
        grant_type: CoreGrantType,
    ) -> Self {
        let private_key = create_private_key();

        let (token_mock, access_token, refresh_token) = create_token_mock(
            server,
            &private_key,
            expires_in,
            refresh_expires_in,
            grant_type,
        );
        let discovery_mock = create_oidc_discovery_mock(server);
        let jwks_mock = create_jwks_mock(server, &private_key);

        Self {
            discovery_mock,
            jwks_mock,
            token_mock,
            access_token,
            refresh_token,
            refresh_token_mock: None,
            private_key,
        }
    }

    pub(crate) fn create_refresh_token_mock(&mut self, server: &mut ServerGuard, expires_in: u64) {
        (self.refresh_token_mock, self.access_token) =
            create_refresh_token_mock(server, &self.private_key, expires_in, &self.refresh_token);
    }

    pub(crate) fn assert(&self) {
        if let Some(refresh_token_mock) = &self.refresh_token_mock {
            refresh_token_mock.assert();
        }
        self.discovery_mock.assert();
        self.jwks_mock.assert();
        self.token_mock.assert();
    }
}
