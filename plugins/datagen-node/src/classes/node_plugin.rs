use crate::classes::current_schema::CurrentSchema;
use datagen_rs::generate::current_schema::CurrentSchemaRef;
use datagen_rs::generate::generated_schema::GeneratedSchema;
use datagen_rs::plugins::plugin::Plugin;
use futures::TryFutureExt;
use napi::bindgen_prelude::{ClassInstance, SharedReference};
use napi::threadsafe_function::ThreadsafeFunction;
use napi::threadsafe_function::{ErrorStrategy, JsValuesTupleIntoVec};
use napi::{Env, JsFunction};
use serde_json::Value;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

pub struct GenerateArgs {
    pub args: Value,
    pub schema: CurrentSchema,
}

#[napi(object)]
pub struct TransformArgs {
    pub args: Value,
    pub schema: ClassInstance<CurrentSchema>,
    pub value: Value,
}

#[napi(object)]
pub struct SerializeArgs {
    pub args: Value,
    pub value: Value,
}

#[napi]
#[derive(Clone)]
pub struct NodePlugin {
    name: String,
    generate: ThreadsafeFunction<GenerateArgs>,
    //transform: ThreadsafeFunction<TransformArgs>,
    //serialize: ThreadsafeFunction<SerializeArgs>,
}

unsafe impl Send for NodePlugin {}
unsafe impl Sync for NodePlugin {}

#[napi]
impl NodePlugin {
    #[napi(constructor)]
    pub fn new(
        name: String,
        #[napi(ts_arg_type = "(err: Error | null, schema: CurrentSchema, args: any) => any")]
        generate: JsFunction,
        #[napi(ts_arg_type = "(args: TransformArgs) => any")] transform: JsFunction,
        #[napi(ts_arg_type = "(args: SerializeArgs) => any")] serialize: JsFunction,
        env: Env,
    ) -> napi::Result<Self> {
        Ok(Self {
            name,
            generate: env.create_threadsafe_function(&generate, 1, |ctx| {
                let args: GenerateArgs = ctx.value;
                let mut res = args.schema.into_vec(&ctx.env)?;
                res.push(ctx.env.to_js_value(&args.args)?);
                Ok(res)
            })?,
            //transform,
            //serialize,
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
    ) -> datagen_rs::util::types::Result<Arc<GeneratedSchema>> {
        println!("generate");
        let args = GenerateArgs {
            args,
            schema: CurrentSchema::from_ref(schema),
        };

        let res: Value = futures::executor::block_on(self.generate.call_async(Ok(args)))?;
        serde_json::from_value::<GeneratedSchema>(res)
            .map_err(Into::into)
            .map(Into::into)
    }
}
