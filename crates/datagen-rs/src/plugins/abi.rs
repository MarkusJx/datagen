use crate::generate::generated_schema::GeneratedSchema;
use crate::generate::resolved_reference::ResolvedReference;
use crate::plugins::abi_impl::{GeneratedSchemaMapAbiImpl, GeneratedSchemaVecAbiImpl};
use abi_stable::pmr::{ROption, RResult};
use abi_stable::std_types::vec::IntoIter;
use abi_stable::std_types::{RBox, RString, RVec};
use abi_stable::{sabi_trait, StableAbi};
use anyhow::anyhow;
use serde_json::Value;
use std::sync::Arc;

#[repr(C)]
#[derive(StableAbi)]
pub struct PluginError {
    message: ROption<RString>,
    call_stack: RVec<RString>,
}

pub type PluginResult<T> = RResult<T, PluginError>;

pub trait WrapResult<T> {
    fn wrap<F: Fn() -> anyhow::Result<T>>(f: F) -> PluginResult<T>;
}

pub trait IntoAnyhow<T> {
    fn into_anyhow(self) -> anyhow::Result<T>;
}

pub trait IntoPluginResult<T> {
    fn into_plugin_result(self) -> PluginResult<T>;
}

impl<T> WrapResult<T> for PluginResult<T> {
    fn wrap<F: FnOnce() -> anyhow::Result<T>>(f: F) -> Self {
        f().into_plugin_result()
    }
}

impl<T> IntoAnyhow<T> for PluginResult<T> {
    fn into_anyhow(self) -> anyhow::Result<T> {
        self.map_err(|e| {
            let mut res = anyhow::Error::msg(e.message.map(|m| m.to_string()).unwrap_or_default());
            for call in e.call_stack {
                res = res.context(call.to_string());
            }

            res
        })
        .into()
    }
}

impl<T> IntoPluginResult<T> for anyhow::Result<T> {
    fn into_plugin_result(self) -> PluginResult<T> {
        self.map_err(|e| PluginError {
            message: Some(e.to_string().into()).into(),
            call_stack: e.chain().map(|e| e.to_string().into()).collect(),
        })
        .into()
    }
}

#[repr(C)]
#[derive(StableAbi, Clone)]
pub struct JsonValue {
    value: RVec<u8>,
}

impl JsonValue {
    pub fn into_value(self) -> Value {
        serde_json::from_slice(&self.value)
            .map_err(|e| anyhow!("{}", e))
            .unwrap()
    }

    pub fn from_value(value: Value) -> Self {
        serde_json::to_vec(&value)
            .map(|value| Self {
                value: value.into(),
            })
            .map_err(|e| anyhow!("{}", e))
            .unwrap()
    }

    pub fn as_value(&self) -> Value {
        serde_json::from_slice(&self.value)
            .map_err(|e| anyhow!("{}", e))
            .unwrap()
    }
}

impl From<Value> for JsonValue {
    fn from(value: Value) -> Self {
        Self::from_value(value)
    }
}

impl From<JsonValue> for Value {
    fn from(value: JsonValue) -> Self {
        value.into_value()
    }
}

#[sabi_trait]
pub trait PluginAbi {
    fn name(&self) -> RString;

    fn generate(
        &self,
        schema: CurrentSchemaAbiBox,
        args: JsonValue,
    ) -> PluginResult<GeneratedSchemaAbi>;

    fn transform(
        &self,
        schema: CurrentSchemaAbiBox,
        value: GeneratedSchemaAbi,
        args: JsonValue,
    ) -> PluginResult<GeneratedSchemaAbi>;

    fn serialize(&self, value: GeneratedSchemaAbi, args: JsonValue) -> PluginResult<RString>;

    #[allow(unused_variables)]
    fn serialize_with_progress(
        &self,
        value: GeneratedSchemaAbi,
        args: JsonValue,
        //callback: Fn,
    ) -> PluginResult<RString> {
        self.serialize(value, args)
    }
}

pub type PluginAbiBox = PluginAbi_TO<'static, RBox<()>>;

#[sabi_trait]
pub trait CurrentSchemaAbi: Clone {
    fn child(
        &self,
        sibling: ROption<CurrentSchemaAbiBox>,
        path: RString,
    ) -> PluginResult<CurrentSchemaAbiBox>;

    fn resolve_ref(&self, reference: RString) -> PluginResult<ResolvedReferenceAbi>;

    fn finalize(&self, schema: GeneratedSchemaAbi) -> GeneratedSchemaAbi;

    fn path(&self) -> RString;
}

