use datagen_rs::generate::current_schema::CurrentSchema;
use datagen_rs::generate::generated_schema::IntoRandom;
use datagen_rs::plugins::plugin::Plugin;
use datagen_rs::plugins::plugin_list::PluginList;
use datagen_rs::schema::schema_definition::Schema;
use napi::bindgen_prelude::ToNapiValue;
use napi::threadsafe_function::ThreadsafeFunction;
use napi::{Env, JsFunction};
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) fn generate_random_data_with_progress<F: Fn(usize, usize)>(
    schema: Schema,
    progress_callback: Option<F>,
    additional_plugins: Option<HashMap<String, Box<dyn Plugin>>>,
) -> anyhow::Result<String> {
    let plugins = PluginList::from_schema(&schema, additional_plugins)?;
    let options = Arc::new(schema.options.unwrap_or_default());
    let root = CurrentSchema::root(options.clone(), plugins.clone());
    let generated = schema.value.into_random(root)?;

    let serializer = options.serializer.as_ref().unwrap_or_default();

    if let Some(callback) = progress_callback {
        serializer.serialize_generated_with_progress(generated, Some(plugins), &callback)
    } else {
        serializer.serialize_generated(generated, Some(plugins))
    }
}

pub(crate) trait EnvExt {
    fn create_tsfn<U: Send + ToNapiValue>(
        &self,
        func: Option<JsFunction>,
    ) -> napi::Result<Option<ThreadsafeFunction<U>>>;
}

impl EnvExt for Env {
    fn create_tsfn<U: Send + ToNapiValue>(
        &self,
        func: Option<JsFunction>,
    ) -> napi::Result<Option<ThreadsafeFunction<U>>> {
        func.map(|f| self.create_threadsafe_function(&f, 0, |ctx| Ok(vec![ctx.value])))
            .map_or(Ok(None), |f| f.map(Some))
    }
}
