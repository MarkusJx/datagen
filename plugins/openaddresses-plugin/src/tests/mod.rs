use crate::objects::geo_data::{GeoFeature, Geometry};
use crate::OpenAddressesPlugin;
use datagen_rs::generate::current_schema::CurrentSchema;
use datagen_rs::generate::generated_schema::GeneratedSchema;
use datagen_rs::plugins::plugin::Plugin;
use datagen_rs::plugins::plugin_list::PluginList;
use datagen_rs::schema::schema_definition::SchemaOptions;
use ordered_float::OrderedFloat;
use serde_json::json;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;

mod memory;
#[cfg(feature = "sqlite")]
mod sqlite;

pub(crate) const ADDR_FILE_NAME: &str = "src/tests/addresses.geojson";

pub(crate) fn generate_random(plugin: &OpenAddressesPlugin) -> TestAddress {
    plugin
        .generate(
            CurrentSchema::root(SchemaOptions::default().into(), PluginList::empty().into()),
            json!({
                "number": "number",
                "street": "street",
                "city": "city",
                "postcode": "postcode",
                "latitude": "latitude",
                "longitude": "longitude",
                "hash": "hash"
            }),
        )
        .unwrap()
        .into()
}

macro_rules! assert_enum {
    ($enum:expr, $variant: path) => {
        match $enum {
            $variant(val) => val,
            _ => panic!("Expected {}, got {:?}", stringify!($variant), $enum),
        }
    };
    () => {};
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct TestAddress {
    pub number: String,
    pub street: String,
    pub city: String,
    pub postcode: String,
    pub latitude: OrderedFloat<f64>,
    pub longitude: OrderedFloat<f64>,
    pub hash: String,
}

impl From<GeoFeature> for TestAddress {
    fn from(value: GeoFeature) -> Self {
        Self {
            number: value.properties.number,
            street: value.properties.street,
            city: value.properties.city,
            postcode: value.properties.postcode,
            latitude: match value.geometry {
                Geometry::Point { coordinates } => coordinates[1].into(),
            },
            longitude: match value.geometry {
                Geometry::Point { coordinates } => coordinates[0].into(),
            },
            hash: value.properties.hash,
        }
    }
}

impl From<Arc<GeneratedSchema>> for TestAddress {
    fn from(value: Arc<GeneratedSchema>) -> Self {
        let obj = assert_enum!(value.as_ref(), GeneratedSchema::Object);

        Self {
            number: assert_enum!(obj.get("number").unwrap().as_ref(), GeneratedSchema::String)
                .clone(),
            street: assert_enum!(obj.get("street").unwrap().as_ref(), GeneratedSchema::String)
                .clone(),
            city: assert_enum!(obj.get("city").unwrap().as_ref(), GeneratedSchema::String).clone(),
            postcode: assert_enum!(
                obj.get("postcode").unwrap().as_ref(),
                GeneratedSchema::String
            )
            .clone(),
            latitude: *assert_enum!(
                obj.get("latitude").unwrap().as_ref(),
                GeneratedSchema::Number
            ),
            longitude: *assert_enum!(
                obj.get("longitude").unwrap().as_ref(),
                GeneratedSchema::Number
            ),
            hash: assert_enum!(obj.get("hash").unwrap().as_ref(), GeneratedSchema::String).clone(),
        }
    }
}

pub(crate) fn get_by_hash(hash: &str) -> TestAddress {
    BufReader::new(File::open(ADDR_FILE_NAME).unwrap())
        .lines()
        .map(|l| serde_json::from_str::<GeoFeature>(&l.unwrap()).unwrap())
        .map(TestAddress::from)
        .find(|a| a.hash == hash)
        .unwrap()
}
