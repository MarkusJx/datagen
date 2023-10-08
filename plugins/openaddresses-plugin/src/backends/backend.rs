use crate::objects::args::PluginArgs;
use crate::objects::geo_data::GeoFeature;
#[cfg(test)]
use std::any::Any;
use std::fmt::Debug;

pub(crate) trait Backend: Debug + Send + Sync {
    fn get_random_feature(&mut self) -> anyhow::Result<GeoFeature>;

    #[cfg(test)]
    fn as_any(&self) -> &dyn Any;

    #[cfg(test)]
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

pub(crate) trait BackendConstructor: Backend + Sized {
    fn new(paths: Vec<String>, args: PluginArgs) -> anyhow::Result<Self>;
}
