use crate::validation::path::ValidationPath;
use serde_json::Value;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Index;
use std::slice::SliceIndex;

/// A schema validation result
pub type ValidationResult = Result<(), ValidationErrors>;

/// A trait for validating an iterator of items
pub trait IterValidate {
    /// Validate an iterator of items
    ///
    /// # Arguments
    /// * `iterable` - The iterator of items to validate
    /// * `mapper` - A function that maps an item to a validation result
    ///
    /// # Returns
    /// A validation result
    fn validate<T, I, F>(iterable: T, mapper: F) -> Self
    where
        T: Iterator<Item = I>,
        F: Fn(usize, I) -> Self;

    /// Concatenate two validation results
    ///
    /// # Arguments
    /// * `other` - The other validation result
    ///
    /// # Returns
    /// The concatenated validation result
    fn concat(self, other: Self) -> Self;

    /// Validate an iterator of items and concatenate the results
    ///
    /// # Arguments
    /// * `iterable` - The iterator of items to validate
    /// * `mapper` - A function that maps an item to a validation result
    ///
    /// # Returns
    /// The concatenated validation result
    fn with<T, I, F>(self, iterable: T, mapper: F) -> Self
    where
        T: Iterator<Item = I>,
        F: Fn(usize, I) -> Self,
        Self: Sized,
    {
        self.concat(Self::validate(iterable, mapper))
    }

    /// Create a valid validation result
    ///
    /// # Returns
    /// `Ok(())`
    fn valid() -> Self
    where
        Self: Sized;

    /// Create a single validation error
    ///
    /// # Arguments
    /// * `message` - The error message
    /// * `path` - The path of the error
    /// * `cause` - The cause of the error
    /// * `invalid_value` - The invalid json value
    ///
    /// # Returns
    /// A validation result with a single error
    fn single<S: ToString>(
        message: S,
        path: &ValidationPath,
        cause: Option<anyhow::Error>,
        invalid_value: Option<Value>,
    ) -> Self
    where
        Self: Sized;

    /// Ensure a condition is true
    ///
    /// # Arguments
    /// * `condition` - The condition to ensure
    /// * `message` - The error message
    /// * `path` - The path of the error
    ///
    /// # Returns
    /// A valid result if the condition is true, otherwise a single error
    fn ensure<S>(condition: bool, message: S, path: &ValidationPath) -> Self
    where
        Self: Sized,
        S: ToString,
    {
        if condition {
            Self::valid()
        } else {
            Self::single(message, path, None, None)
        }
    }

    /// Ensure a result is ok
    ///
    /// # Arguments
    /// * `result` - The result to ensure is ok
    /// * `message` - The error message
    /// * `path` - The path of the error
    /// * `invalid_value` - The invalid json value
    ///
    /// # Returns
    /// A valid result if the result is ok, otherwise a single error
    fn ensure_ok<S, R, E>(
        result: Result<R, E>,
        message: S,
        path: &ValidationPath,
        invalid_value: Option<Value>,
    ) -> Self
    where
        Self: Sized,
        S: ToString,
        E: Into<anyhow::Error>,
    {
        match result {
            Ok(_) => Self::valid(),
            Err(e) => Self::single(message, path, Some(e.into()), invalid_value),
        }
    }

    /// Create a validation result with a single message
    ///
    /// # Arguments
    /// * `message` - The error message
    ///
    /// # Returns
    /// A validation result with a single error
    fn message<S: ToString>(message: S) -> Self
    where
        Self: Sized,
    {
        Self::single(message, &ValidationPath::root(), None, None)
    }
}

impl IterValidate for ValidationResult {
    fn validate<T, I, F>(iterable: T, mapper: F) -> ValidationResult
    where
        T: Iterator<Item = I>,
        F: Fn(usize, I) -> ValidationResult,
    {
        let errors = iterable
            .enumerate()
            .filter_map(|(i, item)| match mapper(i, item) {
                Ok(_) => None,
                Err(e) => Some(e),
            })
            .flat_map(|e| e.errors)
            .collect::<Vec<_>>();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationErrors::new(errors))
        }
    }

    fn concat(self, other: Self) -> Self {
        match (self, other) {
            (Ok(_), Ok(_)) => Ok(()),
            (Ok(_), Err(e)) => Err(e),
            (Err(e), Ok(_)) => Err(e),
            (Err(mut e1), Err(e2)) => {
                e1.errors.extend(e2.errors);
                Err(e1)
            }
        }
    }

    fn valid() -> Self {
        Ok(())
    }

    fn single<S: ToString>(
        message: S,
        path: &ValidationPath,
        cause: Option<anyhow::Error>,
        invalid_value: Option<Value>,
    ) -> Self {
        Err(ValidationErrors::single(
            message,
            path,
            cause,
            invalid_value,
        ))
    }
}

