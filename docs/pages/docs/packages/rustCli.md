# datagen-rs-cli

The command line interface for the `datagen-rs` crate.

## Installation

```bash
cargo install datagen-rs-cli
```

## Usage

```text
Usage: datagen <COMMAND>

Commands:
  write-json-schema  Write the JSON schema to a file
  generate           Generate random data from a schema
  help               Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Generate data

```text
datagen generate <SCHEMA_FILE> [OUT_FILE]
```

This will generate data from the given schema file and write it to the given
output file. If no output file is given, the data will be written to stdout.

### Write JSON schema

```text
datagen.exe write-json-schema <PATH>
```

This will write the JSON schema to the given path.
