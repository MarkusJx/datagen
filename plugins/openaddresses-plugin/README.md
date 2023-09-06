# Openaddresses

This plugin allows you to load data from [OpenAddresses](http://openaddresses.io/).

## Usage

This plugin requires configuration in your schema definition. You can select
between two different backends: `sqlite` and `memory`. The `sqlite` backend
loads and stores the data in a SQLite database. The `memory` backend loads the
data into memory and stores it in a `HashMap`. The `memory` backend is faster
for one-time loads, but the `sqlite` backend is faster if the tool is run multiple
times as the results are stored inside a database file.

In order to use the plugin, you need to pass an object with your items and
respective types from the address files as the argument. Currently, the following
types are supported: `strett`, `number`, `unit`, `city`, `district`, `region`,
`postcode`, `latitute`, `longitude`. Nested objects are also supported.

```json
{
  "options": {
    "plugins": {
      "openaddresses_plugin": {
        "files": "us_ak_haines.geojson",
        "backend": {
          "type": "sqlite",
          "databaseName": "addresses.db"
        }
      }
    }
  },
  "type": "generator",
  "pluginName": "openaddresses_plugin",
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