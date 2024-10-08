use crate::auth::authentication::Authentication;
use crate::auth::oidc::objects::OidcAuthArgs;
use crate::auth::oidc::oidc_token::OidcToken;
use async_trait::async_trait;
use oauth2::AuthType;
use openidconnect::core::CoreProviderMetadata;
use openidconnect::{ClientId, ClientSecret, IssuerUrl};
use tokio::sync::Mutex;

use super::auth_client::AuthClient;

pub(crate) struct OidcAuth {
    args: OidcAuthArgs,
    client: AuthClient,
    token: Mutex<Option<OidcToken>>,
}

impl OidcAuth {
    pub(crate) fn new(args: OidcAuthArgs, client: reqwest::Client) -> Self {
        Self {
            args,
            client: client.into(),
            token: Mutex::new(None),
        }
    }

    async fn init(&self) -> anyhow::Result<OidcToken> {
        let metadata = CoreProviderMetadata::discover_async(
            IssuerUrl::new(self.args.discovery_url.clone())?,
            |req| self.client.request(req),
        )
        .await?;

        let client = openidconnect::core::CoreClient::from_provider_metadata(
            metadata,
            ClientId::new(self.args.client_id.clone()),
            self.args.client_secret.clone().map(ClientSecret::new),
        )
        .set_auth_type(
            self.args
                .auth_type
                .clone()
                .map(Into::into)
                .unwrap_or(AuthType::RequestBody),
        );

        let (token, client) = self
            .args
            .method
            .get_token(client, &self.client, &self.args)
            .await?;

        OidcToken::new(token, client)
    }

    async fn fetch_token(&self) -> anyhow::Result<String> {
        let mut token_lock = self.token.lock().await;
        if token_lock.is_none() {
            token_lock.replace(self.init().await?);
        }

        token_lock
            .as_mut()
            .unwrap()
            .get_token(&self.client, &self.args)
            .await
    }
}

#[async_trait]
impl Authentication for OidcAuth {
    async fn add_auth(
        &self,
        builder: reqwest::RequestBuilder,
    ) -> anyhow::Result<reqwest::RequestBuilder> {
        Ok(builder.bearer_auth(self.fetch_token().await?))
    }
}
