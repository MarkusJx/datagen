use crate::auth::keycloak_auth::{KeycloakAuth, KeycloakAuthArgs};
use crate::objects::auth_args::AuthArgs;
use async_trait::async_trait;
use reqwest::RequestBuilder;

#[async_trait]
pub(crate) trait AnyAuth {
    async fn add_auth(self, auth: &dyn Authentication) -> anyhow::Result<Self>
    where
        Self: Sized;
}

#[async_trait]
impl AnyAuth for RequestBuilder {
    async fn add_auth(self, auth: &dyn Authentication) -> anyhow::Result<Self> {
        auth.add_auth(self).await
    }
}

#[async_trait]
pub(crate) trait Authentication: Send + Sync {
    async fn add_auth(&self, builder: RequestBuilder) -> anyhow::Result<RequestBuilder>;

    fn from_args(args: Option<AuthArgs>) -> Box<dyn Authentication>
    where
        Self: Sized,
    {
        match args {
            Some(AuthArgs::Basic { username, password }) => {
                Box::new(BasicAuth { username, password })
            }
            Some(AuthArgs::Bearer { token }) => Box::new(BearerAuth { token }),
            Some(AuthArgs::Keycloak {
                realm,
                username,
                password,
                client_id,
                host,
            }) => Box::new(KeycloakAuth::new(KeycloakAuthArgs {
                realm,
                username,
                password,
                client_id,
                host,
            })),
            None => Box::new(NoAuth),
        }
    }
}

pub(crate) struct BasicAuth {
    username: String,
    password: Option<String>,
}

#[async_trait]
impl Authentication for BasicAuth {
    async fn add_auth(&self, builder: RequestBuilder) -> anyhow::Result<RequestBuilder> {
        Ok(builder.basic_auth(&self.username, self.password.as_ref()))
    }
}

pub(crate) struct BearerAuth {
    token: String,
}

#[async_trait]
impl Authentication for BearerAuth {
    async fn add_auth(&self, builder: RequestBuilder) -> anyhow::Result<RequestBuilder> {
        Ok(builder.bearer_auth(&self.token))
    }
}

pub(crate) struct NoAuth;

#[async_trait]
impl Authentication for NoAuth {
    async fn add_auth(&self, builder: RequestBuilder) -> anyhow::Result<RequestBuilder> {
        Ok(builder)
    }
}
