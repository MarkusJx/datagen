use crate::classes::current_schema::CurrentSchema;
use crate::util::traits::IntoNapiResult;
use datagen_rs::generate::current_schema::CurrentSchemaRef;
use datagen_rs::generate::generated_schema::GeneratedSchema;
use napi::{Env, JsObject, JsUnknown};
use serde_json::Value;
use std::sync::Arc;

#[napi]
pub struct DatagenPlugin {
    name: String,
    schema: CurrentSchemaRef,
}

#[napi]
impl DatagenPlugin {
    pub fn new(name: String, schema: CurrentSchemaRef) -> napi::Result<Self> {
        if schema.plugin_exists(&name) {
            Ok(Self { name, schema })
        } else {
            Err(napi::Error::from_reason(format!(
                "Plugin '{name}' does not exist",
            )))
        }
    }

    #[napi(ts_return_type = "Promise<any>")]
    pub fn generate(
        &self,
        env: Env,
        schema: &CurrentSchema,
        #[napi(ts_arg_type = "any")] args: JsUnknown,
    ) -> napi::Result<JsObject> {
        let args: Value = env.from_js_value(args)?;
        let this_schema = self.schema.clone();
        let schema = schema.clone();
        let name = self.name.clone();

        env.execute_tokio_future(
            futures::future::lazy(move |_| {
                this_schema
                    .get_plugin(&name)
                    .into_napi()?
                    .generate(Box::new(schema.inner()), args)
                    .into_napi()
            }),
            |env, res| env.to_js_value(&res),
        )
    }

    #[napi(ts_return_type = "Promise<any>")]
    pub fn transform(
        &self,
        env: Env,
        schema: &CurrentSchema,
        #[napi(ts_arg_type = "any")] value: JsUnknown,
        #[napi(ts_arg_type = "any")] args: JsUnknown,
    ) -> napi::Result<JsObject> {
        let value: Arc<GeneratedSchema> = env.from_js_value(value)?;
        let args: Value = env.from_js_value(args)?;

        let this_schema = self.schema.clone();
        let schema = schema.clone();
        let name = self.name.clone();

        env.execute_tokio_future(
            futures::future::lazy(move |_| {
                this_schema
                    .get_plugin(&name)
                    .into_napi()?
                    .transform(Box::new(schema.inner()), value, args)
                    .into_napi()
            }),
            |env, res| env.to_js_value(&res),
        )
    }

    #[napi(ts_return_type = "Promise<string>")]
    pub fn serialize(
        &self,
        env: Env,
        #[napi(ts_arg_type = "any")] value: JsUnknown,
        #[napi(ts_arg_type = "any")] args: JsUnknown,
    ) -> napi::Result<JsObject> {
        let value: Arc<GeneratedSchema> = env.from_js_value(value)?;
        let args: Value = env.from_js_value(args)?;

        let this_schema = self.schema.clone();
        let name = self.name.clone();

        env.execute_tokio_future(
            futures::future::lazy(move |_| {
                this_schema
                    .get_plugin(&name)
                    .into_napi()?
                    .serialize(&value, args)
                    .into_napi()
            }),
            |env, res| env.create_string(&res),
        )
    }
}
