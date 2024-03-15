use std::ops::{Add, Sub};
use std::str::FromStr;
use std::time::Duration;

use crate::auth::oidc::objects::OidcAuthArgs;
use chrono::{DateTime, Utc};
use openidconnect::core::{CoreClient, CoreIdToken, CoreIdTokenVerifier, CoreTokenResponse};
use openidconnect::reqwest::async_http_client;
use openidconnect::{AccessToken, Nonce, OAuth2TokenResponse, RefreshToken};

pub(crate) struct OidcToken {
    access_token: AccessToken,
    expires_at: DateTime<Utc>,
    refresh_token: Option<RefreshToken>,
    refresh_expires_at: Option<DateTime<Utc>>,
    client: CoreClient,
}

impl OidcToken {
    pub(crate) fn new(token: CoreTokenResponse, client: CoreClient) -> anyhow::Result<Self> {
        Ok(Self {
            access_token: token.access_token().clone(),
            expires_at: Self::get_access_token_expiry(&token)?,
            refresh_token: token.refresh_token().cloned(),
            refresh_expires_at: Self::get_refresh_token_expiry(&token)?,
            client,
        })
    }

    pub async fn get_token(&mut self, args: &OidcAuthArgs) -> anyhow::Result<String> {
        if Self::is_expired(&self.expires_at) {
            self.refresh(args).await?;
        }

        Ok(self.access_token.secret().clone())
    }

    async fn refresh(&mut self, args: &OidcAuthArgs) -> anyhow::Result<()> {
        if self
            .refresh_expires_at
            .as_ref()
            .map(Self::is_expired)
            .unwrap_or(true)
        {
            self.re_fetch_tokens(args).await?;
            return Ok(());
        }

        let token = self
            .client
            .exchange_refresh_token(self.refresh_token.as_ref().unwrap())
            .request_async(async_http_client)
            .await?;

        self.access_token = token.access_token().clone();
        self.expires_at = Self::get_access_token_expiry(&token)?;

        Ok(())
    }

    async fn re_fetch_tokens(&mut self, args: &OidcAuthArgs) -> anyhow::Result<()> {
        let token = args.method.get_token(self.client.clone(), args).await?.0;

        self.access_token = token.access_token().clone();
        self.expires_at = Self::get_access_token_expiry(&token)?;
        self.refresh_token = token.refresh_token().cloned();
        self.refresh_expires_at = Self::get_refresh_token_expiry(&token)?;

        Ok(())
    }

    fn get_access_token_expiry(token: &CoreTokenResponse) -> anyhow::Result<DateTime<Utc>> {
        token
            .expires_in()
            .map(Self::expires_at)
            .map(Ok)
            .unwrap_or_else(|| Self::get_token_expiry(token.access_token().secret()))
    }

    fn get_refresh_token_expiry(
        token: &CoreTokenResponse,
    ) -> anyhow::Result<Option<DateTime<Utc>>> {
        token
            .refresh_token()
            .map(|rt| Self::get_token_expiry(rt.secret()))
            .map_or(Ok(None), |rt| rt.map(Some))
    }

    fn get_token_expiry(secret: &str) -> anyhow::Result<DateTime<Utc>> {
        Ok(CoreIdToken::from_str(secret)?
            .claims(
                &CoreIdTokenVerifier::new_insecure_without_verification(),
                Self::nonce_verifier,
            )?
            .expiration())
    }

    fn expires_at(expires_in: Duration) -> DateTime<Utc> {
        Utc::now().add(expires_in)
    }

    fn is_expired(expires_at: &DateTime<Utc>) -> bool {
        expires_at.sub(Duration::from_secs(10)) <= Utc::now()
    }

    fn nonce_verifier(_: Option<&Nonce>) -> Result<(), String> {
        Ok(())
    }
}
