# openaddresses-plugin

The `openaddresses-plugin` plugin can be used to generate real addresses from
the [OpenAddresses project](https://openaddresses.io/). The plugin uses
`geojson` files from the project to generate addresses.

## Usage

Before using the plugin, you need to initialize the plugin in the
[`plugin`](https://markusjx.github.io/datagen/docs/options/#plugin) options.
The plugin takes the following options:

- `files`: A single string or an array of strings containing the
  paths to the files to use. The files should be in the `geojson` format.
- `backend`: The plugin has two backends: `memory` and `sqlite`. The
  `memory` backend loads the entire file into memory, while the `sqlite`
  backend loads the file into a sqlite database. The `sqlite` backend is
  slower than the `memory` backend but uses less memory. The default
  backend is `memory`. If the `sqlite` backend is used, the plugin requires
  a path to the sqlite database file. The database file will be created if
  it does not exist.

### Initialize the `memory` backend

```json
{
  "options": {
    "plugins": {
      "openaddresses-plugin": {
        "path": "/path/to/openaddresses_plugin",
        "args": {
          "files": ["/path/to/geojson/file1", "/path/to/geojson/file2"],
          "backend": {
            "type": "memory"
          }
        }
      }
    }
  }
}
```

### Initialize the `sqlite` backend

This will initialize the `sqlite` backend and create a sqlite database
at `/path/to/sqlite/database.db`. The database will be populated with
the data from the `geojson` files, which may take some time. If the
database already exists, it will be used instead of creating a new one.

```json
{
  "options": {
    "plugins": {
      "openaddresses-plugin": {
        "path": "/path/to/openaddresses_plugin",
        "args": {
          "files": ["/path/to/geojson/file1", "/path/to/geojson/file2"],
          "backend": {
            "type": "sqlite",
            "databaseName": "/path/to/sqlite/database.db",
            "batchSize": 1000,
            "cacheSize": 10000
          }
        }
      }
    }
  }
}
```

#### Arguments

- `databaseName`: The path to the sqlite database file. The file will be
  created if it does not exist.
- `batchSize`: The number of rows to insert into the database at once.
  The default is set during the build.
- `cacheSize`: The number of rows to cache in memory. The default is `10000`.

### Generate addresses

In order to use the plugin, provide the `openaddresses-plugin` plugin
name in the [`plugin`](https://markusjx.github.io/datagen/docs/generators/plugin/)
generator.

The plugin accepts an object containing the names of the properties to
generate as keys and the fields to use as values. The following field
types are supported:

- `number`: The number of addresses to generate.
- `street`: The street name.
- `city`: The city name.
- `unit`: The unit number.
- `district`: The district name.
- `region`: The region name.
- `postcode`: The postcode.
- `latitude`: The latitude.
- `longitude`: The longitude.

The objects may also be nested, to create nested objects.

#### Example

```json
{
  "type": "plugin",
  "pluginName": "openaddresses-plugin",
  "args": {
    "street": "street",
    "houseNumber": "number",
    "city": "city",
    "coordinates": {
      "latitude": "latitude",
      "longitude": "longitude"
    }
  }
}
```

This will produce an object like this:

```json
{
  "street": "Muncaster Rd",
  "houseNumber": "831",
  "city": "HAINES",
  "coordinates": {
    "latitude": 59.2442386,
    "longitude": -135.4394579
  }
}
```
