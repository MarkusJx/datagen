# Plugins

`datagen` can be extended by plugins. Plugins can be used to add new
generators, transformers and serializers.

Plugins are written in rust and can be compiled to a dynamic library
that can be loaded by `datagen`. For a plugin to be valid, it needs to
expose a function with the following signature:

```rust
pub unsafe extern "C" fn _plugin_create(
    args: *mut serde_json::Value,
) -> datagen_rs::plugins::plugin::PluginInitResult {
    // ...
}
```

Check out the [Creating plugins](plugins/createPlugin.mdx) page for more
information on how to create a plugin.

Check out the [Using plugins](plugins/usePlugin.md) page for more information
on how to use plugins.
