use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct FeatureProperties {
    pub hash: String,
    pub number: String,
    pub street: String,
    pub unit: String,
    pub city: String,
    pub district: String,
    pub region: String,
    pub postcode: String,
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub(crate) enum Geometry {
    Point { coordinates: [f64; 2] },
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct GeoFeature {
    pub properties: FeatureProperties,
    pub geometry: Geometry,
}
