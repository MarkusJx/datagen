use crate::objects::args::IntoGenerated;
use crate::objects::geo_data::{GeoFeature, Geometry};
use datagen_rs::generate::current_schema::CurrentSchemaRef;
use datagen_rs::generate::generated_schema::GeneratedSchema;
use datagen_rs::util::types::Result;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum AddressType {
    Number,
    Street,
    City,
    Unit,
    District,
    Region,
    Postcode,
    Latitude,
    Longitude,
    Hash,
}

impl IntoGenerated for AddressType {
    fn into_generated(
        self,
        _schema: &CurrentSchemaRef,
        feature: &GeoFeature,
    ) -> Result<Arc<GeneratedSchema>> {
        Ok(match self {
            AddressType::Number => GeneratedSchema::String(feature.properties.number.clone()),
            AddressType::Street => GeneratedSchema::String(feature.properties.street.clone()),
            AddressType::City => GeneratedSchema::String(feature.properties.city.clone()),
            AddressType::Unit => GeneratedSchema::String(feature.properties.unit.clone()),
            AddressType::District => GeneratedSchema::String(feature.properties.district.clone()),
            AddressType::Region => GeneratedSchema::String(feature.properties.region.clone()),
            AddressType::Postcode => GeneratedSchema::String(feature.properties.postcode.clone()),
            AddressType::Latitude => GeneratedSchema::Number(match feature.geometry {
                Geometry::Point { coordinates } => coordinates[1].into(),
            }),
            AddressType::Longitude => GeneratedSchema::Number(match feature.geometry {
                Geometry::Point { coordinates } => coordinates[0].into(),
            }),
            AddressType::Hash => GeneratedSchema::String(feature.properties.hash.clone()),
        }
        .into())
    }
}
