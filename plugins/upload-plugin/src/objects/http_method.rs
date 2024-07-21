use reqwest::{Client, IntoUrl, RequestBuilder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) enum HttpMethod {
    #[default]
    Post,
    Put,
    Patch,
}

impl Default for &HttpMethod {
    fn default() -> Self {
        &HttpMethod::Post
    }
}

impl HttpMethod {
    pub(crate) fn get_builder<U: IntoUrl>(&self, client: &Client, url: U) -> RequestBuilder {
        match self {
            HttpMethod::Post => client.post(url),
            HttpMethod::Put => client.put(url),
            HttpMethod::Patch => client.patch(url),
        }
    }
}
