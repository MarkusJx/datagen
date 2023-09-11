# datagen

A random data generator which can be used to generate random data for testing purposes.
The result schema can be defined using a JSON file.

The readme is still a work in progress, but you can check out
the [docs](https://markusjx.github.io/datagen/) for more information and examples.

## Similar projects

* [synth](https://github.com/shuttle-hq/synth)
* [generatedata](https://github.com/benkeen/generatedata)

This project is heavily inspired by [synth](https://github.com/shuttle-hq/synth)
but features more complex references and a plugin system.

## Usage

Simply grab a [binary built during a workflow run](https://github.com/MarkusJx/datagen/actions/workflows/build.yml?query=branch%3Amain)
or build it yourself using `cargo build -p cli --release`.

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
