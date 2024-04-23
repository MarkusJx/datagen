#![allow(clippy::unnecessary_cast)]

use crate::generate::resolved_reference::ResolvedReference;
use crate::plugins::abi_impl::GeneratedSchemaVecAbiImpl;
use crate::plugins::plugin::PluginSerializeCallback;
use abi_stable::pmr::{ROption, RResult};
use abi_stable::std_types::{RArc, RBox, RString, RVec};
use abi_stable::{sabi_trait, StableAbi};
use anyhow::bail;
use app_state::{stateful, AppStateTrait, MutAppState, MutAppStateLock};
use serde::{Deserialize, Serialize};

/// A C ABI compatible plugin error.
#[repr(C)]
#[derive(StableAbi)]
pub struct PluginError {
    message: ROption<RString>,
    call_stack: RVec<RString>,
}

/// A C ABI compatible plugin result.
pub type PluginResult<T> = RResult<T, PluginError>;

/// Wraps a function that returns a [`Result`] into a [`PluginResult`].
pub trait WrapResult<T> {
    /// Wrap a function that returns a [`Result`] into a [`PluginResult`].
    fn wrap<F: Fn() -> anyhow::Result<T>>(f: F) -> PluginResult<T>;
}

/// Convert a [`PluginResult`] into an [`anyhow::Result`].
pub trait IntoAnyhow<T> {
    /// Convert the result into an [`anyhow::Result`].
    fn into_anyhow(self) -> anyhow::Result<T>;
}

/// Convert any [`Result`] into a [`PluginResult`].
pub trait IntoPluginResult<T> {
    /// Convert the result into a [`PluginResult`].
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

/// A C ABI compatible JSON value.
/// This is a wrapper around a JSON value that can be serialized and deserialized.
/// This is used to pass JSON values between plugins.
///
/// # Usage
/// ```
/// use datagen_rs::plugins::abi::JsonValue;
///
/// let value = JsonValue::read_from(42).unwrap();
/// let number: i32 = value.parse_into().unwrap();
/// assert_eq!(number, 42);
/// ```
#[repr(C)]
#[derive(StableAbi, Clone)]
pub struct JsonValue {
    value: RVec<u8>,
}

impl JsonValue {
    /// Convert a [`serde::ser::Serialize`] value into a [`JsonValue`].
    pub fn read_from<T: Serialize>(value: T) -> anyhow::Result<Self> {
        serde_json::to_vec(&value)
            .map(|value| Self {
                value: value.into(),
            })
            .map_err(anyhow::Error::from)
    }

    /// Convert this [`JsonValue`] into a [`serde::de::Deserialize`] value.
    pub fn parse_into<'a, T: Deserialize<'a>>(&'a self) -> anyhow::Result<T> {
        serde_json::from_slice(&self.value).map_err(anyhow::Error::from)
    }
}

/// A C ABI compatible plugin.
/// This trait is meant for internal use and should not be implemented by users.
///
/// See the [`crate::plugins::plugin::Plugin`] trait for the public API.
#[sabi_trait]
pub trait PluginAbi: Send + Sync {
    /// Get the name of the plugin.
    fn name(&self) -> RString;

    /// Generate a random value with the given schema and arguments.
    fn generate(
        &self,
        schema: CurrentSchemaAbiBox,
        args: JsonValue,
    ) -> PluginResult<GeneratedSchemaAbi>;

    /// Transform a value with the given schema and arguments.
    fn transform(
        &self,
        schema: CurrentSchemaAbiBox,
        value: GeneratedSchemaAbi,
        args: JsonValue,
    ) -> PluginResult<GeneratedSchemaAbi>;

    /// Serialize a value with the given schema and arguments.
    fn serialize(&self, value: GeneratedSchemaAbi, args: JsonValue) -> PluginResult<RString>;

