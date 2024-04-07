use crate::generate::datagen_context::{DatagenContext, DatagenContextRef};
use crate::generate::generated_schema::GeneratedSchema;
use crate::generate::resolved_reference::ResolvedReference;
use crate::generate::schema_path::SchemaPath;
use crate::generate::schema_value::SchemaProperties;
use crate::plugins::abi::{
    CurrentSchemaAbi, CurrentSchemaAbiBox, CurrentSchemaAbi_TO, GeneratedSchemaAbi,
    GeneratedSchemaMapAbi, GeneratedSchemaMapAbiBox, GeneratedSchemaMapAbi_TO,
    GeneratedSchemaVecAbi, GeneratedSchemaVecAbiBox, GeneratedSchemaVecAbi_TO, IntoAnyhow,
    IntoPluginResult, JsonValue, PluginAbiBox, PluginAbi_TO, PluginResult, ResolvedReferenceAbi,
    SchemaPathAbi, SchemaPathAbiBox, SchemaPathAbi_TO, SerializeCallback, WrapResult,
};
use crate::plugins::plugin::{Plugin, PluginContainer};
use crate::schema::schema_definition::SchemaOptions;
use abi_stable::derive_macro_reexports::ROption;
use abi_stable::erased_types::TD_CanDowncast;
use abi_stable::std_types::{RString, RVec};
use anyhow::anyhow;
use indexmap::IndexMap;
use ordered_float::OrderedFloat;
use serde_json::Value;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct CurrentSchemaAbiImpl {
    inner: DatagenContextRef,
}

impl CurrentSchemaAbiImpl {
    pub fn from_datagen_context(inner: DatagenContextRef) -> CurrentSchemaAbiBox {
        CurrentSchemaAbi_TO::from_value(Self { inner }, TD_CanDowncast)
    }
}

impl CurrentSchemaAbi for CurrentSchemaAbiImpl {
    fn child(
        &self,
        sibling: ROption<CurrentSchemaAbiBox>,
        path: RString,
    ) -> PluginResult<CurrentSchemaAbiBox> {
        PluginResult::wrap(|| {
            Ok(CurrentSchemaAbiImpl::from_datagen_context(
                self.inner
                    .child(sibling.clone().map(Into::into).into_option(), path.as_str())?,
            ))
        })
    }

    fn resolve_ref(&self, reference: RString) -> PluginResult<ResolvedReferenceAbi> {
        PluginResult::wrap(|| {
            ResolvedReferenceAbi::try_from(
                self.inner
                    .resolve_ref(reference.as_str())
                    .map_err(|e| anyhow!("{}", e))?,
            )
        })
    }

    fn finalize(&self, schema: GeneratedSchemaAbi) -> PluginResult<GeneratedSchemaAbi> {
        PluginResult::wrap(|| self.inner.finalize(schema.clone().try_into()?)?.try_into())
    }

    fn path(&self) -> PluginResult<SchemaPathAbiBox> {
        self.inner.path().map(Into::into).into_plugin_result()
    }

    fn get_plugin(&self, key: RString) -> PluginResult<PluginAbiBox> {
        self.inner
            .get_plugin(key.as_str())
            .map(Into::into)
            .into_plugin_result()
    }

    fn plugin_exists(&self, key: RString) -> PluginResult<bool> {
        self.inner.plugin_exists(key.as_str()).into_plugin_result()
    }

    fn options(&self) -> PluginResult<JsonValue> {
        self.inner
            .options()
            .and_then(JsonValue::read_from)
            .map_err(Into::into)
            .into_plugin_result()
    }

    fn schema_value_properties(&self) -> PluginResult<JsonValue> {
        PluginResult::wrap(|| {
            let properties_mutex = self.inner.__schema_value_properties()?;
            let properties = properties_mutex.lock().map_err(|e| anyhow!("{}", e))?;
            JsonValue::read_from(properties.clone())
        })
    }
}

impl From<CurrentSchemaAbiBox> for DatagenContextRef {
    fn from(value: CurrentSchemaAbiBox) -> Self {
        Box::new(value)
    }
}

