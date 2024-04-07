use crate::generate::generated_schema::GeneratedSchema;
use std::sync::Arc;

#[derive(Clone)]
pub enum ResolvedReference {
    Single(Arc<GeneratedSchema>),
    Multiple(Vec<Arc<GeneratedSchema>>),
    None,
}

#[cfg(feature = "map-schema")]
pub mod map_schema {
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::generate::resolved_reference::ResolvedReference;
    use anyhow::anyhow;
    use rand::seq::SliceRandom;
    use std::sync::Arc;

    impl ResolvedReference {
        pub fn none() -> Self {
            Self::None
        }

        pub fn single(schema: GeneratedSchema) -> Self {
            Self::Single(Arc::new(schema))
        }

        pub fn multiple(schemas: Vec<Arc<GeneratedSchema>>) -> Self {
            Self::Multiple(schemas)
        }

        pub fn into_random(self) -> anyhow::Result<Arc<GeneratedSchema>> {
            Ok(match self {
                Self::Single(schema) => schema,
                Self::Multiple(schemas) => schemas
                    .choose(&mut rand::thread_rng())
                    .ok_or(anyhow!("Failed to choose random schema value"))?
                    .clone(),
                Self::None => Arc::new(GeneratedSchema::None),
            })
        }

        pub fn into_vec(self) -> Option<Vec<Arc<GeneratedSchema>>> {
            match self {
                Self::Single(schema) => Some(vec![schema]),
                Self::Multiple(schemas) => Some(schemas),
                Self::None => None,
            }
        }
    }
}
