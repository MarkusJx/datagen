use datagen_rs::generate::generated_schema::GeneratedSchema;
use datagen_rs::schema::serializer::Serializer;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Copy, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) enum UploadIn {
    #[default]
    Body,
    Query,
    Form,
}

pub(crate) trait AddData {
    fn add_data(
        self,
        upload_in: &UploadIn,
        serializer: &Serializer,
        data: Arc<GeneratedSchema>,
    ) -> anyhow::Result<RequestBuilder>;
}

impl AddData for RequestBuilder {
    fn add_data(
        self,
        upload_in: &UploadIn,
        serializer: &Serializer,
        data: Arc<GeneratedSchema>,
    ) -> anyhow::Result<RequestBuilder> {
        Ok(match upload_in {
            UploadIn::Body => self.body(serializer.serialize_generated(data, None)?),
            UploadIn::Query => self.query(&data),
            UploadIn::Form => self.form(&data),
        })
    }
}
