use crate::objects::address_type::AddressType;
use crate::objects::args::IntoGenerated;
use crate::objects::call_args::CallArgs;
use crate::objects::geo_data::GeoFeature;
use datagen_rs::generate::datagen_context::DatagenContextRef;
use datagen_rs::generate::generated_schema::GeneratedSchema;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub(crate) enum ArgValue {
    Args(CallArgs),
    Address(AddressType),
}

impl IntoGenerated for ArgValue {
    fn into_generated(
        self,
        schema: &DatagenContextRef,
        feature: &GeoFeature,
    ) -> anyhow::Result<Arc<GeneratedSchema>> {
        match self {
            ArgValue::Args(call_args) => call_args.into_generated(schema, feature),
            ArgValue::Address(address) => address.into_generated(schema, feature),
        }
    }
}
