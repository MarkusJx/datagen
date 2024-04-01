use crate::generate::current_schema::{CurrentSchema, CurrentSchemaRef};
use crate::generate::generated_schema::GeneratedSchema;
use crate::generate::resolved_reference::ResolvedReference;
use crate::plugins::abi::{
    CurrentSchemaAbi, CurrentSchemaAbiBox, CurrentSchemaAbi_TO, GeneratedSchemaAbi,
    GeneratedSchemaMapAbi, GeneratedSchemaMapAbiBox, GeneratedSchemaMapAbi_TO,
    GeneratedSchemaVecAbi, GeneratedSchemaVecAbiBox, GeneratedSchemaVecAbi_TO, JsonValue,
    PluginResult, ResolvedReferenceAbi, WrapResult,
};
use abi_stable::derive_macro_reexports::ROption;
use abi_stable::erased_types::TD_CanDowncast;
use abi_stable::std_types::{RString, RVec};
use anyhow::anyhow;
use indexmap::IndexMap;
use ordered_float::OrderedFloat;
use std::any::Any;
use std::sync::Arc;

#[derive(Clone)]
pub struct CurrentSchemaAbiImpl {
    inner: CurrentSchemaRef,
}

impl CurrentSchemaAbiImpl {
    pub fn from_schema(inner: CurrentSchemaRef) -> CurrentSchemaAbiBox {
        CurrentSchemaAbi_TO::from_value(Self { inner }, TD_CanDowncast)
    }

    pub fn inner(&self) -> &CurrentSchemaRef {
        &self.inner
    }
}

impl CurrentSchemaAbi for CurrentSchemaAbiImpl {
    fn child(
        &self,
        sibling: ROption<CurrentSchemaAbiBox>,
        path: RString,
    ) -> PluginResult<CurrentSchemaAbiBox> {
        PluginResult::wrap(|| {
            Ok(CurrentSchemaAbiImpl::from_schema(
                CurrentSchema::child(
                    self.inner.clone(),
                    sibling
                        .clone()
                        .map(|s| -> anyhow::Result<_> {
                            Ok(s.obj
                                .downcast_as::<CurrentSchemaAbiImpl>()
                                .map_err(|e| anyhow!("{e}"))?
                                .inner()
                                .clone())
                        })
                        .into_option()
                        .map_or(Ok(None), |s| s.map(Some))?,
                    path.to_string(),
                )
                .into(),
            ))
        })
    }

    fn resolve_ref(&self, reference: RString) -> PluginResult<ResolvedReferenceAbi> {
        PluginResult::wrap(|| {
            Ok(ResolvedReferenceAbi::from(
                self.inner
                    .resolve_ref(reference.to_string())
                    .map_err(|e| anyhow!("{}", e))?,
            ))
        })
    }

    fn finalize(&self, schema: GeneratedSchemaAbi) -> GeneratedSchemaAbi {
        self.inner
            .finalize(Arc::<GeneratedSchema>::from(schema))
            .into()
    }

    fn path(&self) -> RString {
        self.inner.path().to_string().into()
    }
}

impl crate::plugins::plugin::ICurrentSchema for CurrentSchemaAbiBox {
    fn _inner_abi(&self) -> &CurrentSchemaAbiBox {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn original_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<CurrentSchemaAbiBox>()
    }
}

pub trait IntoCurrentSchemaAbi {
    fn as_current_schema_abi(&self) -> Box<dyn crate::plugins::plugin::ICurrentSchema>;
}

impl IntoCurrentSchemaAbi for CurrentSchemaRef {
    fn as_current_schema_abi(&self) -> Box<dyn crate::plugins::plugin::ICurrentSchema> {
        Box::new(CurrentSchemaAbiImpl::from_schema(self.clone()))
    }
}