    /// Serialize a value with the given schema and arguments, with progress.
    #[allow(unused_variables)]
    fn serialize_with_progress(
        &self,
        value: GeneratedSchemaAbi,
        args: JsonValue,
        callback: SerializeCallback,
    ) -> PluginResult<RString>;
}

//unsafe impl Send for SerializeCallback {}
//unsafe impl Sync for SerializeCallback {}

type PluginSerializeCallbackVec = Vec<Option<PluginSerializeCallback>>;

#[stateful(init(state))]
extern "C" fn call_serialize_callback(
    current: usize,
    total: usize,
    id: usize,
    mut state: MutAppStateLock<PluginSerializeCallbackVec>,
) -> PluginResult<()> {
    PluginResult::wrap(|| {
        let Some(Some(callback)) = state.get(id) else {
            bail!("Callback with id {} not found", id);
        };

        callback(current, total)
    })
}

#[repr(C)]
#[derive(StableAbi)]
struct SerializeCallbackImpl {
    id: usize,
    func: extern "C" fn(usize, usize, usize) -> PluginResult<()>,
}

impl Drop for SerializeCallbackImpl {
    #[stateful(init(state))]
    fn drop(&mut self, mut state: MutAppStateLock<PluginSerializeCallbackVec>) {
        state[self.id] = None;
    }
}

#[repr(C)]
#[derive(StableAbi, Clone)]
pub struct SerializeCallback {
    inner: RArc<SerializeCallbackImpl>,
}

impl SerializeCallback {
    #[stateful(init(state))]
    pub fn new(
        func: PluginSerializeCallback,
        mut state: MutAppStateLock<PluginSerializeCallbackVec>,
    ) -> Self {
        state.push(Some(func));

        Self {
            inner: RArc::new(SerializeCallbackImpl {
                func: call_serialize_callback,
                id: state.len() - 1,
            }),
        }
    }

    pub fn call(&self, current: usize, total: usize) -> anyhow::Result<()> {
        (self.inner.func)(current, total, self.inner.id).into_anyhow()
    }
}

pub type PluginAbiBox = PluginAbi_TO<'static, RBox<()>>;

#[sabi_trait]
pub trait CurrentSchemaAbi: Clone + Send + Sync {
    fn child(
        &self,
        sibling: ROption<CurrentSchemaAbiBox>,
        path: RString,
    ) -> PluginResult<CurrentSchemaAbiBox>;

    fn resolve_ref(&self, reference: RString) -> PluginResult<ResolvedReferenceAbi>;

    fn finalize(&self, schema: GeneratedSchemaAbi) -> PluginResult<GeneratedSchemaAbi>;

    fn path(&self) -> PluginResult<SchemaPathAbiBox>;

    fn get_plugin(&self, key: RString) -> PluginResult<PluginAbiBox>;

    fn plugin_exists(&self, key: RString) -> PluginResult<bool>;

    fn options(&self) -> PluginResult<JsonValue>;

    fn schema_value_properties(&self) -> PluginResult<JsonValue>;
}

pub type CurrentSchemaAbiBox = CurrentSchemaAbi_TO<'static, RBox<()>>;

#[sabi_trait]
pub trait SchemaPathAbi {
    fn append(&self, path: RString) -> SchemaPathAbiBox;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn pop(&self, num: i32) -> SchemaPathAbiBox;

    fn as_normalized_path(&self) -> RString;

    fn parts(&self) -> RVec<RString>;
}

pub type SchemaPathAbiBox = SchemaPathAbi_TO<'static, RBox<()>>;

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

    fn into_vec(self) -> PluginResult<RVec<GeneratedSchemaAbi>>;
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

impl TryFrom<GeneratedSchemaVecAbiBox> for Vec<GeneratedSchemaAbi> {
    type Error = anyhow::Error;

    fn try_from(value: GeneratedSchemaVecAbiBox) -> anyhow::Result<Self> {
        value.into_vec().into_anyhow().map(Into::into)
    }
}

pub type GeneratedSchemaVecAbiBox = GeneratedSchemaVecAbi_TO<'static, RBox<()>>;

#[sabi_trait]
pub trait GeneratedSchemaMapAbi: Clone {
    fn keys(&self) -> RVec<RString>;

    fn values(&self) -> PluginResult<RVec<GeneratedSchemaAbi>>;
}

pub type GeneratedSchemaMapAbiBox = GeneratedSchemaMapAbi_TO<'static, RBox<()>>;

#[repr(C)]
#[derive(StableAbi)]
pub enum ResolvedReferenceAbi {
    Single(GeneratedSchemaAbi),
    Multiple(GeneratedSchemaVecAbiBox),
    None,
}

impl TryFrom<ResolvedReferenceAbi> for ResolvedReference {
    type Error = anyhow::Error;

    fn try_from(value: ResolvedReferenceAbi) -> anyhow::Result<Self> {
        Ok(match value {
            ResolvedReferenceAbi::Single(schema) => ResolvedReference::Single(schema.try_into()?),
            ResolvedReferenceAbi::Multiple(schemas) => ResolvedReference::multiple(
                schemas
                    .into_vec()
                    .into_anyhow()?
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<anyhow::Result<_>>()?,
            ),
            ResolvedReferenceAbi::None => ResolvedReference::none(),
        })
    }
}
