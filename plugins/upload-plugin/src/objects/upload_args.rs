use std::fmt::{Display, Formatter};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time;
use time::Duration;

use anyhow::anyhow;
use futures::{stream, StreamExt};
use indexmap::IndexMap;
use log::debug;
use reqwest::header::{HeaderMap, HeaderName};
use reqwest::{Client, IntoUrl, RequestBuilder};
use serde::{Deserialize, Serialize};
use tokio::runtime::Builder;

use datagen_rs::generate::generated_schema::GeneratedSchema;
use datagen_rs::plugins::plugin::PluginSerializeCallback;
use datagen_rs::schema::serializer::Serializer;

use crate::auth::authentication::{AnyAuth, Authentication, NoAuth};
use crate::objects::auth_args::AuthArgs;
use crate::objects::http_method::HttpMethod;
use crate::objects::upload_in::{AddData, UploadIn};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UploadArgs {
    /// The serializer to use when serializing the generated data.
    /// If not specified, the default is JSON.
    pub serializer: Option<Serializer>,
    /// Whether to discard the serialized value.
    /// If set to `true`, the serialized value will be uploaded to the server
    /// and then discarded, e.g. an empty string will be returned.
    /// Defaults to `false`.
    pub return_null: Option<bool>,
    /// The URL to upload the data to.
    /// This is required.
    pub url: StringOrMap,
    /// The HTTP method to use when uploading the data.
    /// Defaults to `post`.
    pub method: Option<HttpMethod>,
    /// Whether to split the top level array into multiple requests.
    /// If set to `true`, each item in the top level array will be uploaded
    /// in a separate request. If the top level value is not an array,
    /// this option will be ignored.
    /// Defaults to `false`.
    pub split_top_level_array: Option<bool>,
    /// The number of parallel requests to make.
    /// Defaults to `1`.
    pub num_parallel_requests: Option<usize>,
    /// The expected status code.
    /// If the status code does not match, an error will be thrown.
    /// If not specified, the default is any 2xx code.
    pub expected_status_code: Option<u16>,
    /// The authentication to use when uploading the data.
    /// If not specified, no authentication will be used.
    pub auth: Option<AuthArgs>,
    /// The timeout in milliseconds.
    /// If not specified, the default is no timeout.
    pub timeout: Option<u64>,
    /// Whether to upload the data in the body or as a query parameter.
    /// If not specified, the default is the body.
    pub upload_in: Option<UploadIn>,
    /// The headers to send with the request.
    /// If not specified, no additional headers will be sent.
    pub headers: Option<IndexMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged, deny_unknown_fields)]
pub enum StringOrMap {
    String(String),
    Map(IndexMap<String, String>),
}

impl Display for StringOrMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StringOrMap::String(s) => write!(f, "{}", s),
            StringOrMap::Map(map) => write!(f, "{:?}", map),
        }
    }
}

struct RequestCreator {
    method: HttpMethod,
    timeout: Option<Duration>,
    client: Client,
    auth: Box<dyn Authentication>,
    num_parallel_requests: usize,
}

impl From<&UploadArgs> for RequestCreator {
    fn from(args: &UploadArgs) -> Self {
        Self {
            method: args.method.clone().unwrap_or_default(),
            timeout: args.timeout.map(Duration::from_millis),
            client: Client::new(),
            auth: NoAuth::from_args(args.auth.clone()),
            num_parallel_requests: args.num_parallel_requests.unwrap_or(1),
        }
    }
}

impl RequestCreator {
    async fn get_builder<U: IntoUrl>(&self, url: U) -> anyhow::Result<RequestBuilder> {
        let mut builder = self
            .method
            .get_builder(&self.client, url)
            .add_auth(self.auth.as_ref())
            .await?;

        if let Some(timeout) = self.timeout {
            builder = builder.timeout(timeout);
        }

        Ok(builder)
    }
}

