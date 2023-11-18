# datagen-rs-node-runner

This package provides a Node.js runner for the
[`datagen-rs` cli](https://markusjx.github.io/datagen/docs/packages/rustCli/).
Using this package, you can use [Node.js plugins](https://markusjx.github.io/datagen/docs/plugins/node/)
within the `datagen-rs` cli.

Check the [documentation on how to use node plugins](https://markusjx.github.io/datagen/docs/plugins/node/use/)
for further information on how to use Node.js plugins.

## Programmatic Usage

### Installation

To install this package as a dependency, you can use the following command:

```bash
cargo install datagen-rs-node-runner
```

### Usage

To use this package programmatically, you can use the following code:

```rust
use datagen_rs::util::helpers::generate_random_data;
use datagen_rs_node_runner::runner::node_runner::NodeRunner;
use serde_json::{from_str};

fn main() {
  let schema = from_str(r#"
  {
    "options": {
      "plugins": {
        "my-plugin": {
          "path": "node:my-plugin",
          "args": {
            "arg1": "value1",
            "arg2": "value2"
          }
        }
      }
    },
    "type": "plugin",
    "pluginName": "my-plugin",
    "args": {
      "arg1": "value1",
      "arg2": "value2"
    }
  }
  "#).unwrap();

  let (_runner, plugins) = NodeRunner::init(&schema).unwrap();
  let generated = generate_random_data(schema, Some(plugins)).unwrap();

  println!("{}", generated);
}
```

### Load plugins after initialization

As the `NodeRunner::init` function can only be called once per process, you can use the
`NodeRunner::load_new_plugins` function to load plugins after initialization.

```rust
fn main() {
  let schema = from_str(r#"
  {
    "options": {
      "plugins": {
        "my-plugin": {
          "path": "node:my-plugin",
          "args": {
            "arg1": "value1",
            "arg2": "value2"
          }
        }
      }
    },
    "type": "plugin",
    "pluginName": "my-plugin",
    "args": {
      "arg1": "value1",
      "arg2": "value2"
    }
  }
  "#).unwrap();

  let (runner, plugins) = NodeRunner::init(&schema).unwrap();
  let generated = generate_random_data(schema, Some(plugins)).unwrap();

  println!("{}", generated);

  let new_plugins = runner.load_new_plugins(&schema).unwrap();
  let generated = generate_random_data(schema, Some(new_plugins)).unwrap();

  println!("{}", generated);
}
```
