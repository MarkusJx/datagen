use crate::objects::args::PluginArgs;
use crate::objects::geo_data::GeoFeature;
use datagen_rs::util::types::Result;
use std::fmt::Debug;

pub(crate) trait Backend: Debug {
    fn get_random_feature(&mut self) -> Result<GeoFeature>;
}

pub(crate) trait BackendConstructor: Backend + Sized {
    fn new(paths: Vec<String>, args: PluginArgs) -> Result<Self>;
}
