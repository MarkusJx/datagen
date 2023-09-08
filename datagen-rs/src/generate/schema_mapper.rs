use crate::generate::current_schema::CurrentSchema;
use crate::generate::generated_schema::GeneratedSchema;
use crate::schema::transform::Transform;
use crate::util::types::Result;
use indexmap::IndexMap;
use std::sync::Arc;

pub trait MapSchema {
    fn map_index_map<K, F: Fn(&Arc<CurrentSchema>, K) -> Result<Arc<GeneratedSchema>>>(
        &self,
        map: IndexMap<String, K>,
        transform: Option<Transform>,
        finalize: bool,
        func: F,
    ) -> Result<Arc<GeneratedSchema>>;

    fn map_array<K: Clone, F: Fn(&Arc<CurrentSchema>, K) -> Result<Arc<GeneratedSchema>>>(
        &self,
        num: usize,
        arg: K,
        transform: Option<Transform>,
        finalize: bool,
        func: F,
    ) -> Result<Arc<GeneratedSchema>>;
}

impl MapSchema for Arc<CurrentSchema> {
    fn map_index_map<K, F: Fn(&Arc<CurrentSchema>, K) -> Result<Arc<GeneratedSchema>>>(
        &self,
        map: IndexMap<String, K>,
        transform: Option<Transform>,
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
            let mut res = GeneratedSchema::Object(res).into();
            if let Some(transform) = transform {
                res = transform.transform(self.clone(), res)?;
            }

            Ok(self.finalize(res))
        } else {
            Ok(GeneratedSchema::Object(res).into())
        }
    }

    fn map_array<K: Clone, F: Fn(&Arc<CurrentSchema>, K) -> Result<Arc<GeneratedSchema>>>(
        &self,
        length: usize,
        value: K,
        transform: Option<Transform>,
        finalize: bool,
        func: F,
    ) -> Result<Arc<GeneratedSchema>> {
        let mut res = Vec::with_capacity(length as _);
        let mut current_schema: Option<Arc<CurrentSchema>> = None;

        for i in 0..length {
            current_schema = if let Some(cur) = current_schema {
                Some(Arc::new(CurrentSchema::child(
                    self.clone(),
                    Some(cur),
                    i.to_string(),
                )))
            } else {
                Some(Arc::new(CurrentSchema::child(
                    self.clone(),
                    None,
                    i.to_string(),
                )))
            };

            res.push(func(current_schema.as_ref().unwrap(), value.clone())?);
        }

        if finalize {
            let mut res = GeneratedSchema::Array(res).into();
            if let Some(transform) = transform {
                res = transform.transform(self.clone(), res)?;
            }

            Ok(self.finalize(res))
        } else {
            Ok(GeneratedSchema::Array(res).into())
        }
    }
}
