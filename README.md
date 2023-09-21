# datagen

A random data generator which can be used to generate random data for testing purposes.
The result schema can be defined using a JSON file.

The readme is still a work in progress, but you can check out
the [docs](https://markusjx.github.io/datagen/) for more information and examples.
Check out the [demo](https://markusjx.github.io/datagen/demo/) to see `datagen` in action.

## Similar projects

* [synth](https://github.com/shuttle-hq/synth)
* [generatedata](https://github.com/benkeen/generatedata)

This project is heavily inspired by [synth](https://github.com/shuttle-hq/synth)
but features more complex references and a plugin system.

## Usage

Simply grab a [binary built during a workflow run](https://github.com/MarkusJx/datagen/actions/workflows/build.yml?query=branch%3Amain)
or build it yourself using `cargo build -p cli --release`.

### Command-line interface

`datagen` provides a command-line interface either written in Rust or TypeScript.

#### Rust CLI

The Rust CLI is the main CLI and is the most feature-rich CLI. It is also the fastest CLI.

##### Installation

You can download a binary from the [releases page](https://github.com/MarkusJx/datagen/releases)
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

* [`integer`](https://markusjx.github.io/datagen/docs/content/integer/): Generates random integers.
* [`number`](https://markusjx.github.io/datagen/docs/content/number/): Generates random floating point numbers.
* [`string`](https://markusjx.github.io/datagen/docs/content/string/): Generates random strings.
* [`bool`](https://markusjx.github.io/datagen/docs/content/bool/): Generates random booleans.
* [`array`](https://markusjx.github.io/datagen/docs/content/array/): Generates random arrays.
* [`object`](https://markusjx.github.io/datagen/docs/content/object/): Generates random objects.
* [`reference`](https://markusjx.github.io/datagen/docs/content/reference/): Used to reference (or copy) other data.
* [`anyOf`](https://markusjx.github.io/datagen/docs/content/anyof/): Chooses random data from a list of data.
* [`flatten`](https://markusjx.github.io/datagen/docs/content/flatten/): Flattens an array or object.
* [`plugin`](https://markusjx.github.io/datagen/docs/content/plugin/): Generates data using [plugins](https://markusjx.github.io/datagen/docs/plugins/).
