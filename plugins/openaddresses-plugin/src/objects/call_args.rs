use crate::objects::arg_value::ArgValue;
use crate::objects::args::IntoGenerated;
use crate::objects::geo_data::GeoFeature;
use datagen_rs::generate::datagen_context::DatagenContextRef;
use datagen_rs::generate::generated_schema::GeneratedSchema;
use datagen_rs::generate::schema_mapper::MapSchema;
use indexmap::IndexMap;
use std::sync::Arc;

pub(crate) type CallArgs = IndexMap<String, ArgValue>;

impl IntoGenerated for CallArgs {
    fn into_generated(
        self,
        schema: &DatagenContextRef,
        feature: &GeoFeature,
    ) -> anyhow::Result<Arc<GeneratedSchema>> {
        schema
            .map_index_map(self, None, true, |schema, value| {
                value.into_generated(schema, feature)
            })
            .map(Into::into)
    }
}
