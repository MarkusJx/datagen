# datagen-rs

A Rust library for generating random data from a JSON schema.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
datagen-rs = "0.1.0"
```

## Usage

### Generate data from a schema file

```rust
use datagen_rs::util::helpers::{generate_random_data, read_schema};

fn main() {
    let schema = read_schema("path/to/schema.json").unwrap();
    let data = generate_random_data(schema, None).unwrap();
    println!("{}", data);
}
```

### Generate data from a schema string

```rust
use datagen_rs::util::helpers::{generate_random_data_from_str, read_schema};

fn main() {
    let schema: Schema = serde_json::from_str(r#"
    {
        "type": "string",
        "value": "test
    }
    "#).unwrap();
    let data = generate_random_data_from_str(&schema, None).unwrap();
    println!("{}", data);
}
```

### Write the JSON schema to a file

```rust
use datagen_rs::util::helpers::write_json_schema;

fn main() {
    write_json_schema("path/to/schema.json").unwrap();
}
```

## Features

| Feature         | Description                                                          | Depends on                            |
|-----------------|----------------------------------------------------------------------|---------------------------------------|
| `plugin`        | Enables loading plugins                                              |                                       |
| `native-plugin` | Enables loading plugins written in Rust                              | `plugin`                              |
| `serialize`     | Enables serialization/deserialization for all structs using `serde`. |                                       |
| `map-schema`    | Enables data generation for several types                            | `serialize`                           |
| `generate`      | Enables data generation for all types                                | `map-schema`                          |
| `schema`        | Enables JSON schema generation using `schemars`                      | `serialize`                           |
| `all`           | Enables all features                                                 | `native-plugin`, `generate`, `schema` |
