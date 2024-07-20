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
#[cfg_attr(feature = "serialize", serde(untagged, deny_unknown_fields))]
pub enum ArrayLength {
    ShortConstant(u32),
    Constant { value: u32 },
    Random { min: u32, max: u32 },
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(untagged, deny_unknown_fields))]
pub enum Array {
    RandomArray(RandomArray),
    ArrayWithValues(ArrayWithValues),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct RandomArray {
    pub length: ArrayLength,
    pub items: AnyValue,
    pub transform: Option<Vec<MaybeValidTransform>>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct ArrayWithValues {
    pub values: Vec<AnyValue>,
    pub transform: Option<Vec<MaybeValidTransform>>,
}

impl GetTransform for RandomArray {
    fn get_transform(&self) -> Option<Vec<MaybeValidTransform>> {
        self.transform.clone()
    }
}

impl GetTransform for ArrayWithValues {
    fn get_transform(&self) -> Option<Vec<MaybeValidTransform>> {
        self.transform.clone()
    }
}

impl GetTransform for Array {
    fn get_transform(&self) -> Option<Vec<MaybeValidTransform>> {
        match self {
            Array::RandomArray(random_array) => random_array.get_transform(),
            Array::ArrayWithValues(array_with_values) => array_with_values.get_transform(),
        }
    }
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::datagen_context::DatagenContextRef;
    use crate::generate::generated_schema::generate::IntoGeneratedArc;
    use crate::generate::generated_schema::{GeneratedSchema, IntoRandom};
    use crate::generate::schema_mapper::MapSchema;
    use crate::schema::array::{Array, ArrayLength};
    use rand::Rng;
    use std::sync::Arc;

    use super::{ArrayWithValues, RandomArray};

    impl ArrayLength {
        pub fn get_length(&self) -> u32 {
            match self {
                ArrayLength::ShortConstant(value) => *value,
                ArrayLength::Constant { value } => *value,
                ArrayLength::Random { min, max } => {
                    let mut rng = rand::thread_rng();
                    rng.gen_range(*min..=*max)
                }
            }
        }
    }

    impl IntoGeneratedArc for Array {
        fn into_generated_arc(
            self,
            schema: DatagenContextRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            match self {
                Array::RandomArray(random_array) => random_array.into_generated_arc(schema),
                Array::ArrayWithValues(array_with_values) => {
                    array_with_values.into_generated_arc(schema)
                }
            }
        }
    }

    impl IntoGeneratedArc for RandomArray {
        fn into_generated_arc(
            self,
            schema: DatagenContextRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            let length = self.length.get_length();
            schema.map_array(length as _, self.items, None, false, |cur, value| {
                value.into_random(cur.clone())
            })
        }
    }

    impl IntoGeneratedArc for ArrayWithValues {
        fn into_generated_arc(
            self,
            schema: DatagenContextRef,
        ) -> anyhow::Result<Arc<GeneratedSchema>> {
            Ok(GeneratedSchema::Array(
                self.values
                    .into_iter()
                    .map(|e| e.into_generated_arc(schema.clone()))
                    .collect::<anyhow::Result<Vec<_>>>()?,
            )
            .into())
        }
    }
}

#[cfg(feature = "validate-schema")]
pub mod validate {
    use crate::schema::array::{Array, ArrayWithValues, RandomArray};
    use crate::validation::path::ValidationPath;
    use crate::validation::result::{IterValidate, ValidationResult};
    use crate::validation::validate::{Validate, ValidateGenerateSchema};

    impl ValidateGenerateSchema for RandomArray {
        fn validate_generate_schema(&self, path: &ValidationPath) -> ValidationResult {
            self.items.validate(&path.append_single("items"))
        }
    }

    impl ValidateGenerateSchema for ArrayWithValues {
        fn validate_generate_schema(&self, path: &ValidationPath) -> ValidationResult {
            ValidationResult::validate(self.values.iter(), |i, value| {
                value.validate(&path.append("items", i))
            })
        }
    }

    impl Validate for Array {
        fn validate(&self, path: &ValidationPath) -> ValidationResult {
            match self {
                Array::RandomArray(random_array) => random_array.validate(path),
                Array::ArrayWithValues(array_with_values) => array_with_values.validate(path),
            }
        }
    }
}
