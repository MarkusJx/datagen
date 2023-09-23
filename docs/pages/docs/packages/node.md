# @datagen/node

This package contains node.js bindings to `datagen` for generating random data.
The bindings are built using [napi-rs](https://napi.rs).

## Installation

```bash
npm install @datagen/node
```

If you want to use the bindings in a TypeScript project, you may also need to install the
[`@datagen/types`](https://markusjx.github.io/datagen/docs/packages/nodeTypes/) package:

```bash
npm install --save-dev @datagen/types
```

## Usage

### Generate random data

In order to generate random data, you must pass a JSON schema to the `generateRandomData`
function:

```ts
import { generateRandomData } from '@datagen/node';

const generated = await generateRandomData({
  type: 'string',
  generator: {
    type: 'uuid',
  },
});
```

The result will be the serialized generated data as a string. In order to customize the
serialization, you can pass a serializer into the
[`options` object](https://markusjx.github.io/datagen/docs/options/#serializer).

### Generate random data with progress

If you want to get progress updates while generating data, you can
pass a progress callback to the `generateRandomData` function:

```ts
import { generateRandomData } from '@datagen/node';

const generated = await generateRandomData(
  {
    type: 'string',
    generator: {
      type: 'uuid',
    },
  },
  ({ current, total }) => {
    console.log(`Generated ${current}/${total} items`);
  }
);
```

The progress value is of type `GenerateProgress` and has the following structure:

```ts
interface GenerateProgress {
  current: number;
  total: number;
}
```

Check the [`progress-plugin` documentation](https://markusjx.github.io/datagen/docs/plugins/default/progress/)
for further information on how the progress is calculated.

### Generate random data using a plugin

You can pass additional plugins to the `generateRandomData` function. These plugins
will be loaded before the generation starts and can be used to extend the functionality
of `datagen`. Check out the
[plugin documentation](https://markusjx.github.io/datagen/docs/plugins/node/create/)
for more information on creating node.js plugins.

```ts
import { generateRandomData, CurrentSchema } from '@datagen/node';

const generated = await generateRandomData(
  {
    type: 'plugin',
    pluginName: 'myPlugin',
    args: {
      name: 'test',
    },
  },
  null,
  {
    myPlugin: {
      generate(schema: CurrentSchema, args: any): any {
        return 'Hello World!';
      },
    },
  }
);
```

Note that this package also exports a `CurrentSchema` type that can be used to type the
`schema` parameter of the `generate` function. This is simply the implementation
of the `CurrentSchema` interface from the `@datagen/types` package.

### Retrieve the JSON schema

In order to retrieve the JSON schema, you can use the `getJsonSchema` or
`getJsonSchemaAsync` functions:

```ts
import { getJsonSchema, getJsonSchemaAsync } from '@datagen/node';

const schema = getJsonSchema();
const schemaAsync = await getJsonSchemaAsync();
```
