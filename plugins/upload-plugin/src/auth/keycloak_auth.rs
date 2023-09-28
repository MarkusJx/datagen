use crate::auth::authentication::Authentication;
use chrono::{DateTime, Utc};
use datagen_rs::util::types::Result;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::ops::Add;
use std::str::FromStr;
use std::sync::Mutex;
use std::time::Duration;

#[derive(Clone)]
pub(crate) struct KeycloakAuthArgs {
    pub(crate) realm: String,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) client_id: String,
    pub(crate) host: String,
}

#[derive(Serialize)]
struct KeycloakAuthRequest {
    grant_type: String,
    client_id: String,
    username: String,
    password: String,
}

impl From<KeycloakAuthArgs> for KeycloakAuthRequest {
    fn from(value: KeycloakAuthArgs) -> Self {
        Self {
            grant_type: "password".into(),
            client_id: value.client_id,
            username: value.username,
            password: value.password,
        }
    }
}

#[derive(Deserialize)]
struct KeycloakAuthResponse {
    access_token: String,
    refresh_token: String,
    expires_in: u64,
    refresh_expires_in: u64,
}

#[derive(Serialize)]
struct KeycloakRefreshRequest {
    client_id: String,
    grant_type: String,
    refresh_token: String,
}

impl KeycloakRefreshRequest {
    fn new(args: &KeycloakAuthArgs, refresh_token: &str) -> Self {
        Self {
            client_id: args.client_id.clone(),
            refresh_token: refresh_token.to_owned(),
            grant_type: "refresh_token".into(),
        }
    }
}

struct KeycloakToken {
    access_token: String,
    refresh_token: String,
    expires_at: DateTime<Utc>,
    refresh_expires_at: DateTime<Utc>,
    client: Client,
}

impl KeycloakToken {
    fn new(res: KeycloakAuthResponse, client: Client) -> Self {
        Self {
            access_token: res.access_token,
            expires_at: Self::expires_at(res.expires_in),
            refresh_token: res.refresh_token,
            refresh_expires_at: Self::expires_at(res.refresh_expires_in),
            client,
        }
    }

    fn expires_at(expires_in: u64) -> DateTime<Utc> {
        Utc::now().add(Duration::from_secs(expires_in))
    }

    fn token_headers() -> Result<HeaderMap> {
        Ok(vec![
            (
                HeaderName::from_str("Content-Type")?,
                HeaderValue::from_str("application/x-www-form-urlencoded")?,
            ),
            (
                HeaderName::from_str("Accept")?,
                HeaderValue::from_str("application/json")?,
            ),
        ]
        .into_iter()
        .collect())
    }

    fn fetch(args: &KeycloakAuthArgs, client: Option<Client>) -> Result<Self> {
        let client = client.unwrap_or_else(Client::new);

        let res: KeycloakAuthResponse = client
            .post(format!(
                "{}/realms/{}/protocol/openid-connect/token",
                args.host, args.realm
            ))
            .timeout(Duration::from_secs(10))
            .headers(Self::token_headers()?)
            .body(serde_urlencoded::to_string(KeycloakAuthRequest::from(
                args.clone(),
            ))?)
            .send()?
            .json()?;

        Ok(Self::new(res, client))
    }

    fn refresh_token(&mut self, args: &KeycloakAuthArgs) -> Result<()> {
        let res: KeycloakAuthResponse = self
            .client
            .post(format!(
                "{}/realms/{}/protocol/openid-connect/token",
                args.host, args.realm
            ))
            .timeout(Duration::from_secs(10))
            .headers(Self::token_headers()?)
            .body(serde_urlencoded::to_string(KeycloakRefreshRequest::new(
                args,
                &self.refresh_token,
            ))?)
            .send()?
            .json()?;

        self.access_token = res.access_token;
        self.expires_at = Self::expires_at(res.expires_in);
        self.refresh_token = res.refresh_token;
        self.refresh_expires_at = Self::expires_at(res.refresh_expires_in);

        Ok(())
    }

    fn get_token(&mut self, args: &KeycloakAuthArgs) -> Result<String> {
        if self.refresh_expires_at.add(Duration::from_secs(10)) >= Utc::now() {
            let new = Self::fetch(args, Some(self.client.clone()))?;

            self.access_token = new.access_token;
            self.expires_at = new.expires_at;
            self.refresh_token = new.refresh_token;
            self.refresh_expires_at = new.refresh_expires_at;
        } else if self.expires_at.add(Duration::from_secs(10)) >= Utc::now() {
            self.refresh_token(args)?;
        }

        Ok(self.access_token.clone())
    }
}

pub(crate) struct KeycloakAuth {
    args: KeycloakAuthArgs,
    token: Mutex<Option<KeycloakToken>>,
}

impl KeycloakAuth {
    pub(crate) fn new(args: KeycloakAuthArgs) -> Self {
        Self {
            args,
            token: Mutex::new(None),
        }
    }

    fn fetch_token(&self) -> Result<String> {
        let mut token_lock = self.token.lock().unwrap();
        if token_lock.is_none() {
            token_lock.replace(KeycloakToken::fetch(&self.args, None)?);
        }

        token_lock.as_mut().unwrap().get_token(&self.args)
    }
}

impl Authentication for KeycloakAuth {
    fn add_auth(&self, builder: RequestBuilder) -> Result<RequestBuilder> {
        Ok(builder.bearer_auth(self.fetch_token()?))
    }
}
