use crate::generate::generated_schema::generate::IntoGeneratedArc;
use crate::schema::any::{Any, MaybeValidAny};
use crate::schema::any_of::AnyOf;
use crate::schema::any_value::AnyValue;
use crate::schema::array::Array;
use crate::schema::file::File;
use crate::schema::flatten::{Flatten, FlattenableValue};
use crate::schema::include::Include;
use crate::schema::object::Object;
use crate::schema::schema_definition::Schema;
use crate::schema::transform::MaybeValidTransform;
use crate::validation::path::ValidationPath;
use crate::validation::result::{IterValidate, ValidationErrors, ValidationResult};
use serde_json::Value;

/// A trait for validating a schema
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
/// schema.validate().unwrap();
/// ```
pub trait Validate {
    fn validate(&self) -> ValidationResult;
}

impl Validate for Schema {
    fn validate(&self) -> ValidationResult {
        validate_any_value(&self.value, ValidationPath::root())
    }
}

fn validate_any_value(any_value: &AnyValue, path: ValidationPath) -> ValidationResult {
    match any_value {
        AnyValue::Any(any) => validate_any(any, path),
        AnyValue::String(_) => Ok(()),
        AnyValue::Number(_) => Ok(()),
        AnyValue::Bool(_) => Ok(()),
        AnyValue::Null => Ok(()),
    }
}

fn validate_any(any: &MaybeValidAny, path: ValidationPath) -> ValidationResult {
    validate_any_inner(any_into_inner(any, &path)?, path)
}

fn any_into_inner<'a>(
    any: &'a MaybeValidAny,
    path: &ValidationPath,
) -> Result<&'a Any, ValidationErrors> {
    match any {
        MaybeValidAny::Valid(inner) => Ok(inner),
        MaybeValidAny::Invalid(err) => Err(ValidationErrors::single(
            "Failed to parse schema",
            path,
            None,
            Some(serde_json::to_value(err).unwrap_or_default()),
        )),
    }
}

fn validate_any_inner(any_inner: &Any, path: ValidationPath) -> ValidationResult {
    match any_inner {
        Any::Number(number) => validate_transforms(number, path),
        Any::Integer(integer) => validate_transforms(integer, path),
        Any::Counter(counter) => validate_transforms(counter, path),
        Any::Bool(boolean) => validate_transforms(boolean, path),
        Any::String(string) => validate_transforms(string, path),
        Any::AnyOf(any_of) => validate_any_of(any_of, path),
        Any::Reference(reference) => validate_transforms(reference, path),
        Any::Plugin(plugin) => validate_transforms(plugin, path),
        Any::Array(array) => validate_array(array.as_ref(), path),
        Any::Object(object) => validate_object(object.as_ref(), path),
        Any::Flatten(flatten) => validate_flatten(flatten, path),
        Any::File(file) => validate_file(file, path).map_err(Into::into),
        Any::Include(include) => validate_include(include, path),
    }
}

fn validate_any_of(any_of: &AnyOf, path: ValidationPath) -> ValidationResult {
    ValidationResult::validate(any_of.values.iter(), |i, any| {
        validate_any_value(any, path.append("values", i))
    })
    .with(any_of.transform.iter().flatten(), |i, transform| {
        validate_transform(transform, path.append("transform", i))
    })
}

fn validate_object(object: &Object, path: ValidationPath) -> ValidationResult {
    ValidationResult::validate(object.properties.iter(), |_, (key, value)| {
        validate_any_value(value, path.append("properties", key))
    })
    .with(object.transform.iter().flatten(), |i, transform| {
        validate_transform(transform, path.append("transform", i))
    })
}

fn validate_array(array: &Array, path: ValidationPath) -> ValidationResult {
    match array {
        Array::ArrayWithValues(array_with_values) => {
            ValidationResult::validate(array_with_values.values.iter(), |i, value| {
                validate_any_value(value, path.append_single(i))
            })
        }
        Array::RandomArray(random_array) => {
            validate_any_value(&random_array.items, path.append_single("items"))
        }
    }
    .with(array.get_transform().iter().flatten(), |i, transform| {
        validate_transform(transform, path.append("transform", i))
    })
}

fn validate_flatten(flatten: &Flatten, path: ValidationPath) -> ValidationResult {
    ValidationResult::validate(flatten.values.iter(), |i, value| match value {
        FlattenableValue::Object(object) => validate_object(object, path.append("values", i)),
        FlattenableValue::Reference(_) => Ok(()),
        FlattenableValue::Plugin(_) => Ok(()),
        FlattenableValue::Array(array) => validate_array(array, path.append("values", i)),
    })
    .with(flatten.transform.iter().flatten(), |i, transform| {
        validate_transform(transform, path.append("transform", i))
    })
}

fn validate_file(file: &File, path: ValidationPath) -> ValidationResult {
    let read_result = std::fs::File::open(&file.path).map_err(|e| {
        ValidationErrors::single(
            format!("Failed to open file at path '{}'", file.path),
            &path,
            Some(e.into()),
            Some(Value::String(file.path.clone())),
        )
    });

    if read_result.is_err() {
        return read_result
            .map(|_| ())
            .with(file.transform.iter().flatten(), |i, transform| {
                validate_transform(transform, path.append("transform", i))
            });
    }

    serde_json::from_reader::<std::fs::File, Value>(read_result?)
        .map_err(|e| {
            ValidationErrors::single(
                format!("Failed to parse JSON from file at path: {}", file.path),
                &path,
                Some(e.into()),
                Some(Value::String(file.path.clone())),
            )
        })
        .map(|_| ())
        .with(file.transform.iter().flatten(), |i, transform| {
            validate_transform(transform, path.append("transform", i))
        })
}

fn validate_include(include: &Include, path: ValidationPath) -> ValidationResult {
    include
        .as_schema()
        .map_err(|e| ValidationErrors::single("Invalid include schema", &path, Some(e), None))
        .and_then(|schema| validate_any(&schema, path.clone()))
        .with(include.get_transform().iter().flatten(), |i, transform| {
            validate_transform(transform, path.append("transform", i))
        })
}

fn validate_transforms<T: IntoGeneratedArc>(value: &T, path: ValidationPath) -> ValidationResult {
    match value.get_transform() {
        Some(transforms) => ValidationResult::validate(transforms.iter(), |i, transform| {
            validate_transform(transform, path.append("transform", i))
        }),
        None => Ok(()),
    }
}

fn validate_transform(transform: &MaybeValidTransform, path: ValidationPath) -> ValidationResult {
    match transform {
        MaybeValidTransform::Valid(_) => Ok(()),
        MaybeValidTransform::Invalid(err) => Err(ValidationErrors::single(
            "Failed to parse transform schema",
            &path,
            None,
            Some(serde_json::to_value(err).unwrap_or_default()),
        )),
    }
}
