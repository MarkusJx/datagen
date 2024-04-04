use crate::generate::datagen_context::{DatagenContext, DatagenContextRef};
#[cfg(feature = "map-schema")]
use crate::generate::generated_schema::GeneratedSchema;
#[cfg(feature = "map-schema")]
use crate::generate::resolved_reference::ResolvedReference;
#[cfg(feature = "map-schema")]
use crate::generate::schema_path::SchemaPath;
#[cfg(feature = "map-schema")]
use crate::generate::schema_value::SchemaProperties;
use crate::generate::schema_value::SchemaValue;
use crate::plugins::plugin::Plugin;
use crate::plugins::plugin_list::PluginList;
use crate::schema::schema_definition::SchemaOptions;
#[cfg(feature = "map-schema")]
use anyhow::anyhow;
#[cfg(feature = "generate")]
use std::collections::BTreeMap;
use std::sync::atomic::AtomicBool;
#[cfg(feature = "map-schema")]
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

#[cfg_attr(not(feature = "generate"), allow(dead_code))]
pub struct CurrentSchema {
    parent: Option<CurrentSchemaRef>,
    value: Arc<Mutex<SchemaValue>>,
    options: Arc<SchemaOptions>,
    plugins: Arc<PluginList>,
    finalized: AtomicBool,
}

unsafe impl Send for CurrentSchema {}
unsafe impl Sync for CurrentSchema {}

pub type CurrentSchemaRef = Arc<CurrentSchema>;

impl CurrentSchema {
    #[cfg(feature = "generate")]
    pub fn root(options: Arc<SchemaOptions>, plugins: Arc<PluginList>) -> CurrentSchemaRef {
        Self {
            parent: None,
            value: Arc::new(Mutex::new(SchemaValue {
                properties: Arc::new(Mutex::new(BTreeMap::new())),
                path: SchemaPath::root(),
            })),
            options,
            plugins,
            finalized: AtomicBool::default(),
        }
        .into()
    }

    #[cfg(feature = "map-schema")]
    pub fn child(
        parent: CurrentSchemaRef,
        sibling: Option<CurrentSchemaRef>,
        path: String,
    ) -> CurrentSchema {
        CurrentSchema {
            parent: Some(parent.clone()),
            value: Arc::new(Mutex::new(SchemaValue {
                properties: sibling
                    .map(|s| s.value.lock().unwrap().properties.clone())
                    .unwrap_or_default(),
                path: parent.value.lock().unwrap().path.append(path),
            })),
            options: parent.options.clone(),
            plugins: parent.plugins.clone(),
            finalized: AtomicBool::default(),
        }
    }

    #[cfg(feature = "map-schema")]
    fn get_global_properties(&self) -> Arc<Mutex<SchemaProperties>> {
        if let Some(parent) = self.parent.as_ref() {
            parent.get_global_properties()
        } else {
            self.value.lock().unwrap().properties.clone()
        }
    }

    #[cfg(feature = "map-schema")]
    fn get_all_schemas(props: &SchemaProperties, path: &str) -> ResolvedReference {
        if let Some(props) = props.get(path) {
            if props.len() == 1 {
                ResolvedReference::Single(props.front().unwrap().clone())
            } else {
                ResolvedReference::multiple(props.clone().into())
            }
        } else {
            ResolvedReference::none()
        }
    }

    #[cfg(feature = "map-schema")]
    fn resolve_child_ref(&self, reference: String) -> anyhow::Result<ResolvedReference> {
        if reference.starts_with("../") {
            self.parent
                .as_ref()
                .ok_or(anyhow!(
                    "The current schema at path '{}' has no parent",
                    self.value.lock().unwrap().path
                ))?
                .resolve_child_ref(reference.strip_prefix("../").unwrap().to_string())
        } else {
            let properties = &self.value.lock().unwrap().properties;
            let properties = properties.lock().unwrap();

            Ok(Self::get_all_schemas(&properties, &reference))
        }
    }