#[derive(Debug)]
/// A list of validation errors
pub struct ValidationErrors {
    /// The validation errors
    pub errors: Vec<ValidationError>,
}

impl ValidationErrors {
    /// Create a new list of validation errors
    ///
    /// # Arguments
    /// * `errors` - The list of validation errors
    pub fn new(errors: Vec<ValidationError>) -> Self {
        Self { errors }
    }

    /// Create a single validation error
    ///
    /// # Arguments
    /// * `message` - The error message
    /// * `path` - The path of the error
    /// * `cause` - The cause of the error
    /// * `invalid_value` - The invalid json value
    pub fn single<S: ToString>(
        message: S,
        path: &ValidationPath,
        cause: Option<anyhow::Error>,
        invalid_value: Option<Value>,
    ) -> Self {
        Self::new(vec![ValidationError::new(
            message,
            path,
            cause,
            invalid_value,
        )])
    }

    /// Get an iterator over the validation errors
    pub fn iter(&self) -> impl Iterator<Item = &ValidationError> {
        self.errors.iter()
    }

    /// Get the number of validation errors
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Check if the list of validation errors is empty
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Convert an error to a validation result
    pub fn from_error<E: Error>(error: E) -> Self {
        Self::single(error, &ValidationPath::root(), None, None)
    }

    /// Create a validation error with a single message
    ///
    /// # Arguments
    /// * `message` - The error message
    ///
    /// # Returns
    /// A validation errors with a single error
    pub fn message<S: ToString>(message: S) -> Self {
        Self::single(message, &ValidationPath::root(), None, None)
    }
}

impl<I: SliceIndex<[ValidationError]>> Index<I> for ValidationErrors {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.errors[index]
    }
}

impl Display for ValidationErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_assert!(!self.is_empty());

        let cause = self
            .iter()
            .enumerate()
            .map(|(i, e)| {
                format!(
                    "  Validation error #{}:\n{}",
                    i + 1,
                    e.to_string()
                        .split('\n')
                        .map(|s| format!("    {}", s))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(
            f,
            "Found {} schema violation{}:\n{}",
            self.len(),
            if self.len() > 1 { "s" } else { "" },
            cause
        )
    }
}

impl Error for ValidationErrors {}

#[derive(Debug)]
/// A validation error
pub struct ValidationError {
    /// The error message
    pub message: String,
    /// The path of the error
    pub path: String,
    /// The cause of the error
    pub cause: Option<anyhow::Error>,
    /// The invalid json value
    pub invalid_value: Option<Value>,
}

impl ValidationError {
    /// Create a new validation error
    ///
    /// # Arguments
    /// * `message` - The error message
    /// * `path` - The path of the error
    /// * `cause` - The cause of the error
    /// * `invalid_value` - The invalid json value
    pub fn new<S: ToString>(
        message: S,
        path: &ValidationPath,
        cause: Option<anyhow::Error>,
        invalid_value: Option<Value>,
    ) -> Self {
        Self {
            message: message.to_string(),
            path: path.to_string(),
            cause,
            invalid_value,
        }
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cause = self
            .cause
            .as_ref()
            .map(|e| {
                format!(
                    "\nCaused by:\n{}",
                    if e.chain().len() == 1 {
                        e.chain()
                            .map(|c| format!("  {c}"))
                            .collect::<Vec<_>>()
                            .join("\n")
                    } else {
                        e.chain()
                            .enumerate()
                            .map(|(i, c)| format!("  {i}: {c}"))
                            .collect::<Vec<_>>()
                            .join("\n")
                    }
                )
            })
            .unwrap_or_default();

        let invalid_value = self
            .invalid_value
            .as_ref()
            .map(|v| {
                format!(
                    "\nInvalid value was:\n{}",
                    serde_json::to_string_pretty(v)
                        .unwrap_or("UNKNOWN".to_string())
                        .split('\n')
                        .map(|s| format!("  {}", s))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            })
            .unwrap_or_default();

        write!(f, "{} at {}{cause}{invalid_value}", self.message, self.path)
    }
}

impl Error for ValidationError {}
