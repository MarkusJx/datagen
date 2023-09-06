use crate::generate::current_schema::CurrentSchema;
use crate::generate::generated_schema::GeneratedSchema;
use crate::util::types::Result;
use indexmap::IndexMap;
use std::sync::Arc;

pub trait MapSchema {
    fn map_index_map<K, F: Fn(&Arc<CurrentSchema>, K) -> Result<Arc<GeneratedSchema>>>(
        &self,
        map: IndexMap<String, K>,
        finalize: bool,
        func: F,
    ) -> Result<Arc<GeneratedSchema>>;
}

impl MapSchema for Arc<CurrentSchema> {
    fn map_index_map<K, F: Fn(&Arc<CurrentSchema>, K) -> Result<Arc<GeneratedSchema>>>(
        &self,
        map: IndexMap<String, K>,
        finalize: bool,
        func: F,
    ) -> Result<Arc<GeneratedSchema>> {
        let mut res = IndexMap::new();
        let mut current_schema: Option<Arc<CurrentSchema>> = None;

        for (key, value) in map {
            current_schema = if let Some(cur) = current_schema {
                Some(Arc::new(CurrentSchema::child(
                    self.clone(),
                    Some(cur),
                    key.clone(),
                )))
            } else {
                Some(Arc::new(CurrentSchema::child(
                    self.clone(),
                    None,
                    key.clone(),
                )))
            };

            res.insert(key, func(current_schema.as_ref().unwrap(), value)?);
        }

        if finalize {
            Ok(self.finalize(GeneratedSchema::Object(res).into()))
        } else {
            Ok(GeneratedSchema::Object(res).into())
        }
    }
}
