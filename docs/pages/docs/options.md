# Options

`datagen` can be configured using the `options` object inside the schema.
The following options are available:

| Option                | Description                                                                                                   | Default   |
| --------------------- | ------------------------------------------------------------------------------------------------------------- | --------- |
| `plugins`             | The [plugins](plugins.md) to use.                                                                             | `[]`      |
| `maxRefCacheSize`     | The maximum number of references to cache.<br/>Lowering this value will increase the performance of the tool. | unlimited |
| `serializeNonStrings` | Whether to serialize non-string values. Can be overridden by property values.                                 | `false`   |
| `serializer`          | The [serializer](#serializer) to use.                                                                         | `json`    |

## Serializer

The serializer is responsible for serializing the generated data to a string.
The following serializers are available:

- `json` (default)
- `yaml`
- `xml`
- `plugin` (see [plugins](plugins.md))

### Serializer Options

#### JSON

| Option   | Description                       | Default |
| -------- | --------------------------------- | ------- |
| `pretty` | Whether to pretty print the JSON. | `false` |

#### YAML

This serializer has no options.

#### XML

| Option        | Description                   | Default                        |
| ------------- | ----------------------------- | ------------------------------ |
| `rootElement` | The name of the root element. | unset, must be set by the user |

#### Plugin

| Option       | Description                        | Default                        |
| ------------ | ---------------------------------- | ------------------------------ |
| `pluginName` | The name of the plugin to use.     | unset, must be set by the user |
| `args`       | The options to pass to the plugin. | `null`                         |

## Example

```json
{
  "options": {
    "plugins": [
      {
        "name": "my-plugin",
        "args": {
          "foo": "bar"
        }
      }
    ],
    "maxRefCacheSize": 100,
    "serializeNonStrings": true,
    "serializer": {
      "type": "json",
      "pretty": true
    }
  },
  "type": "string",
  "value": "Hello World!"
}
```
