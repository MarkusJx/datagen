# counter

A generator which generates a number that increments each time it is called.

## Arguments

- `step` (optional): The amount to increment the counter by each time it is called. Defaults to `1`. Can be negative.
- `start` (optional): The number to start the counter at. Defaults to `0`.
- `pathSpecific` (optional): Whether to use a counter specific to the path of the
element generated. Defaults to `false`. If set to `true`, the counter will be
incremented each time the generator is called with the same path.

## Example

### Create an array containing the current index

```json
{
  "type": "array",
  "length": {
    "min": 0,
    "max": 10000
  },
  "items": {
    "type": "counter",
    "start": 0
  }
}
```

### Create an array containing the current index, using a counter specific to the path

```json
{
  "type": "array",
  "length": {
    "min": 0,
    "max": 10000
  },
  "items": {
    "type": "object",
    "properties": {
      "index": {
        "type": "counter",
        "start": 0,
        "pathSpecific": true
      },
      "indexTimesTwo": {
        "type": "counter",
        "start": 0,
        "step": 2,
        "pathSpecific": true
      }
    }
  }
}
```
