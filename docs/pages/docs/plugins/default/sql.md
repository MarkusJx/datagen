# SQL-plugin

The `datagen-rs-sql-plugin` is a serializer plugin
for [`datagen-rs`](https://markusjx.github.io/datagen/) that allows you to upload data to
a SQL database.

The generated data must be an object containing at least one array of objects.

## Parameters

The plugin has the following parameters:

| Name             | Type                    | Description                                                                |
| ---------------- | ----------------------- | -------------------------------------------------------------------------- |
| `url`            | `String`                | The URL of the SQL database.                                               |
| `maxChunkSize`   | `u32`                   | The maximum number of rows to insert in a single query. Defaults to `100`. |
| `maxConnections` | `u32`                   | The maximum number of parallel connections to use. Defaults to `5`.        |
| `connectTimeout` | `u32`                   | The timeout for a connection in seconds. Defaults to `10`.                 |
| `mappings`       | [`Mappings`](#mappings) | The mappings for the data.                                                 |

### `Mappings`

A key-value pair that maps the data to the columns in the database.
The key is the name of the table and the value contains the name of the generated object
with the column mappings.

The value contains the following properties:

- `objectName`: The name of the generated object.
- `columnMappings`: A key-value pair that maps the data to the columns in the database.
  The key is the name of the column and the value is the name of the field in the
  generated object.

## Example

The following example shows how to use the `sql-plugin` for
a [PostgreSQL](https://www.postgresql.org/)
database with the following table:

```sql
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255),
  email VARCHAR(255)
);
```

The schema for `datagen` is defined as follows:

```json
{
  "options": {
    "serializer": {
      "type": "plugin",
      "pluginName": "sql-plugin",
      "args": {
        "url": "postgres://user:password@localhost:5432/db",
        "maxChunkSize": 100,
        "maxConnections": 5,
        "connectTimeout": 10,
        "mappings": {
          "users": {
            "objectName": "user",
            "columnMappings": {
              "id": "id",
              "name": "name",
              "email": "email"
            }
          }
        }
      }
    }
  },
  "type": "object",
  "properties": {
    "users": {
      "type": "array",
      "length": {
        "value": 100
      },
      "items": {
        "type": "object",
        "properties": {
          "id": {
            "type": "integer",
            "min": 1
          },
          "name": {
            "type": "string",
            "generator": {
              "type": "fullName"
            }
          },
          "email": {
            "type": "string",
            "generator": {
              "type": "email"
            }
          }
        }
      }
    }
  }
}
```
