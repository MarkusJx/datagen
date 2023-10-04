use crate::objects::geo_data::GeoFeature;
use datagen_rs::generate::current_schema::CurrentSchemaRef;
use datagen_rs::generate::generated_schema::GeneratedSchema;
use serde::Deserialize;
use std::sync::Arc;

pub(crate) trait IntoGenerated {
    fn into_generated(
        self,
        schema: &CurrentSchemaRef,
        feature: &GeoFeature,
    ) -> anyhow::Result<Arc<GeneratedSchema>>;
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PluginArgs {
    pub files: StringOrVec,
    pub backend: Option<BackendType>,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub(crate) enum BackendType {
    #[serde(rename = "sqlite", rename_all = "camelCase")]
    #[cfg_attr(not(feature = "sqlite"), allow(dead_code))]
    SQLite {
        database_name: String,
        batch_size: Option<usize>,
        cache_size: Option<u32>,
    },
    Memory,
}

impl Default for BackendType {
    fn default() -> Self {
        Self::Memory
    }
}

#[derive(Deserialize, Clone)]
#[serde(untagged, deny_unknown_fields)]
pub(crate) enum StringOrVec {
    Single(String),
    Multiple(Vec<String>),
}
