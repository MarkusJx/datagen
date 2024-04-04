use std::sync::Arc;

use crate::classes::datagen_plugin::DatagenPlugin;
use crate::classes::schema_path::SchemaPath;
use crate::util::traits::IntoNapiResult;
use datagen_rs::generate::datagen_context::DatagenContextRef;
use datagen_rs::generate::generated_schema::GeneratedSchema;
use napi::{Env, JsObject, JsUnknown};
use serde_json::Value;

#[napi]
#[derive(Clone)]
pub struct CurrentSchema(DatagenContextRef);

#[napi]
impl CurrentSchema {
    pub fn from_ref(inner: DatagenContextRef) -> anyhow::Result<Self> {
        Ok(Self(inner))
    }

    pub fn inner(&self) -> DatagenContextRef {
        self.0.clone()
    }

    #[napi]
    pub fn child(&self, path: String, sibling: Option<&CurrentSchema>) -> napi::Result<Self> {
        Ok(Self(
            self.0
                .child(sibling.map(|s| s.0.clone()), &path)
                .into_napi()?
                .into(),
        ))
    }

    fn _resolve_ref(
        inner: &DatagenContextRef,
        path: String,
    ) -> napi::Result<Vec<Arc<GeneratedSchema>>> {
        Ok(inner
            .resolve_ref(&path)
            .into_napi()?
            .into_vec()
            .unwrap_or_default())
    }

    #[napi(ts_return_type = "Array<any>")]
    pub fn resolve_ref(&self, env: Env, path: String) -> napi::Result<JsUnknown> {
        env.to_js_value(&Self::_resolve_ref(&self.0, path)?)
    }

    #[napi(ts_return_type = "Promise<Array<any>>")]
    pub fn resolve_ref_async(&self, env: Env, path: String) -> napi::Result<JsObject> {
        let inner = self.0.clone();
        env.execute_tokio_future(
            futures::future::lazy(move |_| Self::_resolve_ref(&inner, path)),
            |env, res| env.to_js_value(&res),
        )
    }

    #[napi(ts_return_type = "any")]
    pub fn finalize(&self, env: Env, value: Value) -> napi::Result<JsUnknown> {
        env.to_js_value(
            &self
                .0
                .finalize(GeneratedSchema::Value(value).into())
                .into_napi()?,
        )
    }

    #[napi(ts_return_type = "Promise<any>")]
    pub fn finalize_async(&self, env: Env, value: Value) -> napi::Result<JsObject> {
        let inner = self.0.clone();
        env.execute_tokio_future(
            futures::future::lazy(move |_| {
                inner
                    .finalize(GeneratedSchema::Value(value).into())
                    .into_napi()
            }),
            |env, res| env.to_js_value(&res),
        )
    }

    #[napi]
    pub fn path(&self) -> napi::Result<SchemaPath> {
        Ok(SchemaPath::from_path(self.0.path().into_napi()?))
    }

    #[napi]
    pub fn get_plugin(&self, name: String) -> napi::Result<DatagenPlugin> {
        DatagenPlugin::new(name, self.0.clone())
    }

    #[napi(getter, ts_return_type = "any")]
    pub fn options(&self, env: Env) -> napi::Result<JsUnknown> {
        env.to_js_value(&self.0.options().into_napi()?)
    }
}
