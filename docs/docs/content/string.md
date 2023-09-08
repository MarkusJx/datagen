---
title: string
---

A `string` value. This generator can be accessed by a [shorthand](#shorthand) command or
using arguments, defining either a fixed value or a generated value.
The `string` generator may take the following arguments:

* `value`: A fixed value to insert
* `generator`: A [generator](#generators) for this string

## Shorthand

The string shorthand, which inserts a fixed value where placed, can be inserted almost
everywhere. Shorthand values can also be [references](#references), replacing the value
with the value of the referenced field.

#### Example

Insert a fixed string value names "city" inside an object:

```json
{
  "type": "object",
  "properties": {
    "city": "New York"
  }
}
```

## Generators

The string generators are used to create random string values.
Internally, `datagen` uses [`fake-rs`](https://github.com/cksac/fake-rs)
to generate the values.

### uuid

The `uuid` generator generates a random
[universally unique identifier](https://en.wikipedia.org/wiki/Universally_unique_identifier):

```json
{
  "type": "string",
  "generator": {
    "type": "uuid"
  }
}
```

### email

The `email` generator generates a random e-mail address:

```json
{
  "type": "string",
  "generator": {
    "type": "email"
  }
}
```

### firstName

The `firstName` generator generates a random first name:

```json
{
  "type": "string",
  "generator": {
    "type": "firstName"
  }
}
```

### lastName

The `lastName` generator generates a random last name:

```json
{
  "type": "string",
  "generator": {
    "type": "email"
  }
}
```

### fullName

The `fullName` generator generates a random first and last name.
Please note that this generator does not output a name
related to [`firstName`](#firstname) or [`lastName`](#lastname).

```json
{
  "type": "string",
  "generator": {
    "type": "fullName"
  }
}
```

### username

The `username` generator generates a random (internet) username.

```json
{
  "type": "string",
  "generator": {
    "type": "username"
  }
}
```

### city

The `city` generator generates a random city name.

```json
{
  "type": "string",
  "generator": {
    "type": "city"
  }
}
```

### country

The `country` generator generates a random country name.

```json
{
  "type": "string",
  "generator": {
    "type": "country"
  }
}
```

### countryCode

The `countryCode` generator generates a random
[ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code.

```json
{
  "type": "string",
  "generator": {
    "type": "countryCode"
  }
}
```

### street

The `street` generator generates a random street name.

```json
{
  "type": "string",
  "generator": {
    "type": "street"
  }
}
```

### state

The `state` generator generates a random state name.

```json
{
  "type": "string",
  "generator": {
    "type": "state"
  }
}
```

### zipCode

The `zipCode` generator generates a random zip code.

```json
{
  "type": "string",
  "generator": {
    "type": "zipCode"
  }
}
```

### latitude

While it is *not technically* a string generator, as it outputs numbers, the
`latitude` generator is still listed as a string generator, since there is no other
place for it. If a string value is required, simply convert the latitude into a string
using [`format`](#format).

```json
{
  "type": "string",
  "generator": {
    "type": "country"
  }
}
```

Get the latitude as a string:

```json
{
  "type": "string",
  "generator": {
    "type": "format",
    "format": "{{latitude}}",
    "args": {
      "latitude": {
        "generator": {
          "type": "latitude"
        }
      }
    }
  }
}
```

### longitude

Equal to [`latitude`](#latitude), the `longitude` generator generates a random longitude.

```json
{
  "type": "string",
  "generator": {
    "type": "longitude"
  }
}
```

### phone

The `phone` generator generates a random phone number.

```json
{
  "type": "string",
  "generator": {
    "type": "phone"
  }
}
```

### format

The `format` generator formats strings using [handlebars](https://handlebarsjs.com/guide/)
templates. This generator requires the following inputs:

* `format`: The format to to use
* `args`: An key-value object containing the values to fill in. The value may be
a simple string a [`string` schema](string), a simple number or a [`reference`](reference).
* `serializeNonStrings`: Whether to serialize non-string values to strings if they are
already not string values. If set to false and a non-string is passed in through
a [reference](#references), an error will be returned. This does not
affect non-string values returned by string generators, like [`latitude`](#latitude), these
kinds of values will always be converted to strings when passed into `format`.

#### Example: Generate the full name of a person

```json
{
  "type": "object",
  "properties": {
    "firstName": {
      "type": "string",
      "generator": {
        "type": "firstname"
      }
    },
    "lastName": {
      "type": "string",
      "generator": {
        "type": "lastname"
      }
    },
    "fullName": {
      "type": "string",
      "generator": {
        "type": "format",
        "format": "{{firstName}} {{lastName}}",
        "args": {
          "firstName": "ref:./firstName",
          "lastName": "ref:./lastName"
        }
      }
    }
  }
}
```

The result will look something like this:

```json
{
  "firstName": "Wilford",
    "lastName": "Schulist",
    "fullName": "Wilford Schulist"
}
```

## References

Fixed string values may also be a [reference](reference). Reference strings start
with the value `"ref:"` and contain the normalized path to the field to reference.
If found, the value of the referenced field will be inserted where the reference string
was placed. Check the [reference](reference) page on further information on references.
The result is not limited to strings, it will have the type of the referenced field,
whatever that may be.

### Example 1: Copy string

Reference a field inside the [current object](reference#local-reference) using a
shorthand string expression, copying its contents into its current position:

```json
{
  "type": "object",
  "properties": {
    "id": {
      "type": "string",
      "generator": {
        "type": "uuid"
      }
    },
    "idCopy": "ref:./id"
  }
}
```

Will result in:

```json
{
  "id": "294993aa-adb6-4902-8f2b-38284ddd6779",
  "idCopy": "294993aa-adb6-4902-8f2b-38284ddd6779"
}
```

### Example 2: Copy an object

Copying any other value works just like copying strings.
Copy an object to the current position using a [global reference](reference#global-reference)
an not using a shorthand string value:

```json
{
  "type": "object",
  "properties": {
    "person": {
      "type": "object",
      "properties": {
      "id": {
        "type": "string",
        "generator": {
          "type": "uuid"
        }
      },
      "name": {
          "type": "string",
          "generator": {
            "type": "fullName"
          }
        }
      }
    },
    "personCopy": {
    "type": "string",
      "value": "ref:person"
    }
  }
}
```

Will produce a result similar to this:

```json
{
  "person": {
    "id": "3a106f78-74f5-4ecd-b79d-414488c03e9d",
    "name": "Adriel Wilderman"
  },
  "personCopy": {
    "id": "3a106f78-74f5-4ecd-b79d-414488c03e9d",
    "name": "Adriel Wilderman"
  }
}
```
