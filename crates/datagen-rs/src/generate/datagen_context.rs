use crate::generate::generated_schema::GeneratedSchema;
use crate::generate::resolved_reference::ResolvedReference;
use crate::generate::schema_path::SchemaPath;
use crate::generate::schema_value::SchemaProperties;
use crate::plugins::plugin::Plugin;
use crate::schema::schema_definition::SchemaOptions;
use dyn_clone::{clone_trait_object, DynClone};
use std::sync::{Arc, Mutex};

pub trait DatagenContext: DynClone + Send + Sync {
    fn child(
        &self,
        sibling: Option<DatagenContextRef>,
        path: &str,
    ) -> anyhow::Result<DatagenContextRef>;

    fn resolve_ref(&self, reference: &str) -> anyhow::Result<ResolvedReference>;

    fn finalize(&self, schema: Arc<GeneratedSchema>) -> anyhow::Result<Arc<GeneratedSchema>>;

    fn path(&self) -> anyhow::Result<SchemaPath>;

    fn get_plugin(&self, key: &str) -> anyhow::Result<Arc<dyn Plugin>>;

    fn plugin_exists(&self, key: &str) -> anyhow::Result<bool>;

    fn options(&self) -> anyhow::Result<Arc<SchemaOptions>>;

    #[doc(hidden)]
    fn __schema_value_properties(&self) -> anyhow::Result<Arc<Mutex<SchemaProperties>>>;
}

clone_trait_object!(DatagenContext);

pub type DatagenContextRef = Box<dyn DatagenContext>;
