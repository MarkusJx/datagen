#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::current_schema::CurrentSchemaRef;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::schema::transform::AnyTransform;
    use crate::util::types::Result;
    use std::sync::Arc;

    pub trait TransformTrait {
        fn transform(
            self,
            schema: CurrentSchemaRef,
            value: Arc<GeneratedSchema>,
        ) -> Result<Arc<GeneratedSchema>>;
    }

    impl TransformTrait for Vec<AnyTransform> {
        fn transform(
            self,
            schema: CurrentSchemaRef,
            mut value: Arc<GeneratedSchema>,
        ) -> Result<Arc<GeneratedSchema>> {
            for transform in self {
                value = transform.transform(schema.clone(), value)?;
            }

            Ok(value)
        }
    }

    pub trait ResolveRef {
        fn resolve_ref(self, schema: &CurrentSchemaRef) -> Result<Arc<GeneratedSchema>>;
    }

    impl ResolveRef for String {
        fn resolve_ref(self, schema: &CurrentSchemaRef) -> Result<Arc<GeneratedSchema>> {
            schema.resolve_ref(self)?.into_random()
        }
    }
}
