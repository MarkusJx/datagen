# datagen

A random data generator.

Check out the [docs](docs) for more information on how
to use `datagen`. Check out the [examples](examples) for
examples and try out the [demo](demo)
to see `datagen` in action.

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
