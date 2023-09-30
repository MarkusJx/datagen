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
    #[serde(rename_all = "camelCase")]
    Keycloak {
        realm: String,
        username: String,
        password: String,
        client_id: String,
        host: String,
    },
}
