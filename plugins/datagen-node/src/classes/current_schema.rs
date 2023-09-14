use datagen_rs::generate::current_schema::CurrentSchemaRef;
use napi::{Env, JsUnknown};

#[napi]
#[derive(Clone)]
pub struct CurrentSchema {
    inner: CurrentSchemaRef,
}

#[napi]
impl CurrentSchema {
    pub fn from_ref(inner: CurrentSchemaRef) -> Self {
        Self { inner }
    }

    #[napi]
    pub fn child(&self, path: String, sibling: Option<&CurrentSchema>) -> Self {
        Self {
            inner: datagen_rs::generate::current_schema::CurrentSchema::child(
                self.inner.clone(),
                sibling.map(|s| s.inner.clone()),
                path,
            )
            .into(),
        }
    }

    #[napi(ts_return_type = "Array<any>")]
    pub fn resolve_ref(&self, env: Env, path: String) -> napi::Result<JsUnknown> {
        env.to_js_value(
            &self
                .inner
                .resolve_ref(path)
                .map_err(|e| napi::Error::from_reason(e.to_string()))?
                .into_vec()
                .unwrap_or_default(),
        )
    }
}
