use oauth2::reqwest::async_http_client;
use oauth2::{
    DeviceAuthorizationResponse, DeviceAuthorizationUrl, EmptyExtraDeviceAuthorizationFields,
    ResourceOwnerPassword, ResourceOwnerUsername, Scope,
};
use openidconnect::core::{CoreClient, CoreResponseType, CoreTokenResponse};
use openidconnect::{AuthType, AuthenticationFlow};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OidcAuthArgs {
    /// The client ID to use.
    pub client_id: String,
    /// The client secret to use. Optional.
    pub client_secret: Option<String>,
    /// The URL of the OpenID Connect discovery document.
    pub discovery_url: String,
    /// The scopes to request. Optional.
    pub scopes: Option<Vec<String>>,
    /// The authentication flow to use. Defaults to `AuthorizationCode`.
    pub auth_flow: Option<OidcAuthFlow>,
    /// The authentication type to use. Defaults to `RequestBody`.
    pub auth_type: Option<OidcAuthType>,
    /// The method to use to log in.
    pub method: OidcLoginMethod,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub(crate) enum OidcLoginMethod {
    ClientCredentials,
    Password {
        username: String,
        password: String,
    },
    #[serde(rename_all = "camelCase")]
    DeviceCode {
        device_authorization_url: String,
        /// The timeout to use for the device code flow in seconds. Optional.
        timeout: Option<u64>,
    },
}

impl OidcLoginMethod {
    pub(crate) async fn get_token(
        &self,
        mut client: CoreClient,
        args: &OidcAuthArgs,
    ) -> anyhow::Result<(CoreTokenResponse, CoreClient)> {
        let scopes = args
            .scopes
            .clone()
            .map(|v| v.into_iter().map(Scope::new).collect::<Vec<_>>())
            .unwrap_or_default();

        let res = match self {
            OidcLoginMethod::ClientCredentials => {
                client
                    .exchange_client_credentials()
                    .add_scopes(scopes)
                    .request_async(async_http_client)
                    .await?
            }
            OidcLoginMethod::Password { username, password } => {
                client
                    .exchange_password(
                        &ResourceOwnerUsername::new(username.clone()),
                        &ResourceOwnerPassword::new(password.clone()),
                    )
                    .add_scopes(scopes)
                    .request_async(async_http_client)
                    .await?
            }
            OidcLoginMethod::DeviceCode {
                device_authorization_url,
                timeout,
            } => {
                client = client.set_device_authorization_uri(DeviceAuthorizationUrl::new(
                    device_authorization_url.clone(),
                )?);

                let dc: DeviceAuthorizationResponse<EmptyExtraDeviceAuthorizationFields> = client
                    .exchange_device_code()?
                    .add_scopes(scopes)
                    .request_async(async_http_client)
                    .await?;

                println!("device_code: {:?}", dc.device_code());
                println!(
                    "Go to: {}",
                    dc.verification_uri_complete()
                        .map(|u| u.secret().clone())
                        .unwrap_or(dc.verification_uri().to_string())
                );

                client
                    .exchange_device_access_token(&dc)
                    .request_async(
                        async_http_client,
                        tokio::time::sleep,
                        timeout.map(tokio::time::Duration::from_secs),
                    )
                    .await?
            }
        };

        Ok((res, client))
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) enum OidcAuthFlow {
    #[default]
    AuthorizationCode,
    Implicit,
}

impl From<OidcAuthFlow> for AuthenticationFlow<CoreResponseType> {
    fn from(value: OidcAuthFlow) -> Self {
        match value {
            OidcAuthFlow::AuthorizationCode => AuthenticationFlow::AuthorizationCode,
            OidcAuthFlow::Implicit => AuthenticationFlow::Implicit(false),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub enum OidcAuthType {
    #[default]
    RequestBody,
    BasicAuth,
}

impl From<OidcAuthType> for AuthType {
    fn from(value: OidcAuthType) -> Self {
        match value {
            OidcAuthType::RequestBody => AuthType::RequestBody,
            OidcAuthType::BasicAuth => AuthType::BasicAuth,
        }
    }
}
