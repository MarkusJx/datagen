use crate::generate::current_schema::CurrentSchemaRef;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::GeneratedSchema;
#[cfg(feature = "generate")]
use crate::schema::transform::AnyTransform;
#[cfg(feature = "generate")]
use crate::util::types::Result;
#[cfg(feature = "generate")]
use std::sync::Arc;

#[cfg(feature = "generate")]
pub trait TransformTrait {
    fn transform(
        self,
        schema: CurrentSchemaRef,
        value: Arc<GeneratedSchema>,
    ) -> Result<Arc<GeneratedSchema>>;
}

#[cfg(feature = "generate")]
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

#[cfg(feature = "generate")]
pub trait ResolveRef {
    fn resolve_ref(self, schema: &CurrentSchemaRef) -> Result<Arc<GeneratedSchema>>;
}

#[cfg(feature = "generate")]
impl ResolveRef for String {
    fn resolve_ref(self, schema: &CurrentSchemaRef) -> Result<Arc<GeneratedSchema>> {
        schema.resolve_ref(self)?.into_random()
    }
}
