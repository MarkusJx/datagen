#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
pub struct RandomRemoveTransform {
    /// The fields to possibly remove.
    /// If this is not specified, all fields from
    /// the object may be removed.
    /// Only applicable to objects.
    pub fields: Option<Vec<String>>,
    /// The minimum number of items to remove.
    /// Only applicable to arrays and objects.
    pub min: Option<usize>,
    /// The maximum number of items to remove.
    /// Only applicable to arrays and objects.
    pub max: Option<usize>,
    /// The chance that an item will be removed.
    /// Only applicable to single items.
    pub chance: Option<f64>,
}

#[cfg(feature = "map-schema")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::transform::random_remove::RandomRemoveTransform;
    use crate::util::traits::generate::TransformTrait;
    use rand::seq::SliceRandom;
    use rand::{thread_rng, Rng};
    use std::sync::Arc;

    impl TransformTrait for RandomRemoveTransform {
        fn transform(
            self,
            _schema: DatagenContextRef,
            value: Arc<GeneratedSchema>,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            let mut rng = thread_rng();

            match value.as_ref() {
                GeneratedSchema::Array(arr) => {
                    let mut arr = arr.clone();
                    let min = self.min.unwrap_or(0);
                    let max = self.max.unwrap_or(arr.len());
                    let num = rng.gen_range(min..=max);
                    for _ in 0..num {
                        arr.remove(rng.gen_range(0..arr.len()));
                    }
                    Ok(Arc::new(GeneratedSchema::Array(arr)))
                }
                GeneratedSchema::Object(obj) => {
                    let mut obj = obj.clone();
                    let mut fields = self
                        .fields
                        .clone()
                        .unwrap_or_else(|| obj.keys().cloned().collect());
                    fields.shuffle(&mut rng);

                    let min = self.min.unwrap_or(0);
                    let max = self.max.unwrap_or(fields.len());
                    let num = rng.gen_range(min..=max);
                    for _ in 0..num {
                        let Some(field) = fields.pop() else {
                            break;
                        };

                        obj.shift_remove(&field);
                    }

                    Ok(Arc::new(GeneratedSchema::Object(obj)))
                }
                _ => {
                    if rng.gen_bool(self.chance.unwrap_or(0.5)) {
                        Ok(Arc::new(GeneratedSchema::None))
                    } else {
                        Ok(value)
                    }
                }
            }
        }
    }
}

#[cfg(feature = "validate-schema")]
pub mod validate {
    use crate::transform::random_remove::RandomRemoveTransform;
    use crate::validation::path::ValidationPath;
    use crate::validation::result::{IterValidate, ValidationResult};
    use crate::validation::validate::Validate;

    impl Validate for RandomRemoveTransform {
        fn validate(&self, path: &ValidationPath) -> ValidationResult {
            ValidationResult::valid()
                .concat(if let Some(max) = self.max {
                    ValidationResult::ensure(
                        max >= self.min.unwrap_or(0),
                        "max must be greater than or equal to min",
                        path,
                    )
                } else {
                    Ok(())
                })
                .concat(if let Some(chance) = self.chance {
                    ValidationResult::ensure(
                        chance >= 0.0 && chance <= 1.0,
                        "chance must be between 0 and 1",
                        path,
                    )
                } else {
                    Ok(())
                })
        }
    }
}
