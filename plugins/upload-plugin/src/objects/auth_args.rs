use crate::auth::oidc::objects::OidcAuthArgs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub(crate) enum AuthArgs {
    Basic {
        username: String,
        password: Option<String>,
    },
    Bearer {
        token: String,
    },
    Oidc(OidcAuthArgs),
}
