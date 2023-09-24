use crate::generate::generated_schema::GeneratedSchema;
use crate::generate::schema_path::SchemaPath;
#[cfg(feature = "map-schema")]
use crate::schema::schema_definition::SchemaOptions;
#[cfg(feature = "serialize")]
use serde::Serialize;
use std::collections::{BTreeMap, VecDeque};
use std::fmt::Debug;
#[cfg(feature = "serialize")]
use std::fmt::Formatter;
use std::sync::{Arc, Mutex};

pub(crate) type SchemaProperties = BTreeMap<String, VecDeque<Arc<GeneratedSchema>>>;

#[cfg_attr(not(feature = "serialize"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(not(feature = "map_schema"), allow(dead_code))]
pub(crate) struct SchemaValue {
    pub properties: Arc<Mutex<SchemaProperties>>,
    pub path: SchemaPath,
}

#[cfg(feature = "map-schema")]
impl SchemaValue {
    pub(crate) fn finalize(
        &mut self,
        options: &Arc<SchemaOptions>,
        schema: Arc<GeneratedSchema>,
        path: String,
    ) {
        if path.is_empty() {
            return;
        }

        let mut properties = self.properties.lock().unwrap();
        if let Some(props) = properties.get_mut(&path) {
            if props.contains(&schema) {
                return;
            }

            if let Some(max_size) = options.max_ref_cache_size {
                if max_size == 0 {
                    return;
                }

                while props.len() >= max_size {
                    props.pop_front();
                }
            }

            props.push_back(schema);
        } else {
            properties.insert(path, vec![schema].into());
        }
    }

    pub(crate) fn path(&self) -> &SchemaPath {
        &self.path
    }
}

#[cfg(feature = "serialize")]
impl Debug for SchemaValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_str(&serde_json::to_string_pretty(&self).unwrap())
        } else {
            f.write_str(&serde_json::to_string(&self).unwrap())
        }
    }
}
