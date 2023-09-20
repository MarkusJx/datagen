# plugin

The `plugin` transformer allows you to use a [plugin](../plugins.md) providing
a custom transformer in your schema.

This transformer has the following properties:

- `name`: The name of the plugin. This is the path to the plugin file.
- `args`: The arguments to pass to the plugin. This can be empty or any JSON value.

## Example

```json
{
  "type": "string",
  "value": "hello",
  "transform": [
    {
      "type": "plugin",
      "name": "my-plugin",
      "args": {
        "foo": "bar"
      }
    }
  ]
}
```