impl From<ResolvedReference> for ResolvedReferenceAbi {
    fn from(value: ResolvedReference) -> Self {
        match value {
            ResolvedReference::Single(schema) => {
                ResolvedReferenceAbi::Single(GeneratedSchemaAbi::from(schema))
            }
            ResolvedReference::Multiple(schemas) => {
                ResolvedReferenceAbi::Multiple(GeneratedSchemaVecAbiImpl::from_schema_vec(schemas))
            }
            ResolvedReference::None => ResolvedReferenceAbi::None,
        }
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
        self.inner.push(Arc::<GeneratedSchema>::from(value));
    }

    fn insert(&mut self, index: usize, value: GeneratedSchemaAbi) {
        self.inner
            .insert(index, Arc::<GeneratedSchema>::from(value));
    }

    fn pop(&mut self) -> ROption<GeneratedSchemaAbi> {
        self.inner.pop().map(GeneratedSchemaAbi::from).into()
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn into_vec(self) -> RVec<GeneratedSchemaAbi>
    where
        Self: Sized,
    {
        self.inner
            .into_iter()
            .map(GeneratedSchemaAbi::from)
            .collect()
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

    pub fn new_boxed() -> GeneratedSchemaMapAbiBox {
        GeneratedSchemaMapAbi_TO::from_value(
            Self {
                inner: IndexMap::new(),
            },
            TD_CanDowncast,
        )
    }
}

impl GeneratedSchemaMapAbi for GeneratedSchemaMapAbiImpl {
    fn get(&self, key: RString) -> ROption<GeneratedSchemaAbi> {
        self.inner
            .get(&key.to_string())
            .cloned()
            .map(GeneratedSchemaAbi::from)
            .into()
    }

    fn insert(&mut self, key: RString, value: GeneratedSchemaAbi) {
        self.inner
            .insert(key.into(), Arc::<GeneratedSchema>::from(value));
    }

    fn remove(&mut self, key: RString) -> ROption<GeneratedSchemaAbi> {
        self.inner
            .shift_remove(&key.to_string())
            .map(GeneratedSchemaAbi::from)
            .into()
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn keys(&self) -> RVec<RString> where {
        self.inner
            .keys()
            .map(|k| RString::from(k.clone()))
            .collect()
    }

    fn values(&self) -> RVec<GeneratedSchemaAbi> where {
        self.inner
            .values()
            .cloned()
            .map(GeneratedSchemaAbi::from)
            .collect()
    }
}

impl From<Arc<GeneratedSchema>> for GeneratedSchemaAbi {
    fn from(value: Arc<GeneratedSchema>) -> Self {
        (&value).into()
    }
}

impl From<&Arc<GeneratedSchema>> for GeneratedSchemaAbi {
    fn from(value: &Arc<GeneratedSchema>) -> Self {
        match &**value {
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
                GeneratedSchemaAbi::Value(JsonValue::from_value(v.clone()))
            }
        }
    }
}

impl From<GeneratedSchemaAbi> for Arc<GeneratedSchema> {
    fn from(value: GeneratedSchemaAbi) -> Self {
        match value {
            GeneratedSchemaAbi::None => Arc::new(GeneratedSchema::None),
            GeneratedSchemaAbi::Number(n) => {
                Arc::new(GeneratedSchema::Number(OrderedFloat::from(n)))
            }
            GeneratedSchemaAbi::Integer(i) => Arc::new(GeneratedSchema::Integer(i)),
            GeneratedSchemaAbi::Bool(b) => Arc::new(GeneratedSchema::Bool(b)),
            GeneratedSchemaAbi::String(s) => Arc::new(GeneratedSchema::String(s.into())),
            GeneratedSchemaAbi::Array(a) => Arc::new(GeneratedSchema::Array(
                a.into_vec()
                    .into_iter()
                    .map(Arc::<GeneratedSchema>::from)
                    .collect::<Vec<_>>(),
            )),
            GeneratedSchemaAbi::Object(map) => Arc::new(GeneratedSchema::Object(
                map.into_iter()
                    .map(|(k, v)| (k.to_string(), Arc::<GeneratedSchema>::from(v)))
                    .collect::<IndexMap<_, _>>(),
            )),
            GeneratedSchemaAbi::Value(v) => Arc::new(GeneratedSchema::Value(v.into_value())),
        }
    }
}
