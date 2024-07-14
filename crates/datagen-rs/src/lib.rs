/// This crate provides a library for generating data for testing and benchmarking.
///
/// # Example
/// ```
/// use datagen_rs::util::helpers::{generate_random_data, read_schema};
/// use serde_json::{json, from_value};
///
/// let schema_json = json!({
///    "type": "object",
///     "properties": {
///         "name": {
///             "type": "string",
///             "generator": {
///                 "type": "firstName",
///             },
///         },
///     },
/// });
///
/// let schema = from_value(schema_json).unwrap();
/// let data = generate_random_data(schema, None).unwrap();
/// println!("{}", data);
/// ```
pub mod generate;
pub mod plugins;
pub mod schema;
#[cfg(test)]
mod tests;
pub mod transform;
pub mod util;
#[cfg(feature = "validate-schema")]
pub mod validation;
