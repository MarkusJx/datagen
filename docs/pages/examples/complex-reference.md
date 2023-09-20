# Complex reference

This example schema creates an array of 100 to 10000 people with a random number of related persons.
The [`reference`](../docs/generators/reference.md) generator is used to create a relationship between two people.

An exception is added to that [`reference`](../docs/generators/reference.md) generator in order to prevent a
person from being related to themselves or to the same person twice.

```json
{
    "type": "array",
    "length": {
        "min": 100,
        "max": 10000
    },
    "items": {
        "type": "object",
        "properties": {
            "id": {
                "type": "string",
                "generator": {
                    "type": "uuid"
                }
            },
            "firstName": {
                "type": "string",
                "generator": {
                    "type": "firstName"
                }
            },
            "lastName": {
                "type": "string",
                "generator": {
                    "type": "lastName"
                }
            },
            "relatedPersons": {
                "type": "array",
                "length": {
                    "min": 0,
                    "max": 3
                },
                "items": {
                    "type": "object",
                    "properties": {
                        "relatedPerson": {
                            "type": "reference",
                            "reference": "id",
                            "except": [
                                "ref:../../relatedPersons.relatedPerson",
                                "ref:../../id"
                            ]
                        },
                        "relationshipType": {
                            "type": "anyOf",
                            "values": [
                                "parent",
                                "child",
                                "sibling",
                                "spouse",
                                "friend"
                            ]
                        }
                    }
                }
            }
        }
    }
}
```

## Explanation

The `relatedPerson` reference is used in this example to create a reference
to an id of an object inside the outer array (the normalized path to the id is simply `id`).
For this, a global reference is used, which means that the reference will search for the
referenced field in the root of the schema and return all matches for the reference.
Since `keepAll` is not defined (thus set to `false`), a random match will be returned.

The `except` array contains two references. The first one is a local reference to the
`relatedPersons.relatedPerson` field. The object tree must be traversed up twice,
as the full (internal) path to the `relatedPerson` is `root.[index].relatedPersons.[index].relatedPerson`.
Traversing up the object tree twice will get you to the object at `root.[index]`.
Adding `relatedPersons.relatedPerson` will now traverse back up the object, ignoring
all indexes, thus returning all values of all `relatedPerson` fields in the `relatedPersons`
array. Explaining this using glob patterns, the full (global) path would look something
like this: `root.[index].relatedPersons.*.relatedPerson` (`[index]` would be replaced by an actual index in this example).
