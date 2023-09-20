# reference

A reference is a way to copy the contents of one field into another field.
The copied contents may be of any type, including objects and arrays, but
this can be limited by the type of the field that is being copied into.

For example, the [`format` string generator](string.md#format) only accepts
strings unless the `serializeNonStrings` option is set to `true`.

## Creating a reference

### Using a [`string`](string.md#references) value

A reference may be created using the [`string`](string.md) generator
and setting the value to a string that starts with `"ref:"` and contains
the normalized path to the field to reference.

#### Example

Generate a random UUID and copy it into another field:

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

### Using the `reference` generator

A reference may also be created using the `reference` generator.
This generator has three inputs:

-   `reference`: The normalized path to the field to reference. This path may
    or may not start with the `"ref:"` prefix.
-   `except`: References or values to exclude from the reference. This is useful
    when the reference is inside the field being referenced. In order to exclude a
    reference, the reference must be a string that starts with `"ref:"` and contains
    the normalized path to the field to exclude.
-   `keepAll`: If set to `true`, the reference will keep all the fields found
    by the reference and return an array if there is more than one. If set to `false`,
    a random field will be returned from the reference.

#### Example

Generate a random UUID and copy it into another field:

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
        "idCopy": {
            "type": "reference",
            "reference": "./id"
        }
    }
}
```

For a more complex example, check out the [complex reference example](../../examples/complex-reference.md).

## The reference path

The reference path is a string that contains the path to the field to reference.
This path may be absolute or relative. It does not contain the indices of any
array fields, instead it only consists of the names of fields inside objects.

For example if you have the following schema:

```json
{
    "type": "object",
    "properties": {
        "field1": {
            "type": "object",
            "properties": {
                "field2": {
                    "type": "string",
                    "value": "test"
                }
            }
        }
    }
}
```

The path to the `field2` field is `"field1.field2"`.
If you want to reference the `field2` field from the root object, you can use
the absolute path `"field1.field2"`. If you want to reference the `field2` field
from inside the `field1` object, you can use the relative path `"./field2"`.

Referencing a specific field inside an array is not possible. Instead, when
referencing a field inside an array, the reference will return the value of
the field for each item in the array.

For example, if you have the following schema:

```json
{
    "type": "object",
    "properties": {
        "field1": {
            "type": "array",
            "length": {
                "value": 10
            },
            "items": {
                "type": "object",
                "properties": {
                    "field2": "test"
                }
            }
        }
    }
}
```

The path to the `field2` field is still `"field1.field2"` since the reference
path does not contain the indices of the array. When referencing the `field2`
field, the reference will return the value of the `field2` field for each item
in the `field1` array. In this case, 10 values will be returned.

**Note:** The full (internal) path to the `field2` field is `"root.field1.0.field2"`.
Thus, when referencing an outside field from inside an array, you need to traverse
up two objects to get to the root object.

## Reference types

References can be classified into two types: local and global.

### Local reference

A local reference is a reference that starts with `"./"` or `"../"` and contains the
normalized path to the field to reference. This path is relative to the
current field.

When referencing a field inside the current object it is important to place the
referenced object **before** the reference. Otherwise, the reference will not
be able to find the field as the field will not be generated yet.

#### Traversing up the object tree

The `"../"` prefix is used to traverse up the object tree. For example, if you
want to reference a field inside the parent object, you can use `"../fieldName"`.
If you want to reference a field inside the grandparent object, you can use
`"../../fieldName"`.

**Note:** When traversing up the object tree with an array inside, the field's index
will count as an object. For example, if you have this schema:

```json
{
    "type": "object",
    "properties": {
        "field1": "test",
        "field2": {
            "type": "array",
            "length": {
                "value": 10
            },
            "items": {
                "type": "object",
                "properties": {
                    "field3": "ref:../../field1",
                    "field4": "test"
                }
            }
        }
    }
}
```

The reference `"ref:../../field1"` will reference the `field1` field inside the
parent object, not the grandparent object. This is because the item's index as
an object. The full path to the `field3` field would be `root.field2.0.field3`
and the full path to the `field1` field would be `root.field1`. Thus, you need to
traverse up two objects to get to the parent object.

But this does not affect references in the other direction, since forward references
use the normalized path to the field, not the full path. In this example, if you wanted
to reference all `field3` fields using the `field4` field, you'd need to use
`ref:../../field2.field3`, since the forward path never contains the index of the field.

Although this may seem confusing, it is actually quite simple. Just remember that
the index of an array counts as an object when traversing up the object tree.
It also has a few advantages, for example, if you want to reference a field inside an
object inside an array from inside that object without referencing all other fields
inside the array, you can traverse up the object tree and still keep the index of
the current object. The [complex reference example](../../examples/complex-reference.md)
shows an example of this.

### Global reference

In order to get either all the fields with an equal path, for example if the objects are
located inside an array at some point, you can use a global reference. A global reference
is a reference that has no prefix and contains the normalized path to the field to reference.

Global references can be used from every point in the schema and will produce the same
results, assuming all fields being referenced have been created, which is hardly ever the case.

#### Example

Get all the `field1` fields from the root object:

```json
{
    "type": "object",
    "properties": {
        "field1": "test",
        "field2": {
            "type": "array",
            "length": {
                "value": 10
            },
            "items": {
                "type": "object",
                "properties": {
                    "field1": "field1",
                    "field2": "field1"
                }
            }
        }
    }
}
```

Both `field2.field1` and `field2.field2` as well as `field1` will be set to `"test"`.

## Excluding references

When referencing a field inside the current object, it is possible to exclude
a field being referenced. This is useful when the reference is inside the
field being referenced.

Check out the [complex reference example](../../examples/complex-reference.md) for
an example of excluding references.
