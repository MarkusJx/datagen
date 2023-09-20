# filterNonNull

The `filterNonNull` transformer removes all `null` values from an array or object.

The `filterNonNull` transformer has no additional properties.

## Examples

### Remove all `null` values from an array

```json
{
  "type": "array",
  "value": ["hello", null, "world"],
  "transform": [
    {
      "type": "filterNonNull"
    }
  ]
}
```

Will result in:

```json
["hello", "world"]
```

### Remove all names matching `John` from an object

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
  },
  "transform": [
    {
      "type": "filter",
      "field": "ref:./name",
      "operator": "notEquals",
      "other": "John"
    },
    {
      "type": "filterNonNull"
    }
  ]
}
```

Will result in:

```json
{
  "age": 20
}
```
