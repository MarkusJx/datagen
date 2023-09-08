use crate::objects::arg_value::ArgValue;
use crate::objects::args::IntoGenerated;
use crate::objects::geo_data::GeoFeature;
use datagen_rs::generate::current_schema::CurrentSchema;
use datagen_rs::generate::generated_schema::GeneratedSchema;
use datagen_rs::generate::schema_mapper::MapSchema;
use datagen_rs::util::types::Result;
use indexmap::IndexMap;
use std::sync::Arc;

pub(crate) type CallArgs = IndexMap<String, ArgValue>;

impl IntoGenerated for CallArgs {
    fn into_generated(
        self,
        schema: &Arc<CurrentSchema>,
        feature: &GeoFeature,
    ) -> Result<Arc<GeneratedSchema>> {
        schema
            .map_index_map(self, None, true, |schema, value| {
                value.into_generated(schema, feature)
            })
            .map(Into::into)
    }
}
