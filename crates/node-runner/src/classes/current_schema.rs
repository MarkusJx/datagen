use std::sync::Arc;

use crate::classes::datagen_plugin::DatagenPlugin;
use crate::classes::schema_path::SchemaPath;
use crate::util::traits::IntoNapiResult;
use datagen_rs::generate::current_schema::CurrentSchemaRef;
use datagen_rs::generate::generated_schema::GeneratedSchema;
use napi::{Env, JsObject, JsUnknown};
use serde_json::Value;

#[napi]
#[derive(Clone)]
pub struct CurrentSchema(CurrentSchemaRef);

#[napi]
impl CurrentSchema {
    pub fn from_ref(
        inner: Box<dyn datagen_rs::plugins::plugin::ICurrentSchema>,
    ) -> anyhow::Result<Self> {
        Ok(Self(
            datagen_rs::generate::current_schema::CurrentSchema::from_boxed(inner)?,
        ))
    }

    pub fn inner(&self) -> CurrentSchemaRef {
        self.0.clone()
    }

    #[napi]
    pub fn child(&self, path: String, sibling: Option<&CurrentSchema>) -> Self {
        Self(
            datagen_rs::generate::current_schema::CurrentSchema::child(
                self.0.clone(),
                sibling.map(|s| s.0.clone()),
                path,
            )
            .into(),
        )
    }

    fn _resolve_ref(
        inner: &CurrentSchemaRef,
        path: String,
    ) -> napi::Result<Vec<Arc<GeneratedSchema>>> {
        Ok(inner
            .resolve_ref(path)
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
        env.to_js_value(&self.0.finalize(GeneratedSchema::Value(value).into()))
    }

    #[napi(ts_return_type = "Promise<any>")]
    pub fn finalize_async(&self, env: Env, value: Value) -> napi::Result<JsObject> {
        let inner = self.0.clone();
        env.execute_tokio_future(
            futures::future::lazy(
                move |_| Ok(inner.finalize(GeneratedSchema::Value(value).into())),
            ),
            |env, res| env.to_js_value(&res),
        )
    }

    #[napi]
    pub fn path(&self) -> SchemaPath {
        SchemaPath::from_path(self.0.path())
    }

    #[napi]
    pub fn get_plugin(&self, name: String) -> napi::Result<DatagenPlugin> {
        DatagenPlugin::new(name, self.0.clone())
    }

    #[napi(getter, ts_return_type = "any")]
    pub fn options(&self, env: Env) -> napi::Result<JsUnknown> {
        env.to_js_value(self.0.options())
    }
}
