---
title: Create a new plugin
---

A `datagen` Plugin is a dynamic library that can be loaded by `datagen` to
extend its functionality. The plugin itself is a struct, which implements
the `Plugin` trait. This trait defines the functions that are called by
`datagen` to initialize the plugin and to get the generators, transformers
and serializers that are provided by the plugin.

In addition to the `Plugin` trait, the plugin struct may also
implement the `PluginConstructor` trait. This trait defines a function
that is called by `datagen` to create a new instance of the plugin `struct`.

In order to expose the plugin to `datagen`, the plugin needs to be compiled
to a dynamic library, exposing a `_plugin_create` and `_plugin_version`
function. The easiest way of adding these functions is by using the
`declare_plugin!` macro.

## Creating a new plugin

First, you need to define a struct, which implements the `Plugin` trait.
The `Plugin` trait has one function that needs to be implemented: `name`,
which returns the name of the function. The `transform`, `generate` and
`serialize` functions are optional and can be implemented if the plugin
provides transformers, generators or serializers. The `Plugin` trait also
requires the `Debug` trait to be implemented.

```rust
use datagen_rs::plugins::plugin::Plugin;
use datagen_rs::generate::current_schema::CurrentSchemaRef;
use datagen_rs::generate::generated_schema::GeneratedSchema;
use datagen_rs::util::types::Result;
use serde_json::Value;

#[derive(Debug, Default)]
struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &'static str {
        "my-plugin"
    }

    // Optional: Implement the `transform` function
    // if the plugin provides a transformer.
    fn transform(
        &self,
        schema: CurrentSchemaRef,
        value: Arc<GeneratedSchema>,
        args: Value,
    ) -> Result<Arc<GeneratedSchema>> {
        // ...
    }
    
    // Optional: Implement the `generate` function
    // if the plugin provides a generator.
    fn generate(&self, schema: CurrentSchemaRef, args: Value) -> Result<Arc<GeneratedSchema>> {
        // ...
    }
    
    // Optional: Implement the `serialize` function
    // if the plugin provides a serializer.
    fn serialize(&self, value: &CurrentSchemaRef, args: Value) -> Result<String> {
        // ...
    }
}
```

If your plugin requires arguments for initialization, you can implement
the `PluginConstructor` trait. This trait defines a function that is
called by `datagen` to create a new instance of the plugin struct.

```rust
use datagen_rs::plugins::plugin::PluginConstructor;

impl PluginConstructor for MyPlugin {
    fn new(args: Value) -> Result<Self> {
        // ...
    }
}
```

## Exposing the plugin

In order to expose the plugin to `datagen`, the plugin needs to be compiled
to a dynamic library, exposing a `_plugin_create` and `_plugin_version`
function. The easiest way of adding these functions is by using the
`declare_plugin!` macro.

```rust
use datagen_rs::plugins::plugin::declare_plugin;

declare_plugin!(MyPlugin);
```

If your plugin doesn't implement `PluginConstructor`, you need to add the
constructor call to the `declare_plugin!` macro.

```rust
use datagen_rs::plugins::plugin::declare_plugin;

declare_plugin!(MyPlugin, MyPlugin::default);
```

## Compiling the plugin

In order to compile the plugin, you need to add the following to your
`Cargo.toml`:

```toml
[package]
name = "my-plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["dylib"]

[dependencies]
datagen-rs = "0.1.0"
```