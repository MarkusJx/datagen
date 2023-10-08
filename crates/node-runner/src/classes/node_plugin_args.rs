use crate::classes::current_schema::CurrentSchema;
use anyhow::{anyhow, Context};
use napi::bindgen_prelude::FromNapiValue;
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi::{CallContext, Env, JsString, JsUnknown, Status};
use serde_json::Value;
use std::fmt::Debug;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;

#[derive(Clone)]
pub struct GenerateArgs {
    pub args: Value,
    pub schema: CurrentSchema,
}

#[derive(Clone)]
pub struct TransformArgs {
    pub args: Value,
    pub schema: CurrentSchema,
    pub value: Value,
}

#[derive(Clone)]
pub struct SerializeArgs {
    pub args: Value,
    pub value: Value,
}

pub type GenerateCall = PluginCall<GenerateArgs, Value>;
pub type TransformCall = PluginCall<TransformArgs, Value>;
pub type SerializeCall = PluginCall<SerializeArgs, String>;

pub trait IntoJsCall {
    fn into_js_call(self, callback: JsUnknown, env: Env) -> napi::Result<Vec<JsUnknown>>;
}

pub struct PluginCall<T: Clone + IntoJsCall, R: FromNapiValue + Debug> {
    args: T,
    sender: Mutex<Option<Sender<Result<R, String>>>>,
}

impl<T: Clone + IntoJsCall + 'static, R: FromNapiValue + Debug + 'static> PluginCall<T, R> {
    pub fn call(func: &ThreadsafeFunction<PluginCall<T, R>>, args: T) -> anyhow::Result<R> {
        let (args, rx) = Self::new(args);

        let status = func.call(Ok(args), ThreadsafeFunctionCallMode::Blocking);
        if status != Status::Ok {
            Err(anyhow!("Could not call function: {:?}", status))
        } else {
            rx.recv()?
                .map_err(anyhow::Error::msg)
                .context("Could not receive result from function")
        }
    }

    fn new(args: T) -> (Self, Receiver<Result<R, String>>) {
        let (sender, receiver) = channel();
        (
            Self {
                args,
                sender: Mutex::new(Some(sender)),
            },
            receiver,
        )
    }

    pub fn into_js_call(self, env: Env) -> napi::Result<Vec<JsUnknown>> {
        let args = self.args.clone();
        let callback = self.into_callback(env)?;
        args.into_js_call(callback, env)
    }

    fn set_result(&self, result: Result<R, String>) -> napi::Result<()> {
        self.sender
            .lock()
            .unwrap()
            .take()
            .ok_or(napi::Error::from_reason("The sender was already invoked"))?
            .send(result)
            .map_err(|_| napi::Error::from_reason("Could not send result to sender"))
    }

    fn into_callback(self, env: Env) -> napi::Result<JsUnknown> {
        Ok(env
            .create_function_from_closure("callback", move |ctx| {
                self.set_result(Self::convert_callback(&ctx))
                    .map_err(|e| napi::Error::from_reason(e.to_string()))
            })?
            .into_unknown())
    }

    fn convert_callback(ctx: &CallContext) -> Result<R, String> {
        let err = ctx.get::<JsUnknown>(0).map_err(|e| e.to_string())?;

        if err.is_error().map_err(|e| e.to_string())? {
            let obj = err.coerce_to_object().map_err(|e| e.to_string())?;
            let message = obj
                .get_named_property::<JsString>("message")
                .map_err(|e| e.to_string())?
                .into_utf16()
                .map_err(|e| e.to_string())?
                .as_str()
                .map_err(|e| e.to_string())?;

            Err(message)
        } else {
            R::from_unknown(err).map_err(|e| e.to_string())
        }
    }
}

impl IntoJsCall for GenerateArgs {
    fn into_js_call(self, callback: JsUnknown, env: Env) -> napi::Result<Vec<JsUnknown>> {
        let schema = self
            .schema
            .into_instance(env)?
            .as_object(env)
            .into_unknown();
        let args = env.to_js_value(&self.args)?;

        Ok(vec![callback, schema, args])
    }
}

impl IntoJsCall for TransformArgs {
    fn into_js_call(self, callback: JsUnknown, env: Env) -> napi::Result<Vec<JsUnknown>> {
        let schema = self
            .schema
            .into_instance(env)?
            .as_object(env)
            .into_unknown();
        let args = env.to_js_value(&self.args)?;
        let value = env.to_js_value(&self.value)?;

        Ok(vec![callback, schema, args, value])
    }
}

impl IntoJsCall for SerializeArgs {
    fn into_js_call(self, callback: JsUnknown, env: Env) -> napi::Result<Vec<JsUnknown>> {
        let args = env.to_js_value(&self.args)?;
        let value = env.to_js_value(&self.value)?;

        Ok(vec![callback, args, value])
    }
}
