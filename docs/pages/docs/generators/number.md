# number

The `number` generator generates a random or fixed floating point number.
It has two modes: `constant` and `random`.

## Constant mode

In constant mode, the `number` generator generates a fixed floating point number.
This mode has one input:

- `value`: The fixed floating point number to generate

### Example

Generate the number `10.5`:

```json
{
  "type": "number",
  "value": 10.5
}
```

### Shorthand

The `number` generator can also be used in shorthand mode.
In this mode, the generator will generate a fixed floating point number.

#### Example

Generate the number `10.5` inside an object:

```json
{
  "type": "object",
  "properties": {
    "num": 10.5
  }
}
```

## Random mode

In random mode, the `number` generator generates a random floating point number.
This mode has three optional inputs:

- `min`: The minimum value to generate. Defaults to [`f64::MIN`](https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.MIN)
- `max`: The maximum value to generate. Defaults to [`f64::MAX`](https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.MAX)
- `precision`: The precision of the generated number. Will not be used if not set.

### Example

Generate a random floating point number:

```json
{
  "type": "number"
}
```

Generate a random floating point number between `0.0` and `100.0` with a precision of `2`:

```json
{
  "type": "number",
  "min": 0.0,
  "max": 100.0,
  "precision": 2
}
```
