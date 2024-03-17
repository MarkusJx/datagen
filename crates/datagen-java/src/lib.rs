use datagen_rs_progress_plugin::{PluginWithSchemaResult, ProgressPlugin};
use jni::JNIEnv;
use jni_bindgen::jni;
use jni_bindgen::objects::traits::{FromJNI, ObjectFromJNI};

#[jni(package = "io.github.markusjx.datagen.generated")]
trait GenerateCallback: Send + Sync {
    fn progress(&self, env: &mut JNIEnv, current: i32, total: i32) -> jni_bindgen::Result<()>;
}

struct DatagenImpl;

#[jni(
    package = "io.github.markusjx.datagen.generated",
    not_null_annotation = "jakarta.validation.constraints.NotNull",
    nullable_annotation = "jakarta.annotation.Nullable"
)]
impl DatagenImpl {
    #[jni]
    fn generate_random_data(
        env: &mut JNIEnv,
        schema: String,
        callback: Option<Box<dyn GenerateCallback>>,
    ) -> anyhow::Result<String> {
        let vm = env.get_java_vm()?;
        let schema = serde_json::from_str(&schema)?;
        let (schema, plugins) = if let Some(callback) = callback {
            let PluginWithSchemaResult { schema, plugins } =
                ProgressPlugin::with_schema(schema, move |current, total| {
                    if let Err(e) =
                        Self::call_callback(&vm, callback.as_ref(), current as i32, total as i32)
                    {
                        eprintln!("Failed to call callback: {}", e);
                    }
                })?;

            (schema, Some(plugins))
        } else {
            (schema, None)
        };

        datagen_rs::util::helpers::generate_random_data(schema, plugins)
    }

    #[jni]
    fn get_schema() -> anyhow::Result<String> {
        let schema = datagen_rs::util::helpers::get_schema_value()?;
        serde_json::to_string_pretty(&schema).map_err(Into::into)
    }

    fn call_callback(
        vm: &jni::JavaVM,
        callback: &dyn GenerateCallback,
        current: i32,
        total: i32,
    ) -> jni_bindgen::Result<()> {
        let mut env = vm.get_env()?;
        callback.progress(&mut env, current, total)
    }
}
