# plugin

The `plugin` generator is a special generator which uses a [plugin](../plugins) to generate data.
This generator is used to call custom generators and takes the following properties:

-   `pluginName`: The path of the plugin to use to generate the data. This name must
    match the path of the plugin in the `options.plugins` property at the root of the
    schema.
-   `args`: The arguments to pass to the plugin. This can be any valid JSON value.

The plugin must be a valid `datagen` plugin and must implement the `Plugin` trait
and export the `_plugin_create` and `_plugin_version` methods.

**Note:** Some plugins may require additional configuration during creation.
These configuration options are passed to the plugin using the
`options.plugins.[pluginPath].args` property at the root of the schema.

## Example

Use the [`openaddresses` plugin](../plugins/default/openaddresses.md) to generate real addresses:

```json
{
    "options": {
        "plugins": {
            "openaddresses_plugin": {
                "files": "./path/to/a/address.geojson",
                "backend": "memory"
            }
        }
    },
    "type": "array",
    "length": {
        "value": 10
    },
    "items": {
        "type": "plugin",
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
}
```

Note how the plugin's configuration is stored inside the `options.plugins` property
at the top of the document.

The result may look like this:

```json
[
    {
        "street": "Falconer Ave",
        "houseNumber": "118",
        "city": "HAINES",
        "coordinates": {
            "latitude": 59.3996734,
            "longitude": -135.8990132
        }
    }
]
```
