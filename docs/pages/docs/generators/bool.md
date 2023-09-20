# bool

The `bool` generator generates a random or fixed boolean value.
It has two modes: `constant` and `random`.

## Constant mode

In constant mode, the `bool` generator generates a boolean value.
This mode has one input:

- `value`: The fixed boolean value to generate

### Example

Generate the boolean `true`:

```json
{
  "type": "bool",
  "value": true
}
```

### Shorthand

The `bool` generator can also be used in shorthand mode.
In this mode, the generator will generate a boolean value.

#### Example

Generate the boolean `false` inside an object:

```json
{
  "type": "object",
  "properties": {
    "boolean": false
  }
}
```

## Random mode

In random mode, the `bool` generator generates a random boolean value.
This mode has one optional input:

- `probability`: The probability of generating `true`. Defaults to `0.5`.

### Example

Generate a random boolean value:

```json
{
  "type": "bool"
}
```

Generate a random boolean value with a custom probability:

```json
{
  "type": "bool",
  "probability": 0.75
}
```