impl From<DatagenContextRef> for CurrentSchemaAbiBox {
    fn from(value: DatagenContextRef) -> Self {
        CurrentSchemaAbiImpl::from_datagen_context(value)
    }
}

impl DatagenContext for CurrentSchemaAbiBox {
    fn child(
        &self,
        sibling: Option<DatagenContextRef>,
        path: &str,
    ) -> anyhow::Result<DatagenContextRef> {
        CurrentSchemaAbiBox::child(
            self,
            ROption::from(sibling.map(CurrentSchemaAbiImpl::from_datagen_context)),
            RString::from(path),
        )
        .map(Into::into)
        .into_anyhow()
    }

    fn resolve_ref(&self, reference: &str) -> anyhow::Result<ResolvedReference> {
        CurrentSchemaAbiBox::resolve_ref(self, RString::from(reference))
            .into_anyhow()
            .and_then(TryInto::try_into)
    }

    fn finalize(&self, schema: Arc<GeneratedSchema>) -> anyhow::Result<Arc<GeneratedSchema>> {
        CurrentSchemaAbiBox::finalize(self, GeneratedSchemaAbi::try_from(schema)?)
            .into_anyhow()
            .and_then(TryInto::try_into)
    }

    fn path(&self) -> anyhow::Result<SchemaPath> {
        CurrentSchemaAbiBox::path(self)
            .into_anyhow()
            .map(Into::into)
    }

    fn get_plugin(&self, key: &str) -> anyhow::Result<Arc<dyn Plugin>> {
        CurrentSchemaAbiBox::get_plugin(self, RString::from(key))
            .into_anyhow()
            .map(Into::into)
    }

    fn plugin_exists(&self, key: &str) -> anyhow::Result<bool> {
        CurrentSchemaAbiBox::plugin_exists(self, RString::from(key)).into_anyhow()
    }

    fn options(&self) -> anyhow::Result<Arc<SchemaOptions>> {
        CurrentSchemaAbiBox::options(self)
            .into_anyhow()
            .and_then(|o| o.parse_into())
    }

    fn __schema_value_properties(&self) -> anyhow::Result<Arc<Mutex<SchemaProperties>>> {
        CurrentSchemaAbiBox::schema_value_properties(self)
            .into_anyhow()
            .and_then(|p| Ok(Arc::new(Mutex::new(p.parse_into()?))))
    }
}

pub struct SchemaPathAbiImpl {
    inner: SchemaPath,
}

impl SchemaPathAbiImpl {
    pub fn from_schema_path(inner: SchemaPath) -> SchemaPathAbiBox {
        SchemaPathAbi_TO::from_value(Self { inner }, TD_CanDowncast)
    }
}

impl SchemaPathAbi for SchemaPathAbiImpl {
    fn append(&self, path: RString) -> SchemaPathAbiBox {
        self.inner.append(path).into()
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn is_empty(&self) -> bool where {
        self.inner.is_empty()
    }

    fn pop(&self, num: i32) -> SchemaPathAbiBox {
        self.inner.pop(num).into()
    }

    fn as_normalized_path(&self) -> RString {
        self.inner.to_normalized_path().into()
    }

    fn parts(&self) -> RVec<RString> where {
        self.inner
            .0
            .iter()
            .map(|s| RString::from(s.as_str()))
            .collect()
    }
}

impl From<SchemaPath> for SchemaPathAbiBox {
    fn from(value: SchemaPath) -> Self {
        SchemaPathAbiImpl::from_schema_path(value)
    }
}

impl From<SchemaPathAbiBox> for SchemaPath {
    fn from(value: SchemaPathAbiBox) -> Self {
        SchemaPath(
            value
                .parts()
                .into_iter()
                .map(|s| s.into())
                .collect::<VecDeque<_>>(),
        )
    }
}

impl TryFrom<ResolvedReference> for ResolvedReferenceAbi {
    type Error = anyhow::Error;

