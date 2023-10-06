use crate::auth::authentication::Authentication;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::time::Duration;
use tokio::sync::Mutex;

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

#[doc(hidden)]
#[derive(Deserialize, Serialize)]
pub(crate) struct KeycloakAuthResponse {
    #[doc(hidden)]
    pub(crate) access_token: String,
    #[doc(hidden)]
    pub(crate) refresh_token: String,
    #[doc(hidden)]
    pub(crate) expires_in: u64,
    #[doc(hidden)]
    pub(crate) refresh_expires_in: u64,
}

#[derive(Serialize)]
struct KeycloakRefreshRequest {
    grant_type: String,
    client_id: String,
    refresh_token: String,
}

impl KeycloakRefreshRequest {
    fn new(args: &KeycloakAuthArgs, refresh_token: &str) -> Self {
        Self {
            grant_type: "refresh_token".into(),
            client_id: args.client_id.clone(),
            refresh_token: refresh_token.to_owned(),
        }
    }
}

#[derive(Clone)]
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

    fn token_headers() -> anyhow::Result<HeaderMap> {
        Ok(vec![(
            HeaderName::from_str("Accept")?,
            HeaderValue::from_str("application/json")?,
        )]
        .into_iter()
        .collect())
    }

    async fn fetch(args: &KeycloakAuthArgs, client: Option<Client>) -> anyhow::Result<Self> {
        let client = client.unwrap_or_else(Client::new);

        let res: KeycloakAuthResponse = client
            .post(format!(
                "{}/realms/{}/protocol/openid-connect/token",
                args.host, args.realm
            ))
            .timeout(Duration::from_secs(10))
            .headers(Self::token_headers()?)
            .form(&KeycloakAuthRequest::from(args.clone()))
            .send()
            .await?
            .json()
            .await?;

        Ok(Self::new(res, client))
    }

    async fn refresh_token(&mut self, args: &KeycloakAuthArgs) -> anyhow::Result<()> {
        let res: KeycloakAuthResponse = self
            .client
            .post(format!(
                "{}/realms/{}/protocol/openid-connect/token",
                args.host, args.realm
            ))
            .timeout(Duration::from_secs(10))
            .headers(Self::token_headers()?)
            .form(&KeycloakRefreshRequest::new(args, &self.refresh_token))
            .send()
            .await?
            .json()
            .await?;

        self.access_token = res.access_token;
        self.expires_at = Self::expires_at(res.expires_in);
        self.refresh_token = res.refresh_token;
        self.refresh_expires_at = Self::expires_at(res.refresh_expires_in);

        Ok(())
    }

    async fn get_token(&mut self, args: &KeycloakAuthArgs) -> anyhow::Result<String> {
        if self.refresh_expires_at.sub(Duration::from_secs(10)) <= Utc::now() {
            let new = Self::fetch(args, Some(self.client.clone())).await?;

            self.access_token = new.access_token;
            self.expires_at = new.expires_at;
            self.refresh_token = new.refresh_token;
            self.refresh_expires_at = new.refresh_expires_at;
        } else if self.expires_at.sub(Duration::from_secs(10)) <= Utc::now() {
            self.refresh_token(args).await?;
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

    async fn fetch_token(&self) -> anyhow::Result<String> {
        let mut token_lock = self.token.lock().await;
        if token_lock.is_none() {
            token_lock.replace(KeycloakToken::fetch(&self.args, None).await?);
        }

        token_lock.as_mut().unwrap().get_token(&self.args).await
    }
}

#[async_trait]
impl Authentication for KeycloakAuth {
    async fn add_auth(&self, builder: RequestBuilder) -> anyhow::Result<RequestBuilder> {
        Ok(builder.bearer_auth(self.fetch_token().await?))
    }
}
