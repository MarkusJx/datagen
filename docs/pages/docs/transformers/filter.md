# filter

The `filter` transformer is used to filter values by converting
non-matching values to `null`.

The `filter` transformer has the following properties:

-   `field`: Optional. The field or the reference to a field to filter on.
    If not specified, the current value will be used.
-   `operator`: The operator which will be used to compare the value with the other value.
    The operator may be one of:
    -   `equals`: The value must be equal to the other value.
    -   `notEquals`: The value must not be equal to the other value.
-   `other`: The other value to compare the value with.

## Example

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
        }
    ]
}
```

Will result in

```json
{
    "name": null,
    "age": 20
}
```

You can combine this filter with [`filterNonNull`](filterNonNull.md) to remove
all names matching `John`.