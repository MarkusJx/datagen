---
title: Transformers
---

Transformers are used to transform data. They are used in the `transform` property of a schema.
Transformers are supported by every [generator](../content/index.md) except for shorthand generators.

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

* [`filter`](filter.md)
* [`filterNonNull`](filterNonNull.md)
* [`toString`](toString.md)
* [`toLowerCase`](toLowerCase.md)
* [`toUpperCase`](toUpperCase.md)
* [`plugin`](plugin.md)
