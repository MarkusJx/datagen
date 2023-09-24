# Creating node.js plugins

Node.js plugins are installed as npm packages. To create a plugin, you need to
create a new npm package. This can be done with the `npm init` command.

```bash
npm init
```

This will create a new npm package. You can then add the `@datagen-rs/types` package as a
dependency, if desired, as this package provides typescript types for the plugin
API.

```bash
npm install --save-dev @datagen-rs/types
```

You can then create a new file, for example `index.ts`, and start writing your
plugin. A plugin must export a function with the following signature:

```ts
export type PluginInitFunction = (
  args: any
) => DatagenPlugin | Promise<DatagenPlugin>;
```

The `args` parameter is an object containing the arguments passed to the plugin
in the configuration file. The `DatagenPlugin` type is defined in the `@datagen-rs/types`
package and looks like this:

```ts
interface DatagenPlugin {
  generate?(schema: CurrentSchema, args: any): any | Promise<any>;
  transform?(schema: CurrentSchema, args: any, value: any): any | Promise<any>;
  serialize?(args: any, value: any): string | Promise<string>;
}
```

All methods are optional. The `generate` method is called when a new value is
generated. The `transform` method is called when a value is transformed. The
`serialize` method is called when a value is serialized to a string.

## Example

You may want to implement your plugin as follows:

```ts
import { DatagenPlugin, CurrentSchema } from '@datagen-rs/types';

class MyPlugin implements DatagenPlugin {
  constructor(args: any) {
    // Initialize the plugin
  }

  generate(schema: CurrentSchema, args: any): any {
    // Generate a new value
  }

  transform(schema: CurrentSchema, args: any, value: any): any {
    // Transform a value
  }

  serialize(args: any, value: any): string {
    // Serialize a value
  }
}

export default function pluginConstructor(args: any): DatagenPlugin {
  return new MyPlugin(args);
}
```
