use crate::schema::transform::MaybeValidTransform;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serialize",
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
pub struct Counter {
    pub step: Option<i64>,
    pub transform: Option<Vec<MaybeValidTransform>>,
    pub path_specific: Option<bool>,
    pub start: Option<i64>,
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGenerated;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::schema::counter::Counter;
    use crate::schema::transform::MaybeValidTransform;
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

    fn fetch_inc_path_specific_counter(path: &str, start: i64, step: i64) -> i64 {
        let mut lock = PATH_SPECIFIC_COUNTER.lock().unwrap();
        let map = lock.get_or_insert_with(HashMap::new);

        match map.get_mut(path) {
            Some(el) => {
                el.add_assign(step);
                *el
            }
            None => {
                map.insert(path.into(), start);
                start
            }
        }
    }

    impl IntoGenerated for Counter {
        fn into_generated(self, schema: DatagenContextRef) -> anyhow::Result<GeneratedSchema> {
            let value = if self.path_specific.unwrap_or(false) {
                fetch_inc_path_specific_counter(
                    &schema.path()?.to_normalized_path(),
                    self.start.unwrap_or(0),
                    self.step.unwrap_or(1),
                )
            } else {
                if COUNTER.load(Ordering::SeqCst) == 0 {
                    COUNTER.store(self.start.unwrap_or(0), Ordering::SeqCst);
                }

                COUNTER.fetch_add(self.step.unwrap_or(1), Ordering::SeqCst)
            };

            Ok(GeneratedSchema::Integer(value))
        }

        fn get_transform(&self) -> Option<Vec<MaybeValidTransform>> {
            self.transform.clone()
        }
    }
}
