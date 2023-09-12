---
title: regexFilter
---

The `regexFilter` transformer will filter out values that do not match a regular expression.

It has the following properties:

* `pattern` - The regular expression pattern to match against.
* `serializeNonStrings` - If `true`, non-string values will be converted to strings
before matching against the regular expression. Defaults to `false`. If set to`false`,
passing non-string values will throw an error.

## Example

```json
{
  "type": "string",
  "value": "hello",
  "transform": [
    {
      "type": "regexFilter",
      "pattern": "^[a-z]+$"
    }
  ]
}
```
