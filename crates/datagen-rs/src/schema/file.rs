use crate::schema::transform::Transform;
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
    pub transform: Option<Vec<Transform>>,
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

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGenerated;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::schema::file::{File, FileMode};
    use crate::schema::transform::Transform;
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

        fn get_transform(&self) -> Option<Vec<Transform>> {
            self.transform.clone()
        }
    }
}
