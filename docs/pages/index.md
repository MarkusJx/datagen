# datagen

A random data generator which can be used to generate random data for testing purposes.
The result schema can be defined using a JSON file.

The readme is still a work in progress, but you can check out
the [docs](https://markusjx.github.io/datagen/) for more information and examples.
Check out the [demo](https://markusjx.github.io/datagen/demo/) to see `datagen` in action.

## Similar projects

- [synth](https://github.com/shuttle-hq/synth)
- [generatedata](https://github.com/benkeen/generatedata)

This project is heavily inspired by [synth](https://github.com/shuttle-hq/synth)
but features more complex references and a plugin system.

## Usage

Simply grab
a [binary built during a workflow run](https://github.com/MarkusJx/datagen/actions/workflows/build.yml?query=branch%3Amain)
or build it yourself using `cargo build -p cli --release`.

### Docker

You can also use the Docker image `ghcr.io/markusjx/datagen` to run `datagen` in a
container.

```sh
docker run -v $(pwd):/data ghcr.io/markusjx/datagen generate /data/schema.json /data/output.json
```

Check out
the [docker image documentation](https://markusjx.github.io/datagen/docs/docker/)
for more information.

### Command-line interface

`datagen` provides a command-line interface either written in Rust or TypeScript.

#### Rust CLI

The Rust CLI is the main CLI and is the most feature-rich CLI. It is also the fastest CLI.

##### Installation

You can download a binary from
the [releases page](https://github.com/MarkusJx/datagen/releases)
or build it yourself using `cargo build -p cli --release`.
The node CLI can be installed using `npm install -g @datagen/cli`.

### Quick start

Create a file called `schema.json` with the following content:

```json
{
  "type": "object",
  "properties": {
    "name": {
      "type": "string",
      "value": "John"
    },
    "age": {
      "type": "integer",
      "value": 20
    }
  }
}
```

Then run `datagen generate schema.json` to generate data.

### Available generators

Generators are used to generate random data. The generators are defined in the
schema file as JSON objects. The following generators are available:

- [`integer`](https://markusjx.github.io/datagen/docs/generators/integer/): Generates
  random integers.
- [`number`](https://markusjx.github.io/datagen/docs/generators/number/): Generates random
  floating point numbers.
- [`string`](https://markusjx.github.io/datagen/docs/generators/string/): Generates random
  strings.
- [`bool`](https://markusjx.github.io/datagen/docs/generators/bool/): Generates random
  booleans.
- [`counter`](https://markusjx.github.io/datagen/docs/generators/counter/) Generate
  numbers which increment each time.
- [`array`](https://markusjx.github.io/datagen/docs/generators/array/): Generates random
  arrays.
- [`object`](https://markusjx.github.io/datagen/docs/generators/object/): Generates random
  objects.
- [`reference`](https://markusjx.github.io/datagen/docs/generators/reference/): Used to
  reference (or copy) other data.
- [`anyOf`](https://markusjx.github.io/datagen/docs/generators/anyof/): Chooses random
  data from a list of data.
- [`flatten`](https://markusjx.github.io/datagen/docs/generators/flatten/): Flattens an
  array or object.
- [`plugin`](https://markusjx.github.io/datagen/docs/generators/plugin/): Generates data
  using [plugins](https://markusjx.github.io/datagen/docs/plugins/).
- [`file`](https://markusjx.github.io/datagen/docs/generators/file/): Read random values
  from a JSON array inside a file.
- [`include`](https://markusjx.github.io/datagen/docs/generators/include/): Include
  external schema files.

### Schema validation

You can use `datagen validate schema.json` to validate a schema file.
Currently, the following checks are performed:

- Check if the schema is a valid JSON file.
- Check if all `type`s match the supported types.
- Check if all arguments are valid for the given type.
- Check if files included by the `include` generator exist and are valid.
- Check if files included by the `file` generator exist.
- Check if all [transformers](https://markusjx.github.io/datagen/docs/transformers/) are
  valid.

The validation also runs before generating data using `datagen generate`.
You can disable this behavior using the `--no-validate` flag.
This disables all validation checks except for the first one.
Potential schema errors will be thrown during generation, with less detailed error
messages.

A validation error includes:

- The path to the error.
- The error message.
- The invalid value, if available.
- The underlying error, if available.

### JSON schema

A JSON schema file is provided for type checking. You can find it
[here](https://markusjx.github.io/datagen/schema.json).
