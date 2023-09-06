#[cfg(any(feature = "schema", feature = "serialize"))]
use crate::schema::schema_definition::Schema;
#[cfg(any(feature = "schema", feature = "serialize"))]
use crate::util::types::Result;
#[cfg(feature = "schema")]
use schemars::schema_for;
#[cfg(any(feature = "schema", feature = "serialize"))]
use std::fs::File;
#[cfg(any(feature = "schema", feature = "serialize"))]
use std::path::Path;

#[cfg(feature = "schema")]
pub fn write_json_schema<P: AsRef<Path>>(path: P) -> Result<()> {
    let file = File::create(path)?;
    let schema = schema_for!(Schema);

    serde_json::to_writer_pretty(file, &schema).map_err(|e| e.into())
}

#[cfg(feature = "serialize")]
pub fn read_schema<P: AsRef<Path>>(path: P) -> Result<Schema> {
    let file = File::open(path)?;
    let schema: Schema = serde_json::from_reader(file)?;

    Ok(schema)
}