    #[cfg(feature = "map-schema")]
    pub fn resolve_ref(&self, reference: String) -> anyhow::Result<ResolvedReference> {
        if reference.starts_with("ref:") {
            let stripped = reference.strip_prefix("ref:").unwrap().to_string();
            if stripped.starts_with("./") {
                let properties = &self.value.lock().unwrap().properties;
                let properties = properties.lock().unwrap();

                Ok(Self::get_all_schemas(
                    &properties,
                    stripped.strip_prefix("./").unwrap(),
                ))
            } else if stripped.starts_with("../") {
                self.resolve_child_ref(stripped)
            } else {
                let global_properties = self.get_global_properties();
                let properties = global_properties.lock().unwrap();

                Ok(Self::get_all_schemas(&properties, &stripped))
            }
        } else {
            Ok(ResolvedReference::single(GeneratedSchema::String(
                reference,
            )))
        }
    }

    #[cfg(feature = "map-schema")]
    fn finalize_inner(&self, schema: Arc<GeneratedSchema>, path: &SchemaPath, remove: i32) {
        self.value.lock().unwrap().finalize(
            &self.options,
            schema.clone(),
            path.pop(remove).to_normalized_path(),
        );

        if let Some(parent) = self.parent.as_ref() {
            parent.finalize_inner(schema, path, remove - 1);
        }
    }

    #[cfg(feature = "map-schema")]
    pub fn finalize(&self, schema: Arc<GeneratedSchema>) -> Arc<GeneratedSchema> {
        if self.finalized.load(Ordering::SeqCst) {
            return schema;
        }

        let path = self.value.lock().unwrap().path().clone();
        self.finalize_inner(schema.clone(), &path, (path.len() as i32) - 1);
        self.finalized.store(true, Ordering::SeqCst);

        schema
    }

    #[cfg(feature = "map-schema")]
    pub fn path(&self) -> SchemaPath {
        self.value.lock().unwrap().path.clone()
    }

    pub fn get_plugin<'a>(&'a self, key: &String) -> anyhow::Result<&'a Arc<dyn Plugin>> {
        self.plugins.get(key)
    }

    #[allow(dead_code)]
    pub fn plugin_exists(&self, key: &String) -> bool {
        self.plugins.exists(key)
    }

    pub fn options(&self) -> &Arc<SchemaOptions> {
        &self.options
    }
}

impl DatagenContext for CurrentSchemaRef {
    fn child(
        &self,
        sibling: Option<Box<dyn DatagenContext>>,
        path: &str,
    ) -> anyhow::Result<Box<dyn DatagenContext>> {
        Ok(Box::new(Arc::new(CurrentSchema {
            parent: Some(self.clone()),
            value: Arc::new(Mutex::new(SchemaValue {
                properties: sibling
                    .map(|s| s.__schema_value_properties())
                    .map_or(Ok(None), |s| s.map(Some))?
                    .unwrap_or_default(),
                path: DatagenContext::path(self)?.append(path),
            })),
            options: self.options.clone(),
            plugins: self.plugins.clone(),
            finalized: AtomicBool::default(),
        })))
    }

    fn resolve_ref(&self, reference: &str) -> anyhow::Result<ResolvedReference> {
        CurrentSchema::resolve_ref(self.as_ref(), reference.to_string())
    }

    fn finalize(&self, schema: Arc<GeneratedSchema>) -> anyhow::Result<Arc<GeneratedSchema>> {
        Ok(CurrentSchema::finalize(self.as_ref(), schema))
    }

    fn path(&self) -> anyhow::Result<SchemaPath> {
        Ok(CurrentSchema::path(self.as_ref()))
    }

    fn get_plugin<'a>(&self, key: &str) -> anyhow::Result<Arc<dyn Plugin>> {
        CurrentSchema::get_plugin(self.as_ref(), &key.to_string()).cloned()
    }

    fn plugin_exists(&self, key: &str) -> anyhow::Result<bool> {
        Ok(CurrentSchema::plugin_exists(
            self.as_ref(),
            &key.to_string(),
        ))
    }

    fn options(&self) -> anyhow::Result<Arc<SchemaOptions>> {
        Ok(CurrentSchema::options(self.as_ref()).clone())
    }

    fn __schema_value_properties(&self) -> anyhow::Result<Arc<Mutex<SchemaProperties>>> {
        Ok(self.value.lock().unwrap().properties.clone())
    }
}

impl From<CurrentSchemaRef> for DatagenContextRef {
    fn from(schema: CurrentSchemaRef) -> DatagenContextRef {
        Box::new(schema)
    }
}
