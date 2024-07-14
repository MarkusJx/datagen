use crate::generate::datagen_context::DatagenContextRef;
use crate::generate::generated_schema::GeneratedSchema;
use crate::schema::transform::MaybeValidTransform;
use indexmap::IndexMap;
use std::sync::Arc;

pub trait MapSchema {
    fn map_index_map<K, F: Fn(&DatagenContextRef, K) -> anyhow::Result<Arc<GeneratedSchema>>>(
        &self,
        map: IndexMap<String, K>,
        transform: Option<Vec<MaybeValidTransform>>,
        finalize: bool,
        func: F,
    ) -> anyhow::Result<Arc<GeneratedSchema>>;

    fn map_array<K: Clone, F: Fn(&DatagenContextRef, K) -> anyhow::Result<Arc<GeneratedSchema>>>(
        &self,
        num: usize,
        arg: K,
        transform: Option<Vec<MaybeValidTransform>>,
        finalize: bool,
        func: F,
    ) -> anyhow::Result<Arc<GeneratedSchema>>;
}

#[cfg(feature = "map-schema")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::generate::schema_mapper::MapSchema;
    use crate::schema::transform::MaybeValidTransform;
    use crate::util::traits::generate::TransformTrait;
    use indexmap::IndexMap;
    use std::sync::Arc;

    impl MapSchema for DatagenContextRef {
        fn map_index_map<
            K,
            F: Fn(&DatagenContextRef, K) -> anyhow::Result<Arc<GeneratedSchema>>,
        >(
            &self,
            map: IndexMap<String, K>,
            transform: Option<Vec<MaybeValidTransform>>,
            finalize: bool,
            func: F,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            let mut res = IndexMap::new();
            let mut current_schema: Option<DatagenContextRef> = None;

            for (key, value) in map {
                current_schema = if let Some(cur) = current_schema {
                    Some(self.child(Some(cur), &key)?)
                } else {
                    Some(self.child(None, &key)?)
                };

                res.insert(key, func(current_schema.as_ref().unwrap(), value)?);
            }

            if finalize {
                let mut res = GeneratedSchema::Object(res).into();
                if let Some(transform) = transform {
                    res = transform.transform(self.clone(), res)?;
                }

                self.finalize(res)
            } else {
                Ok(GeneratedSchema::Object(res).into())
            }
        }

        fn map_array<
            K: Clone,
            F: Fn(&DatagenContextRef, K) -> anyhow::Result<Arc<GeneratedSchema>>,
        >(
            &self,
            length: usize,
            value: K,
            transform: Option<Vec<MaybeValidTransform>>,
            finalize: bool,
            func: F,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            let mut res = Vec::with_capacity(length as _);

            for i in 0..length {
                let current_schema = self.child(None, &i.to_string())?;
                res.push(func(&current_schema, value.clone())?);
            }

            if finalize {
                let mut res = GeneratedSchema::Array(res).into();
                if let Some(transform) = transform {
                    res = transform.transform(self.clone(), res)?;
                }

                self.finalize(res)
            } else {
                Ok(GeneratedSchema::Array(res).into())
            }
        }
    }
}
