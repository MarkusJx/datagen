# @datagen-rs/cli

The `@datagen-rs/cli` package provides a command line interface for generating data
using `datagen`.

## Installation

```bash
npm install -g @datagen-rs/cli
```

## Usage

```text
datagen <schema> [output]

Positionals:
  schema  The schema file                                               [string]

Options:
  --help     Show help                                                 [boolean]
  --version  Show version number                                       [boolean]
  --output   The output file                                            [string]
```

The `schema` argument must be a path to a valid schema file. The `output` argument
can be set to a file to write to. If no `output` is provided, the output will be
written to `stdout`. Check the [generators documentation](https://markusjx.github.io/datagen/docs/generators/)
or the [examples](https://markusjx.github.io/datagen/examples/) for more information
on how to write a schema file.
