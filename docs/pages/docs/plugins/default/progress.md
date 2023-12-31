# progress-plugin

The progress plugin is a native plugin providing progress for the generation process.

## Usage

The easiest way to use the progress plugin is to call `ProgressPlugin::with_schema()`
while providing a schema and a callback function:

```rust
use std::error::Error;

use datagen_rs::schema::schema_definition::Schema;
use progress_plugin::{PluginWithSchemaResult, ProgressPlugin};
use datagen_rs::util::helpers::generate_random_data;

fn generate(schema: Schema) -> Result<(), Box<dyn Error>> {
    let PluginWithSchemaResult {
        schema,
        plugins,
    } = ProgressPlugin::with_schema(schema, |current, total| {
        println!("{} / {}", current, total);
    })?;

    let generated = generate_random_data(schema, Some(plugins))?;
    println!("{}", generated);
}
```

## How the number of elements is calculated

The progress is calculated by first generating a list of array lengths
for all arrays in the schema.

If the array length is a fixed value,
the progress is calculated by multiplying the fixed value with the
number of elements in the array. This is a recursive process, so
if the array contains another array, the progress is calculated
by multiplying the length of the array with the length of the
inner array.

If the array length is a random value, the array length is calculated
by generating a random value between the minimum and maximum value
and storing this value in a list containing all array lengths.
Then this value is multiplied with the number of elements in the array.

The number of elements in [`objects`](https://markusjx.github.io/datagen/docs/generators/object/) and
[`anyOf`](https://markusjx.github.io/datagen/docs/generators/anyof/) is also taken into account.
