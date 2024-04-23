use crate::bail_unsupported;
use crate::generate::datagen_context::{DatagenContext, DatagenContextRef};
use crate::generate::generated_schema::GeneratedSchema;
use crate::generate::resolved_reference::ResolvedReference;
use crate::generate::schema_path::SchemaPath;
use crate::generate::schema_value::SchemaProperties;
use crate::plugins::plugin::Plugin;
use crate::schema::schema_definition::SchemaOptions;
use mockall::automock;
use std::any::Any;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub(crate) struct Context {
    pub mock_context: Arc<Mutex<MockContext>>,
}

#[automock]
#[allow(unused)]
impl Context {
    pub fn child(
        &self,
        _sibling: Option<DatagenContextRef>,
        _path: &str,
    ) -> anyhow::Result<DatagenContextRef> {
        bail_unsupported!("")
    }

    pub fn resolve_ref(&self, _reference: &str) -> anyhow::Result<ResolvedReference> {
        bail_unsupported!("")
    }

    pub fn finalize(&self, _schema: Arc<GeneratedSchema>) -> anyhow::Result<Arc<GeneratedSchema>> {
        bail_unsupported!("")
    }

    pub fn path(&self) -> anyhow::Result<SchemaPath> {
        bail_unsupported!("")
    }

    pub fn get_plugin(&self, _key: &str) -> anyhow::Result<Arc<dyn Plugin>> {
        bail_unsupported!("")
    }

    pub fn plugin_exists(&self, _key: &str) -> anyhow::Result<bool> {
        bail_unsupported!("")
    }

    pub fn options(&self) -> anyhow::Result<Arc<SchemaOptions>> {
        bail_unsupported!("")
    }

    #[allow(non_snake_case)]
    pub fn __schema_value_properties(&self) -> anyhow::Result<Arc<Mutex<SchemaProperties>>> {
        bail_unsupported!("")
    }
}

impl DatagenContext for Context {
    fn child(
        &self,
        sibling: Option<DatagenContextRef>,
        path: &str,
    ) -> anyhow::Result<DatagenContextRef> {
        self.mock_context.lock().unwrap().child(sibling, path)
    }

    fn resolve_ref(&self, reference: &str) -> anyhow::Result<ResolvedReference> {
        self.mock_context.lock().unwrap().resolve_ref(reference)
    }

    fn finalize(&self, schema: Arc<GeneratedSchema>) -> anyhow::Result<Arc<GeneratedSchema>> {
        self.mock_context.lock().unwrap().finalize(schema)
    }

    fn path(&self) -> anyhow::Result<SchemaPath> {
        self.mock_context.lock().unwrap().path()
    }

    fn get_plugin(&self, key: &str) -> anyhow::Result<Arc<dyn Plugin>> {
        self.mock_context.lock().unwrap().get_plugin(key)
    }

    fn plugin_exists(&self, key: &str) -> anyhow::Result<bool> {
        self.mock_context.lock().unwrap().plugin_exists(key)
    }

    fn options(&self) -> anyhow::Result<Arc<SchemaOptions>> {
        self.mock_context.lock().unwrap().options()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn __schema_value_properties(&self) -> anyhow::Result<Arc<Mutex<SchemaProperties>>> {
        self.mock_context
            .lock()
            .unwrap()
            .__schema_value_properties()
    }
}

impl From<MockContext> for DatagenContextRef {
    fn from(mock_context: MockContext) -> Self {
        Box::new(Context {
            mock_context: Arc::new(Mutex::new(mock_context)),
        })
    }
}
