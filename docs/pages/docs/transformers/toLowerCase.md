# toLowerCase

Transforms a (serialized) string to lower case.
This transformer has the following property:

-   `serializeNonStrings`: If set to `true`, non-string values will be serialized to a
    string before being transformed. Defaults to `false`.

## Example

```json
{
    "type": "string",
    "value": "HELLO",
    "transform": [
        {
            "type": "toLowerCase"
        }
    ]
}
```

Will generate:

```json
"hello"
```
