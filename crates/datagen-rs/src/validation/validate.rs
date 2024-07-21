use crate::util::traits::GetTransform;
use crate::validation::path::ValidationPath;
use crate::validation::result::{IterValidate, ValidationResult};

/// Validate a generate schema.
/// Used for automatically implementing [`Validate`] for types that implement this trait.
pub trait ValidateGenerateSchema {
    /// Validate the schema.
    /// This **must not** validate any transforms.
    fn validate_generate_schema(&self, path: &ValidationPath) -> ValidationResult;
}

/// A trait for validating a schema.
/// This gets automatically implemented for all types that implement
/// [`ValidateGenerateSchema`] and [`GetTransform`].
///
/// # Example
/// ```no_run
/// use datagen_rs::schema::any::{MaybeValidAny, Any};
/// use datagen_rs::schema::any_value::AnyValue;
/// use datagen_rs::schema::object::Object;
/// use datagen_rs::schema::schema_definition::Schema;
/// use datagen_rs::validation::validate::Validate;
///
/// let schema = Schema {
///     options: None,
///     value: AnyValue::Any(MaybeValidAny::Valid(Any::Object(Box::new(Object {
///         properties: vec![("key".to_string(), AnyValue::String("value".to_string()))].into_iter().collect(),
///         transform: None,
///     })))),
/// };
///
/// schema.validate_root().unwrap();
/// ```
pub trait Validate {
    /// Validate the root of the schema.
    fn validate_root(&self) -> ValidationResult {
        self.validate(&ValidationPath::root())
    }

    /// Validate a schema or a part of a schema.
    /// this method should validate the schema, all of its children
    /// and all of its transforms.
    fn validate(&self, path: &ValidationPath) -> ValidationResult;
}

impl<T> Validate for T
where
    T: ValidateGenerateSchema + GetTransform,
{
    fn validate(&self, path: &ValidationPath) -> ValidationResult {
        self.validate_generate_schema(path)
            .concat(ValidationResult::validate(
                self.get_transform().iter().flatten(),
                |i, transform| transform.validate(&path.append("transform", i)),
            ))
    }
}

impl<T> Validate for Option<T>
where
    T: Validate,
{
    fn validate(&self, path: &ValidationPath) -> ValidationResult {
        match self {
            Some(value) => value.validate(path),
            None => Ok(()),
        }
    }
}
