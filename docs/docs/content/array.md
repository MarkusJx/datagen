---
title: array
---

In order to generate an array, you can use the `array` generator.
This generator has two inputs:

* `length`: The length of the array, which is either a fixed or random value
* `items`: The schema of the items inside the array

## Defining the number of items generated

The `length` parameter is an `object` containing either a fixed `value`
or a lower and upper length limit to generate a random length between those two values.

### Fixed length

Generate an array with a fixed length of 10 elements:

```json
{
    "type": "array",
    "length": {
        "value": 10
    }
}
```

### Random length

Generate an array with a random length between 5 and 10 elements:

```json
{
    "type": "array",
    "length": {
        "min": 5,
        "max": 10
    }
}
```

## Example

Generate an array full of [strings](string.md) with a random length:

```json
{
    "type": "array",
    "length": {
        "min": 0,
        "max": 10000
    },
    "items": {
        "type": "string",
        "value": "test"
    }
}
```