    fn try_from(value: ResolvedReference) -> anyhow::Result<Self> {
        Ok(match value {
            ResolvedReference::Single(schema) => {
                ResolvedReferenceAbi::Single(GeneratedSchemaAbi::try_from(schema)?)
            }
            ResolvedReference::Multiple(schemas) => {
                ResolvedReferenceAbi::Multiple(GeneratedSchemaVecAbiImpl::from_schema_vec(schemas))
            }
            ResolvedReference::None => ResolvedReferenceAbi::None,
        })
    }
}

#[derive(Clone)]
pub struct GeneratedSchemaVecAbiImpl {
    inner: Vec<Arc<GeneratedSchema>>,
}

impl GeneratedSchemaVecAbiImpl {
    pub fn from_schema_vec(inner: Vec<Arc<GeneratedSchema>>) -> GeneratedSchemaVecAbiBox {
        GeneratedSchemaVecAbi_TO::from_value(Self { inner }, TD_CanDowncast)
    }

    pub fn new_boxed() -> GeneratedSchemaVecAbiBox {
        GeneratedSchemaVecAbi_TO::from_value(Self { inner: Vec::new() }, TD_CanDowncast)
    }
}

impl GeneratedSchemaVecAbi for GeneratedSchemaVecAbiImpl {
    fn push(&mut self, value: GeneratedSchemaAbi) {
        self.inner.push(value.try_into().unwrap());
    }

    fn into_vec(self) -> PluginResult<RVec<GeneratedSchemaAbi>> {
        self.inner
            .into_iter()
            .map(GeneratedSchemaAbi::try_from)
            .collect::<anyhow::Result<RVec<_>>>()
            .into_plugin_result()
    }
}

#[derive(Clone)]
pub struct GeneratedSchemaMapAbiImpl {
    inner: IndexMap<String, Arc<GeneratedSchema>>,
}

impl GeneratedSchemaMapAbiImpl {
    pub fn from_schema_map(
        inner: IndexMap<String, Arc<GeneratedSchema>>,
    ) -> GeneratedSchemaMapAbiBox {
        GeneratedSchemaMapAbi_TO::from_value(Self { inner }, TD_CanDowncast)
    }
}

impl GeneratedSchemaMapAbi for GeneratedSchemaMapAbiImpl {
    fn keys(&self) -> RVec<RString> {
        self.inner
            .keys()
            .map(|k| RString::from(k.clone()))
            .collect()
    }

    fn values(&self) -> PluginResult<RVec<GeneratedSchemaAbi>> {
        self.inner
            .values()
            .cloned()
            .map(GeneratedSchemaAbi::try_from)
            .collect::<anyhow::Result<RVec<_>>>()
            .into_plugin_result()
    }
}

impl TryFrom<Arc<GeneratedSchema>> for GeneratedSchemaAbi {
    type Error = anyhow::Error;

    fn try_from(value: Arc<GeneratedSchema>) -> anyhow::Result<Self> {
        (&value).try_into()
    }
}

impl TryFrom<&Arc<GeneratedSchema>> for GeneratedSchemaAbi {
    type Error = anyhow::Error;

    fn try_from(value: &Arc<GeneratedSchema>) -> anyhow::Result<Self> {
        Ok(match &**value {
            GeneratedSchema::None => GeneratedSchemaAbi::None,
            GeneratedSchema::Number(n) => GeneratedSchemaAbi::Number(n.into_inner()),
            GeneratedSchema::Integer(i) => GeneratedSchemaAbi::Integer(*i),
            GeneratedSchema::Bool(b) => GeneratedSchemaAbi::Bool(*b),
            GeneratedSchema::String(s) => GeneratedSchemaAbi::String(RString::from(s.clone())),
            GeneratedSchema::Array(a) => {
                GeneratedSchemaAbi::Array(GeneratedSchemaVecAbiImpl::from_schema_vec(a.clone()))
            }
            GeneratedSchema::Object(map) => {
                GeneratedSchemaAbi::Object(GeneratedSchemaMapAbiImpl::from_schema_map(map.clone()))
            }
            GeneratedSchema::Value(v) => {
                GeneratedSchemaAbi::Value(JsonValue::read_from(v.clone())?)
            }
        })
    }
}

impl TryFrom<GeneratedSchemaAbi> for Arc<GeneratedSchema> {
    type Error = anyhow::Error;

