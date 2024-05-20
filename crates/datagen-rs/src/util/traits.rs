use log::error;

#[cfg(feature = "map-schema")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::schema::transform::Transform;
    use std::sync::Arc;

    pub trait TransformTrait {
        fn transform(
            self,
            schema: DatagenContextRef,
            value: Arc<GeneratedSchema>,
        ) -> anyhow::Result<Arc<GeneratedSchema>>;
    }

    impl TransformTrait for Vec<Transform> {
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
