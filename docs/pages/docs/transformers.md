# Transformers

Transformers are used to transform data. They are used in the `transform` property of a schema.
Transformers are supported by every [generator](generators.md) except for shorthand generators.

## Usage

In order to apply a transformer to a value, you need to add the `transform` property
to that value. The `transform` property is an array of transformers that will be applied
to the value in order.

```json
{
  "type": "string",
  "value": "hello",
  "transform": [
    {
      "type": "toUpperCase"
    }
  ]
}
```

Will generate:

```json
"HELLO"
```

## Available Transformers

- [`filter`](transformers/filter.md)
- [`filterNonNull`](transformers/filterNonNull.md)
- [`toString`](transformers/toString.md)
- [`toLowerCase`](transformers/toLowerCase.md)
- [`toUpperCase`](transformers/toUpperCase.md)
- [`plugin`](transformers/plugin.md)
- [`sort`](transformers/sort.md)
- [`regexFilter`](transformers/regexFilter.md)
