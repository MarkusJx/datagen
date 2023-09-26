use crate::generate::generated_schema::GeneratedSchema;
use crate::util::types::Result;
use rand::prelude::SliceRandom;
use std::sync::Arc;

pub enum ResolvedReference {
    Single(Arc<GeneratedSchema>),
    Multiple(Vec<Arc<GeneratedSchema>>),
    None,
}

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

    pub fn into_random(self) -> Result<Arc<GeneratedSchema>> {
        Ok(match self {
            Self::Single(schema) => schema,
            Self::Multiple(schemas) => schemas
                .choose(&mut rand::thread_rng())
                .ok_or("Failed to choose random schema value")?
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
