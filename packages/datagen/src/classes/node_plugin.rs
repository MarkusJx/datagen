use crate::classes::current_schema::CurrentSchema;
use crate::classes::node_plugin_args::{
    GenerateArgs, GenerateCall, PluginCall, SerializeArgs, SerializeCall, TransformArgs,
    TransformCall,
};
use anyhow::{anyhow, Context};
use datagen_rs::generate::current_schema::CurrentSchemaRef;
use datagen_rs::generate::generated_schema::GeneratedSchema;
use datagen_rs::plugins::plugin::Plugin;
use napi::threadsafe_function::{ThreadSafeCallContext, ThreadsafeFunction};
use napi::{Env, JsFunction};
use serde_json::Value;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

#[napi]
#[derive(Clone)]
pub struct NodePlugin {
    name: String,
    generate: ThreadsafeFunction<GenerateCall>,
    transform: ThreadsafeFunction<TransformCall>,
    serialize: ThreadsafeFunction<SerializeCall>,
}

unsafe impl Send for NodePlugin {}
unsafe impl Sync for NodePlugin {}

#[napi]
impl NodePlugin {
    #[napi(constructor)]
    pub fn new(
        name: String,
        #[napi(
            ts_arg_type = "(err: Error | null, callback: (res: any) => void, schema: CurrentSchema, args: any) => void"
        )]
        generate: JsFunction,
        #[napi(
            ts_arg_type = "(err: Error | null, callback: (res: any) => void, schema: CurrentSchema, args: any, value: any) => void"
        )]
        transform: JsFunction,
        #[napi(
            ts_arg_type = "(err: Error | null, callback: (res: any) => void, args: any, value: any) => void"
        )]
        serialize: JsFunction,
        env: Env,
    ) -> napi::Result<Self> {
        Ok(Self {
            name,
            generate: env.create_threadsafe_function(
                &generate,
                1,
                |ctx: ThreadSafeCallContext<GenerateCall>| ctx.value.into_js_call(ctx.env),
            )?,
            transform: env.create_threadsafe_function(
                &transform,
                1,
                |ctx: ThreadSafeCallContext<TransformCall>| ctx.value.into_js_call(ctx.env),
            )?,
            serialize: env.create_threadsafe_function(
                &serialize,
                1,
                |ctx: ThreadSafeCallContext<SerializeCall>| ctx.value.into_js_call(ctx.env),
            )?,
        })
    }
}

impl Debug for NodePlugin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodePlugin").finish()
    }
}

impl Plugin for NodePlugin {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn generate(
        &self,
        schema: CurrentSchemaRef,
        args: Value,
    ) -> anyhow::Result<Arc<GeneratedSchema>> {
        let res: Value = PluginCall::call(
            &self.generate,
            GenerateArgs {
                args,
                schema: CurrentSchema::from_ref(schema),
            },
        )
        .with_context(|| {
            anyhow!(
                "Failed to call function 'generate' on plugin '{}'",
                self.name,
            )
        })?;

        serde_json::from_value::<GeneratedSchema>(res)
            .map_err(Into::into)
            .map(Into::into)
    }

    fn transform(
        &self,
        schema: CurrentSchemaRef,
        value: Arc<GeneratedSchema>,
        args: Value,
    ) -> anyhow::Result<Arc<GeneratedSchema>> {
        let res: Value = PluginCall::call(
            &self.transform,
            TransformArgs {
                value: serde_json::to_value(value).map_err(anyhow::Error::new)?,
                args,
                schema: CurrentSchema::from_ref(schema),
            },
        )
        .with_context(|| {
            anyhow!(
                "Failed to call function 'transform' on plugin '{}'",
                self.name,
            )
        })?;

        serde_json::from_value::<GeneratedSchema>(res)
            .map_err(Into::into)
            .map(Into::into)
    }

    fn serialize(&self, value: &Arc<GeneratedSchema>, args: Value) -> anyhow::Result<String> {
        PluginCall::call(
            &self.serialize,
            SerializeArgs {
                args,
                value: serde_json::to_value(value).map_err(anyhow::Error::new)?,
            },
        )
        .with_context(|| {
            anyhow!(
                "Failed to call function 'serialize' on plugin '{}'",
                self.name,
            )
        })
    }
}
