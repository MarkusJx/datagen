#[cfg(feature = "generate")]
use crate::generate::generated_schema::GeneratedSchema;
#[cfg(feature = "generate")]
use crate::generate::resolved_reference::ResolvedReference;
#[cfg(feature = "generate")]
use crate::generate::schema_path::SchemaPath;
#[cfg(feature = "generate")]
use crate::generate::schema_value::SchemaProperties;
use crate::generate::schema_value::SchemaValue;
use crate::plugins::plugin::Plugin;
use crate::plugins::plugin_list::PluginList;
use crate::schema::schema_definition::SchemaOptions;
use crate::util::types::Result;
#[cfg(feature = "generate")]
use std::collections::BTreeMap;
#[cfg(not(feature = "send"))]
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
#[cfg_attr(not(feature = "generate"), allow(dead_code))]
pub struct CurrentSchema {
    parent: Option<CurrentSchemaRef>,
    pub(crate) value: Arc<Mutex<SchemaValue>>,
    options: Arc<SchemaOptions>,
    plugins: Arc<PluginList>,
    finalized: AtomicBool,
}

#[cfg(feature = "send")]
unsafe impl Send for CurrentSchema {}
#[cfg(feature = "send")]
unsafe impl Sync for CurrentSchema {}

#[cfg(feature = "send")]
pub type CurrentSchemaRef = Arc<CurrentSchema>;
#[cfg(not(feature = "send"))]
pub type CurrentSchemaRef = Rc<CurrentSchema>;

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

    #[cfg(feature = "generate")]
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

    #[cfg(feature = "generate")]
    fn get_global_properties(&self) -> Arc<Mutex<SchemaProperties>> {
        if let Some(parent) = self.parent.as_ref() {
            parent.get_global_properties()
        } else {
            self.value.lock().unwrap().properties.clone()
        }
    }

    #[cfg(feature = "generate")]
    fn get_all_schemas(props: &SchemaProperties, path: &str) -> ResolvedReference {
        if let Some(props) = props.get(path) {
            if props.len() == 1 {
                ResolvedReference::Single(props.get(0).unwrap().clone())
            } else {
                ResolvedReference::multiple(props.clone().into())
            }
        } else {
            ResolvedReference::none()
        }
    }

    #[cfg(feature = "generate")]
    fn resolve_child_ref(&self, reference: String) -> Result<ResolvedReference> {
        if reference.starts_with("../") {
            self.parent
                .as_ref()
                .ok_or(format!(
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

    #[cfg(feature = "generate")]
    pub fn resolve_ref(&self, reference: String) -> Result<ResolvedReference> {
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

    #[cfg(feature = "generate")]
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

    #[cfg(feature = "generate")]
    pub fn finalize(&self, schema: Arc<GeneratedSchema>) -> Arc<GeneratedSchema> {
        if self.finalized.load(Ordering::SeqCst) {
            return schema;
        }

        let path = self.value.lock().unwrap().path().clone();
        self.finalize_inner(schema.clone(), &path, (path.len() as i32) - 1);
        self.finalized.store(true, Ordering::SeqCst);

        schema
    }

    pub fn get_plugin<'a>(&'a self, key: &String) -> Result<&'a dyn Plugin> {
        self.plugins.get(key)
    }

    pub fn options(&self) -> &Arc<SchemaOptions> {
        &self.options
    }
}
