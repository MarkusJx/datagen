# Using plugins

In order to use a plugin, you need to define the path to the plugin somewhere
in the schema. This can be done in the `plugins` property of the schema,
a `transform` property, a `generate` property or the `serialize` property.

## Plugin path resolution

Native plugins are shared libraries loaded using
[`libloading`](https://docs.rs/libloading/latest/libloading).
Check the documentation of the `libloading` crate for more information
on how the path to the plugin is resolved. For example, you can set
the `LD_LIBRARY_PATH` environment variable to the directory containing
the plugin library.

In addition to the system paths, you can also set the `DATAGEN_PLUGIN_DIR`
environment variable to a directory containing plugins. This directory
will be searched for plugins before the system paths. In addition to
this path, the directory where the `datagen` executable is located
is also searched for plugins. This means that you can place plugins
in the same directory as the `datagen` executable and they will be
found automatically.

Also, in contrast to the `libloading` crate, the plugin path does not require
the platform-specific file extension. The file extension is automatically
added based on the platform. For example on Linux, the `.so` extension
is added to the plugin path. This only applies if the plugin path does not
contain a file extension. Also, the `lib` prefix is automatically added
to the plugin path if it does not contain it already. This also works
for absolute paths.

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