impl UploadArgs {
    fn get_headers(&self) -> anyhow::Result<HeaderMap> {
        let mut map = HeaderMap::new();

        if self.upload_in.is_none() || self.upload_in.unwrap_or_default() == UploadIn::Body {
            map.insert(
                "Content-Type",
                match self.serializer.as_ref().unwrap_or_default() {
                    Serializer::Json { .. } => "application/json",
                    Serializer::Yaml { .. } => "application/yaml",
                    Serializer::Xml { .. } => "application/xml",
                    _ => return Err(anyhow!("Unsupported serializer")),
                }
                .parse()?,
            );
        }

        if let Some(additional) = self.headers.as_ref() {
            for (key, value) in additional {
                map.insert(HeaderName::try_from(key.clone())?, value.parse()?);
            }
        }

        Ok(map)
    }

    fn split_top_level_array<'a>(
        &'a self,
        value: &Arc<GeneratedSchema>,
        url: &'a String,
    ) -> anyhow::Result<Vec<(&String, Arc<GeneratedSchema>)>> {
        if self.split_top_level_array.unwrap_or_default() {
            if let GeneratedSchema::Array(arr) = value.as_ref() {
                return Ok(arr
                    .iter()
                    .map(|schema| (url, schema.clone()))
                    .collect::<Vec<_>>());
            }
        }

        Ok(vec![(url, value.clone())])
    }

    fn split(
        &self,
        value: &Arc<GeneratedSchema>,
    ) -> anyhow::Result<Vec<(&String, Arc<GeneratedSchema>)>> {
        match &self.url {
            StringOrMap::String(string) => self.split_top_level_array(value, string),
            StringOrMap::Map(urls) => {
                if let GeneratedSchema::Object(obj) = value.as_ref() {
                    Ok(obj
                        .iter()
                        .map(|(key, schema)| {
                            let Some(url) = urls.get(key) else {
                                anyhow::bail!("No URL found for key {}", key);
                            };

                            self.split_top_level_array(schema, url)
                        })
                        .collect::<anyhow::Result<Vec<_>>>()?
                        .into_iter()
                        .flatten()
                        .collect::<Vec<_>>())
                } else {
                    Err(anyhow!("URL is a map, but value is not an object"))
                }
            }
        }
    }

    pub(crate) fn serialize_generated(
        &self,
        value: &Arc<GeneratedSchema>,
    ) -> anyhow::Result<String> {
        self.serializer
            .as_ref()
            .unwrap_or_default()
            .serialize_generated(value.clone(), None)
    }

    pub(crate) fn upload_data(
        &self,
        value: &Arc<GeneratedSchema>,
        progress_callback: PluginSerializeCallback,
    ) -> anyhow::Result<()> {
        let headers = self.get_headers()?;
        let split = self.split(value)?;
        let creator = RequestCreator::from(self);
        let serializer = self.serializer.clone().unwrap_or_default();
        let upload_in = self.upload_in.unwrap_or_default();

        let num_splits = split.len();
        let callback_ref = &progress_callback;
        let counter = AtomicUsize::new(0);
        let counter_ref = &counter;

        debug!("Uploading data to {}", self.url);
        Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                stream::iter(split)
                    .map(|(url, d)| {
                        let headers = headers.clone();
                        let creator = &creator;
                        let serializer = &serializer;
                        let upload_in = &upload_in;

                        debug!("Uploading next chunk");

                        async move {
                            creator
                                .get_builder(url)
                                .await?
                                .headers(headers)
                                .add_data(upload_in, serializer, d)?
                                .send()
                                .await
                                .map_err(|e| anyhow!(e.to_string()))?
                                .error_for_status()
                                .map_err(|e| anyhow!(e.to_string()))
                                .and_then(|res| {
                                    let current_count = counter_ref.fetch_add(1, Ordering::SeqCst);
                                    callback_ref(current_count, num_splits)?;
                                    debug!("Uploaded chunk {current_count}/{num_splits}");
                                    Ok(res)
                                })
                        }
                    })
                    .buffered(creator.num_parallel_requests)
                    .collect::<Vec<_>>()
                    .await
            })
            .into_iter()
            .try_for_each(|res| {
                res.and_then(|ok| {
                    if let Some(expected) = self.expected_status_code {
                        if ok.status() != expected {
                            return Err(anyhow!(
                                "Expected status code {}, got {}",
                                expected,
                                ok.status()
                            ));
                        }
                    }

                    Ok(())
                })
            })
    }
}
