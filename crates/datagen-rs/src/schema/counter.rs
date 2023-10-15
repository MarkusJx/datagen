use crate::schema::transform::Transform;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", serde(rename_all = "camelCase"))]
pub struct Counter {
    pub step: Option<i64>,
    pub transform: Option<Vec<Transform>>,
    pub path_specific: Option<bool>,
    pub start: Option<i64>,
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::current_schema::CurrentSchemaRef;
    use crate::generate::generated_schema::generate::IntoGenerated;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::schema::counter::Counter;
    use crate::schema::transform::Transform;
    use std::{
        collections::HashMap,
        ops::AddAssign,
        sync::{
            atomic::{AtomicI64, Ordering},
            Mutex,
        },
    };

    static COUNTER: AtomicI64 = AtomicI64::new(0);
    static PATH_SPECIFIC_COUNTER: Mutex<Option<HashMap<String, i64>>> = Mutex::new(None);

    fn fetch_inc_path_specific_counter(path: &str) -> i64 {
        let mut lock = PATH_SPECIFIC_COUNTER.lock().unwrap();
        let map = lock.get_or_insert_with(HashMap::new);

        match map.get_mut(path) {
            Some(el) => {
                el.add_assign(1);
                *el
            }
            None => {
                map.insert(path.into(), 1);
                1
            }
        }
    }

    impl IntoGenerated for Counter {
        fn into_generated(self, schema: CurrentSchemaRef) -> anyhow::Result<GeneratedSchema> {
            let value = if self.path_specific.unwrap_or(false) {
                fetch_inc_path_specific_counter(&schema.path().to_normalized_path())
            } else {
                if COUNTER.load(Ordering::SeqCst) == 0 {
                    COUNTER.store(self.start.unwrap_or(0), Ordering::SeqCst);
                }

                COUNTER.fetch_add(1, Ordering::SeqCst)
            };

            Ok(GeneratedSchema::Integer(value))
        }

        fn get_transform(&self) -> Option<Vec<Transform>> {
            self.transform.clone()
        }
    }
}
