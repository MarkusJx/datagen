#[cfg(feature = "schema")]
use datagen_rs::util::helpers::get_schema_value;
#[cfg(feature = "generate")]
use datagen_rs_progress_plugin::{PluginWithSchemaResult, ProgressPlugin};
#[cfg(feature = "generate")]
use js_sys::Function;
#[cfg(any(feature = "schema", feature = "generate"))]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(any(feature = "schema", feature = "generate"))]
use wasm_bindgen::JsError;
#[cfg(feature = "generate")]
use wasm_bindgen::JsValue;

#[cfg(feature = "generate")]
struct FunctionWrapper(Function);

#[cfg(feature = "generate")]
unsafe impl Send for FunctionWrapper {}
#[cfg(feature = "generate")]
unsafe impl Sync for FunctionWrapper {}

#[cfg(feature = "generate")]
impl FunctionWrapper {
    fn call1(&self, this: &JsValue, arg: &JsValue) -> Result<JsValue, JsValue> {
        self.0.call1(this, arg)
    }
}

#[cfg(feature = "generate")]
#[wasm_bindgen]
pub struct GenerateProgress {
    #[wasm_bindgen(readonly)]
    pub current: u32,
    #[wasm_bindgen(readonly)]
    pub total: u32,
}

#[cfg(feature = "schema")]
#[wasm_bindgen(js_name = "getSchema")]
pub fn get_schema() -> Result<String, JsError> {
    match get_schema_value() {
        Ok(v) => serde_json::to_string_pretty(&v).map_err(Into::into),
        Err(e) => Err(JsError::new(&e.to_string())),
    }
}

#[cfg(feature = "generate")]
#[wasm_bindgen(js_name = "generateRandomData")]
pub fn generate_random_data(
    schema: JsValue,
    progress_callback: Option<Function>,
) -> Result<String, JsError> {
    let schema = serde_wasm_bindgen::from_value(schema).map_err(JsError::from)?;
    let (schema, plugins) = if let Some(callback) = progress_callback {
        let callback = FunctionWrapper(callback);
        let PluginWithSchemaResult { schema, plugins } =
            ProgressPlugin::with_schema(schema, move |current, total| {
                callback
                    .call1(
                        &JsValue::null(),
                        &JsValue::from(GenerateProgress {
                            current: current as _,
                            total: total as _,
                        }),
                    )
                    .expect("Failed to call callback");
            })
            .map_err(|e| JsError::new(&e.to_string()))?;

        (schema, Some(plugins))
    } else {
        (schema, None)
    };

    datagen_rs::util::helpers::generate_random_data(schema, plugins)
        .map_err(|e| JsError::new(&e.to_string()))
}
