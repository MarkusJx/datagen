import { getSchemaAsync } from '../../index';
import { compile, JSONSchema } from 'json-schema-to-typescript';
import { writeFile } from 'fs/promises';
import * as path from 'path';

(async () => {
    const schema = await getSchemaAsync();
    const compiled = await compile(schema as JSONSchema, 'Schema');
    await writeFile(path.join(__dirname, '..', 'types.ts'), compiled);
})();