    fn try_from(value: GeneratedSchemaAbi) -> anyhow::Result<Self> {
        Ok(match value {
            GeneratedSchemaAbi::None => Arc::new(GeneratedSchema::None),
            GeneratedSchemaAbi::Number(n) => {
                Arc::new(GeneratedSchema::Number(OrderedFloat::from(n)))
            }
            GeneratedSchemaAbi::Integer(i) => Arc::new(GeneratedSchema::Integer(i)),
            GeneratedSchemaAbi::Bool(b) => Arc::new(GeneratedSchema::Bool(b)),
            GeneratedSchemaAbi::String(s) => Arc::new(GeneratedSchema::String(s.into())),
            GeneratedSchemaAbi::Array(a) => Arc::new(GeneratedSchema::Array(
                a.into_vec()
                    .into_anyhow()?
                    .into_iter()
                    .map(Arc::<GeneratedSchema>::try_from)
                    .collect::<anyhow::Result<Vec<_>>>()?,
            )),
            GeneratedSchemaAbi::Object(map) => Arc::new(GeneratedSchema::Object(
                map.keys()
                    .into_iter()
                    .zip(map.values().into_anyhow()?)
                    .map(|(k, v)| Ok((k.to_string(), Arc::<GeneratedSchema>::try_from(v)?)))
                    .collect::<anyhow::Result<IndexMap<_, _>>>()?,
            )),
            GeneratedSchemaAbi::Value(v) => Arc::new(GeneratedSchema::Value(v.parse_into()?)),
        })
    }
}

impl Plugin for PluginAbiBox {
    fn name(&self) -> String {
        PluginAbiBox::name(self).to_string()
    }

    fn generate(
        &self,
        schema: DatagenContextRef,
        args: Value,
    ) -> anyhow::Result<Arc<GeneratedSchema>> {
        PluginAbiBox::generate(self, schema.into(), JsonValue::read_from(args)?)
            .into_anyhow()
            .and_then(TryInto::try_into)
    }

    fn transform(
        &self,
        schema: DatagenContextRef,
        value: Arc<GeneratedSchema>,
        args: Value,
    ) -> anyhow::Result<Arc<GeneratedSchema>> {
        PluginAbiBox::transform(
            self,
            schema.into(),
            GeneratedSchemaAbi::try_from(value)?,
            JsonValue::read_from(args)?,
        )
        .into_anyhow()
        .and_then(TryInto::try_into)
    }

    fn serialize(&self, value: &Arc<GeneratedSchema>, args: Value) -> anyhow::Result<String> {
        PluginAbiBox::serialize(self, value.try_into()?, JsonValue::read_from(args)?)
            .map(Into::into)
            .into_anyhow()
    }

    fn serialize_with_progress(
        &self,
        value: &Arc<GeneratedSchema>,
        args: Value,
        _callback: &dyn Fn(usize, usize),
    ) -> anyhow::Result<String> {
        PluginAbiBox::serialize_with_progress(
            self,
            value.try_into()?,
            JsonValue::read_from(args)?,
            SerializeCallback { func: dummy },
        )
        .map(Into::into)
        .into_anyhow()
    }
}

extern "C" fn dummy(_current: usize, _total: usize) {
    todo!()
}

impl From<Arc<dyn Plugin>> for PluginAbiBox {
    fn from(value: Arc<dyn Plugin>) -> Self {
        PluginAbi_TO::from_value(PluginContainer::from_arc(value), TD_CanDowncast)
    }
}

impl From<PluginAbiBox> for Arc<dyn Plugin> {
    fn from(value: PluginAbiBox) -> Self {
        value.into()
    }
}
