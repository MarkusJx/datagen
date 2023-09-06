use crate::generate::generated_schema::GeneratedSchema;
use crate::generate::schema_path::SchemaPath;
#[cfg(feature = "map_schema")]
use crate::schema::schema_definition::SchemaOptions;
#[cfg(feature = "map_schema")]
use rand::Rng;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

pub(crate) type SchemaProperties = BTreeMap<String, Vec<Arc<GeneratedSchema>>>;

#[derive(Debug)]
#[cfg_attr(not(feature = "map_schema"), allow(dead_code))]
pub(crate) struct SchemaValue {
    pub properties: Arc<Mutex<SchemaProperties>>,
    pub path: SchemaPath,
}

#[cfg(feature = "map_schema")]
impl SchemaValue {
    pub(crate) fn finalize(
        &mut self,
        options: &Arc<SchemaOptions>,
        schema: Arc<GeneratedSchema>,
        path: String,
    ) {
        let mut properties = self.properties.lock().unwrap();
        if let Some(props) = properties.get_mut(&path) {
            if let Some(max_size) = options.max_ref_cache_size {
                if max_size == 0 {
                    return;
                } else if props.len() >= max_size {
                    let random_index = rand::thread_rng().gen_range(0..props.len());
                    props[random_index] = schema;
                    return;
                }
            }

            props.push(schema);
        } else {
            properties.insert(path, vec![schema]);
        }
    }

    pub(crate) fn path(&self) -> &SchemaPath {
        &self.path
    }
}
