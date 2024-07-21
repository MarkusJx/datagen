use crate::schema::transform::MaybeValidTransform;
use crate::util::traits::GetTransform;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
pub struct File {
    pub path: String,
    pub mode: Option<FileMode>,
    pub transform: Option<Vec<MaybeValidTransform>>,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
pub enum FileMode {
    #[default]
    Sequential,
    Random,
}

impl GetTransform for File {
    fn get_transform(&self) -> Option<Vec<MaybeValidTransform>> {
        self.transform.clone()
    }
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGenerated;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::schema::file::{File, FileMode};
    use crate::util::sequential_vec::SequentialVec;
    use once_cell::sync::Lazy;
    use serde_json::Value;
    use std::collections::HashMap;
    use std::io::BufReader;
    use std::sync::Mutex;

    static FILES: Lazy<Mutex<HashMap<String, SequentialVec<Value>>>> =
        Lazy::new(|| Mutex::new(HashMap::new()));

    impl IntoGenerated for File {
        fn into_generated(self, _: DatagenContextRef) -> anyhow::Result<GeneratedSchema> {
            let mut lock = FILES.lock().unwrap();
            let data = match lock.get_mut(&self.path) {
                Some(val) => val,
                None => {
                    let reader = BufReader::new(std::fs::File::open(&self.path)?);
                    let data = SequentialVec::<Value>::new(serde_json::from_reader(reader)?)?;
                    lock.insert(self.path.clone(), data);
                    lock.get_mut(&self.path).unwrap()
                }
            };

            let value = match self.mode.unwrap_or_default() {
                FileMode::Sequential => data.next_value().clone(),
                FileMode::Random => data.random().clone(),
            };

            Ok(GeneratedSchema::Value(value))
        }
    }
}

#[cfg(feature = "validate-schema")]
pub mod validate {
    use crate::schema::file::File;
    use crate::validation::path::ValidationPath;
    use crate::validation::result::{IterValidate, ValidationErrors, ValidationResult};
    use crate::validation::validate::ValidateGenerateSchema;
    use serde_json::Value;

    impl ValidateGenerateSchema for File {
        fn validate_generate_schema(&self, path: &ValidationPath) -> ValidationResult {
            std::fs::File::open(&self.path)
                .map_err(|e| {
                    ValidationErrors::single(
                        format!("Failed to open file at path '{}'", self.path),
                        path,
                        Some(e.into()),
                        Some(Value::String(self.path.clone())),
                    )
                })
                .and_then(|f| {
                    ValidationResult::ensure_ok(
                        serde_json::from_reader::<std::fs::File, Value>(f),
                        format!("Failed to parse JSON from file at path: {}", self.path),
                        path,
                        Some(Value::String(self.path.clone())),
                    )
                })
        }
    }
}
