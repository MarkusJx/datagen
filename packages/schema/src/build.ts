import { getSchema } from '../../../datagen-wasm/pkg';
import { compile, JSONSchema } from 'json-schema-to-typescript';
import { existsSync } from 'fs';
import { mkdir, rm, writeFile } from 'fs/promises';
import * as path from 'path';

const outDir = path.join(__dirname, '..', 'dist');
const outFile = path.join(outDir, 'types.ts');

(async () => {
    if (existsSync(outDir)) {
        await rm(outDir, { recursive: true });
    }

    await mkdir(outDir);

    const schema = getSchema();//console.log(schema);
    await writeFile(path.join(outDir, 'schema.json'), schema);
    const compiled = await compile(JSON.parse(schema) as JSONSchema, 'Schema');

    await writeFile(outFile, compiled);
})();
