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
