use crate::schema::transform::AnyTransform;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "generate")]
use std::sync::atomic::AtomicI64;

#[cfg(feature = "generate")]
static mut COUNTER: AtomicI64 = AtomicI64::new(0);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Counter {
    pub step: Option<i64>,
    pub transform: Option<Vec<AnyTransform>>,
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::current_schema::CurrentSchemaRef;
    use crate::generate::generated_schema::generate::IntoGenerated;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::schema::counter::{Counter, COUNTER};
    use crate::schema::transform::AnyTransform;
    use crate::util::types::Result;
    use std::sync::atomic::Ordering;

    impl IntoGenerated for Counter {
        fn into_generated(self, _schema: CurrentSchemaRef) -> Result<GeneratedSchema> {
            let value = unsafe { COUNTER.fetch_add(1, Ordering::SeqCst) };
            Ok(GeneratedSchema::Integer(value))
        }

        fn get_transform(&self) -> Option<Vec<AnyTransform>> {
            self.transform.clone()
        }
    }
}
