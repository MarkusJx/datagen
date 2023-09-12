---
title: Using plugins
---

In order to use a plugin, you need to define the path to the plugin somewhere
in the schema. This can be done in the `plugins` property of the schema,
a `transform` property, a `generate` property or the `serialize` property.

## Using a plugin with init arguments

If you want to use a plugin which requires arguments for initialization,
you need to define the arguments in the `options.plugins` property of the schema.

When defining a plugin in the `plugins` object, you can either pass
the path to the plugin as the key and the arguments as the value or
pass a name as the key and an object with the `path` and `args` properties
as the value. You can then use the imported plugin by referencing the
name provided in the `plugins` object.

```json
{
  "type": "string",
  "value": "hello",
  "options": {
    "plugins": {
      "my-plugin": {
        "path": "path/to/my/plugin",
        "args": {
          "my-arg": "my-value"
        }
      }
    }
  },
  "transform": [
    {
      "type": "plugin",
      "name": "my-plugin"
    }
  ]
}
```

If you don't want to define an alias for your plugin, you can also
use the path to the plugin file as the key. In this case, the plugin
must be accessed by the path to the plugin file.

```json
{
  "type": "object",
  "options": {
    "plugins": {
      "path/to/my/plugin": {
        "my-arg": "my-value"
      }
    }
  },
  "properties": {
    "pluginData": {
      "type": "plugin",
      "pluginName": "path/to/my/plugin",
      "args": {
        "my-arg": "my-value"
      }
    }
  }
}
```

## Using a plugin without init arguments

If you want to use a plugin which does not require arguments for initialization,
you can but don't have to define the plugin in the `options.plugins` property

If you choose not to, you can use the path to the plugin file as the name
for the plugin when using it.

```json
{
  "type": "string",
  "value": "hello",
  "transform": [
    {
      "type": "plugin",
      "name": "path/to/my/plugin"
    }
  ]
}
```
