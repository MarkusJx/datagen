---
title: toUpperCase
---

Transforms a (serialized) string to upper case.
This transformer has the following property:

* `serializeNonStrings`: If set to `true`, non-string values will be serialized to a
  string before being transformed. Defaults to `false`.

## Example

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
