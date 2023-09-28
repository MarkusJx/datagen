use crate::auth::keycloak_auth::{KeycloakAuth, KeycloakAuthArgs};
use crate::objects::auth_args::AuthArgs;
use datagen_rs::util::types::Result;
use reqwest::RequestBuilder;

pub(crate) trait AnyAuth {
    fn add_auth(self, auth: &dyn Authentication) -> Result<Self>
    where
        Self: Sized;
}

impl AnyAuth for RequestBuilder {
    fn add_auth(self, auth: &dyn Authentication) -> Result<Self> {
        auth.add_auth(self)
    }
}

pub(crate) trait Authentication: Send + Sync {
    fn add_auth(&self, builder: RequestBuilder) -> Result<RequestBuilder>;

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

impl Authentication for BasicAuth {
    fn add_auth(&self, builder: RequestBuilder) -> Result<RequestBuilder> {
        Ok(builder.basic_auth(&self.username, self.password.as_ref()))
    }
}

pub(crate) struct BearerAuth {
    token: String,
}

impl Authentication for BearerAuth {
    fn add_auth(&self, builder: RequestBuilder) -> Result<RequestBuilder> {
        Ok(builder.bearer_auth(&self.token))
    }
}

pub(crate) struct NoAuth;

impl Authentication for NoAuth {
    fn add_auth(&self, builder: RequestBuilder) -> Result<RequestBuilder> {
        Ok(builder)
    }
}
