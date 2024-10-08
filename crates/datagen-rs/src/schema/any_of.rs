use crate::schema::any_value::AnyValue;
use crate::schema::transform::MaybeValidTransform;
use crate::util::traits::GetTransform;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
pub struct AnyOf {
    /// The values to choose from
    pub values: Vec<AnyValue>,
    /// The number of values to return. Defaults to 1.
    /// If more than 1 specified, an array will be returned.
    /// The actual number of values returned is [1;num),
    /// if allowNull is false, [0;num) otherwise.
    /// Returns an error if num > values.length.
    pub num: Option<i64>,
    /// Whether to allow `null` to be returned.
    /// Will choose a value from the values + null.
    pub allow_null: Option<bool>,
    pub transform: Option<Vec<MaybeValidTransform>>,
}

impl GetTransform for AnyOf {
    fn get_transform(&self) -> Option<Vec<MaybeValidTransform>> {
        self.transform.clone()
    }
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGeneratedArc;
    use crate::generate::generated_schema::{GeneratedSchema, IntoRandom};
    use crate::schema::any_of::AnyOf;
    use rand::seq::SliceRandom;
    use rand::Rng;
    use std::cmp::Ordering;
    use std::sync::Arc;

    impl IntoGeneratedArc for AnyOf {
        fn into_generated_arc(
            mut self,
            schema: DatagenContextRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            self.values.shuffle(&mut rand::thread_rng());
            let min = if self.allow_null.unwrap_or(false) {
                0
            } else {
                1
            };

            let mut num = self.num.unwrap_or(1);
            match num.cmp(&0) {
                Ordering::Equal => num = self.values.len() as i64,
                Ordering::Less => {
                    num = rand::thread_rng().gen_range(min..=self.values.len() as i64)
                }
                _ => {}
            }

            if num > self.values.len() as _ {
                return Err(
                    anyhow::anyhow!(
                        "Maximum number of elements requested by anyOf is greater than the number of values: {num} vs {}",
                        self.values.len()
                    )
                    .context(anyhow::anyhow!("Invalid schema at {}", schema.path()?))
                );
            }

            let values = self
                .values
                .drain(0..num as usize)
                .map(|value| value.into_random(schema.clone()))
                .collect::<anyhow::Result<Vec<_>>>()?;

            if values.is_empty() {
                Ok(Arc::new(GeneratedSchema::None))
            } else if values.len() == 1 {
                Ok(values[0].clone())
            } else {
                Ok(Arc::new(GeneratedSchema::Array(values)))
            }
        }
    }
}

#[cfg(feature = "validate-schema")]
pub mod validate {
    use crate::schema::any_of::AnyOf;
    use crate::validation::path::ValidationPath;
    use crate::validation::result::{IterValidate, ValidationResult};
    use crate::validation::validate::{Validate, ValidateGenerateSchema};

    impl ValidateGenerateSchema for AnyOf {
        fn validate_generate_schema(&self, path: &ValidationPath) -> ValidationResult {
            ValidationResult::validate(self.values.iter(), |i, value| {
                value.validate(&path.append("values", i))
            })
        }
    }
}
