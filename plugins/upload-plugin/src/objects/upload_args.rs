use crate::auth::authentication::{AnyAuth, Authentication, NoAuth};
use crate::objects::auth_args::AuthArgs;
use crate::objects::http_method::HttpMethod;
use datagen_rs::generate::generated_schema::GeneratedSchema;
use datagen_rs::schema::serializer::Serializer;
use datagen_rs::util::types::{AnyError, Result};
use futures::{stream, StreamExt};
use reqwest::header::HeaderMap;
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time;
use time::Duration;
use tokio::runtime::Builder;

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
    pub url: String,
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
}

struct RequestCreator {
    method: HttpMethod,
    url: String,
    timeout: Option<Duration>,
    client: Client,
    auth: Box<dyn Authentication>,
    num_parallel_requests: usize,
}

impl From<&UploadArgs> for RequestCreator {
    fn from(args: &UploadArgs) -> Self {
        Self {
            method: args.method.clone().unwrap_or_default(),
            url: args.url.clone(),
            timeout: args.timeout.map(Duration::from_millis),
            client: Client::new(),
            auth: NoAuth::from_args(args.auth.clone()),
            num_parallel_requests: args.num_parallel_requests.unwrap_or(1),
        }
    }
}

impl RequestCreator {
    async fn get_builder(&self) -> Result<RequestBuilder> {
        let mut builder = self
            .method
            .get_builder(&self.client, self.url.clone())
            .add_auth(self.auth.as_ref())
            .await?;

        if let Some(timeout) = self.timeout {
            builder = builder.timeout(timeout);
        }

        Ok(builder)
    }
}

impl UploadArgs {
    fn get_headers(&self) -> Result<HeaderMap> {
        let content_type = match self.serializer.as_ref().unwrap_or_default() {
            Serializer::Json { .. } => "application/json",
            Serializer::Yaml { .. } => "application/yaml",
            Serializer::Xml { .. } => "application/xml",
            _ => return Err("Unsupported serializer".into()),
        };

        let mut map = HeaderMap::new();
        map.insert("Content-Type", content_type.parse()?);

        Ok(map)
    }

    fn split_and_serialize(&self, value: &Arc<GeneratedSchema>) -> Result<Vec<String>> {
        let serializer = self.serializer.as_ref().unwrap_or_default();
        if self.split_top_level_array.unwrap_or_default() {
            if let GeneratedSchema::Array(arr) = value.as_ref() {
                return arr
                    .iter()
                    .cloned()
                    .map(|v| serializer.serialize_generated(v, None))
                    .collect();
            }
        }

        Ok(vec![serializer.serialize_generated(value.clone(), None)?])
    }

    pub(crate) fn serialize_generated(&self, value: &Arc<GeneratedSchema>) -> Result<String> {
        self.serializer
            .as_ref()
            .unwrap_or_default()
            .serialize_generated(value.clone(), None)
    }

    pub(crate) fn upload_data(&self, value: &Arc<GeneratedSchema>) -> Result<()> {
        let headers = self.get_headers()?;
        let split = self.split_and_serialize(value)?;
        let creator = RequestCreator::from(self);

        Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                stream::iter(split)
                    .map(|d| {
                        let headers = headers.clone();
                        let creator = &creator;

                        async move {
                            creator
                                .get_builder()
                                .await?
                                .headers(headers)
                                .body(d)
                                .send()
                                .await
                                .map_err(|e| AnyError::from(e.to_string()))?
                                .error_for_status()
                                .map_err(|e| AnyError::from(e.to_string()))
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
                            return Err(format!(
                                "Expected status code {}, got {}",
                                expected,
                                ok.status()
                            )
                            .into());
                        }
                    }

                    Ok(())
                })
            })
    }
}
