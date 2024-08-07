use crate::schema::transform::MaybeValidTransform;
use log::error;

/// Trait for getting a transform from a schema.
pub trait GetTransform {
    /// Get the transform from the schema.
    fn get_transform(&self) -> Option<Vec<MaybeValidTransform>>;
}

#[cfg(feature = "map-schema")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::GeneratedSchema;
    use std::sync::Arc;

    pub trait TransformTrait {
        fn transform(
            self,
            schema: DatagenContextRef,
            value: Arc<GeneratedSchema>,
        ) -> anyhow::Result<Arc<GeneratedSchema>>;
    }

    impl<T: TransformTrait> TransformTrait for Vec<T> {
        fn transform(
            self,
            schema: DatagenContextRef,
            mut value: Arc<GeneratedSchema>,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            for transform in self {
                value = transform.transform(schema.clone(), value)?;
            }

            Ok(value)
        }
    }

    pub trait ResolveRef {
        fn resolve_ref(self, schema: &DatagenContextRef) -> anyhow::Result<Arc<GeneratedSchema>>;
    }

    impl ResolveRef for String {
        fn resolve_ref(self, schema: &DatagenContextRef) -> anyhow::Result<Arc<GeneratedSchema>> {
            schema.resolve_ref(self.as_str())?.into_random()
        }
    }
}

pub trait LogError<T> {
    fn log_error(self) -> anyhow::Result<T>;
}

impl<T, E> LogError<T> for Result<T, E>
where
    E: Into<anyhow::Error>,
{
    fn log_error(self) -> anyhow::Result<T> {
        let result = self.map_err(Into::into);
        if let Some(e) = result.as_ref().err() {
            error!("{:?}", e);
        }

        result
    }
}
