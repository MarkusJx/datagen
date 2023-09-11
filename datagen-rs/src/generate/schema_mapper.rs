use crate::generate::current_schema::CurrentSchemaRef;
use crate::generate::generated_schema::GeneratedSchema;
use crate::schema::transform::AnyTransform;
use crate::util::types::Result;
use indexmap::IndexMap;
use std::sync::Arc;

pub trait MapSchema {
    fn map_index_map<K, F: Fn(&CurrentSchemaRef, K) -> Result<Arc<GeneratedSchema>>>(
        &self,
        map: IndexMap<String, K>,
        transform: Option<Vec<AnyTransform>>,
        finalize: bool,
        func: F,
    ) -> Result<Arc<GeneratedSchema>>;

    fn map_array<K: Clone, F: Fn(&CurrentSchemaRef, K) -> Result<Arc<GeneratedSchema>>>(
        &self,
        num: usize,
        arg: K,
        transform: Option<Vec<AnyTransform>>,
        finalize: bool,
        func: F,
    ) -> Result<Arc<GeneratedSchema>>;
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::current_schema::{CurrentSchema, CurrentSchemaRef};
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::generate::schema_mapper::MapSchema;
    use crate::schema::transform::AnyTransform;
    use crate::util::traits::generate::TransformTrait;
    use crate::util::types::Result;
    use indexmap::IndexMap;
    use std::sync::Arc;

    impl MapSchema for CurrentSchemaRef {
        fn map_index_map<K, F: Fn(&CurrentSchemaRef, K) -> Result<Arc<GeneratedSchema>>>(
            &self,
            map: IndexMap<String, K>,
            transform: Option<Vec<AnyTransform>>,
            finalize: bool,
            func: F,
        ) -> Result<Arc<GeneratedSchema>> {
            let mut res = IndexMap::new();
            let mut current_schema: Option<CurrentSchemaRef> = None;

            for (key, value) in map {
                current_schema = if let Some(cur) = current_schema {
                    Some(CurrentSchema::child(self.clone(), Some(cur), key.clone()).into())
                } else {
                    Some(CurrentSchema::child(self.clone(), None, key.clone()).into())
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

        fn map_array<K: Clone, F: Fn(&CurrentSchemaRef, K) -> Result<Arc<GeneratedSchema>>>(
            &self,
            length: usize,
            value: K,
            transform: Option<Vec<AnyTransform>>,
            finalize: bool,
            func: F,
        ) -> Result<Arc<GeneratedSchema>> {
            let mut res = Vec::with_capacity(length as _);

            for i in 0..length {
                let current_schema = CurrentSchema::child(self.clone(), None, i.to_string()).into();
                res.push(func(&current_schema, value.clone())?);
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
}
