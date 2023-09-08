---
title: object
---

In order to generate an object, you can use the `object` generator, which has the following input:

* `properties`: An object containing the properties of the object

## Example

Generate an object with two properties:

```json
{
    "type": "object",
    "properties": {
        "field1": {
            "type": "string",
            "value": "test"
        },
        "field2": {
            "type": "string",
            "value": "test"
        }
    }
}
```
