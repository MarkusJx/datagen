import { getSchema } from 'datagen-rs-wasm';
import { compile, JSONSchema } from 'json-schema-to-typescript';
import { existsSync } from 'fs';
import { rm, writeFile } from 'fs/promises';
import * as path from 'path';

const outDir = path.join(__dirname, '..', 'src');
const typesFile = path.join(outDir, 'types.ts');
const schemaFile = path.join(outDir, 'schema.json');

(async () => {
  if (existsSync(typesFile)) {
    await rm(typesFile);
  }
  if (existsSync(schemaFile)) {
    await rm(schemaFile);
  }

  const schema = getSchema();
  await writeFile(schemaFile, schema);

  const compiled = await compile(JSON.parse(schema) as JSONSchema, 'Schema');
  await writeFile(typesFile, compiled);
})();