pub type CurrentSchemaAbiBox = CurrentSchemaAbi_TO<'static, RBox<()>>;

#[repr(C)]
#[derive(StableAbi, Clone)]
pub enum GeneratedSchemaAbi {
    None,
    Number(f64),
    Integer(i64),
    Bool(bool),
    String(RString),
    Array(GeneratedSchemaVecAbiBox),
    Object(GeneratedSchemaMapAbiBox),
    Value(JsonValue),
}

#[sabi_trait]
pub trait GeneratedSchemaVecAbi: Clone {
    fn push(&mut self, value: GeneratedSchemaAbi);

    fn insert(&mut self, index: usize, value: GeneratedSchemaAbi);

    fn pop(&mut self) -> ROption<GeneratedSchemaAbi>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn into_vec(self) -> RVec<GeneratedSchemaAbi>;
}

impl IntoIterator for GeneratedSchemaVecAbiBox {
    type Item = GeneratedSchemaAbi;
    type IntoIter = IntoIter<GeneratedSchemaAbi>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().into_iter()
    }
}

impl FromIterator<GeneratedSchemaAbi> for GeneratedSchemaVecAbiBox {
    fn from_iter<I: IntoIterator<Item = GeneratedSchemaAbi>>(iter: I) -> Self {
        let mut vec = GeneratedSchemaVecAbiImpl::new_boxed();
        for item in iter {
            vec.push(item);
        }
        vec
    }
}

impl From<GeneratedSchemaVecAbiBox> for Vec<GeneratedSchemaAbi> {
    fn from(value: GeneratedSchemaVecAbiBox) -> Self {
        value.into_vec().into()
    }
}

pub type GeneratedSchemaVecAbiBox = GeneratedSchemaVecAbi_TO<'static, RBox<()>>;

#[sabi_trait]
pub trait GeneratedSchemaMapAbi: Clone {
    fn get(&self, key: RString) -> ROption<GeneratedSchemaAbi>;

    fn insert(&mut self, key: RString, value: GeneratedSchemaAbi);

    fn remove(&mut self, key: RString) -> ROption<GeneratedSchemaAbi>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn keys(&self) -> RVec<RString>;

    fn values(&self) -> RVec<GeneratedSchemaAbi>;
}

impl IntoIterator for GeneratedSchemaMapAbiBox {
    type Item = (RString, GeneratedSchemaAbi);
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.keys()
            .into_iter()
            .zip(self.values())
            .collect::<RVec<_>>()
            .into_iter()
    }
}

impl FromIterator<(String, GeneratedSchemaAbi)> for GeneratedSchemaMapAbiBox {
    fn from_iter<I: IntoIterator<Item = (String, GeneratedSchemaAbi)>>(iter: I) -> Self {
        GeneratedSchemaMapAbiBox::from_iter(iter.into_iter().map(|(k, v)| (RString::from(k), v)))
    }
}

impl FromIterator<(RString, GeneratedSchemaAbi)> for GeneratedSchemaMapAbiBox {
    fn from_iter<I: IntoIterator<Item = (RString, GeneratedSchemaAbi)>>(iter: I) -> Self {
        let mut map = GeneratedSchemaMapAbiImpl::new_boxed();
        for (key, value) in iter {
            map.insert(key, value);
        }
        map
    }
}

impl FromIterator<(String, Arc<GeneratedSchema>)> for GeneratedSchemaMapAbiBox {
    fn from_iter<I: IntoIterator<Item = (String, Arc<GeneratedSchema>)>>(iter: I) -> Self {
        GeneratedSchemaMapAbiBox::from_iter(
            iter.into_iter().map(|(k, v)| (RString::from(k), v.into())),
        )
    }
}

pub type GeneratedSchemaMapAbiBox = GeneratedSchemaMapAbi_TO<'static, RBox<()>>;

#[repr(C)]
#[derive(StableAbi)]
pub enum ResolvedReferenceAbi {
    Single(GeneratedSchemaAbi),
    Multiple(GeneratedSchemaVecAbiBox),
    None,
}

impl From<ResolvedReferenceAbi> for ResolvedReference {
    fn from(value: ResolvedReferenceAbi) -> Self {
        match value {
            ResolvedReferenceAbi::Single(schema) => ResolvedReference::Single(schema.into()),
            ResolvedReferenceAbi::Multiple(schemas) => {
                ResolvedReference::multiple(schemas.into_iter().map(Into::into).collect())
            }
            ResolvedReferenceAbi::None => ResolvedReference::none(),
        }
    }
}
