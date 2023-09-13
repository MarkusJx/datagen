import { Schema } from './types';
import {
    generateRandomDataInternalAsync,
    getSchema,
    getSchemaAsync,
} from '../index';
import { findPlugins, Plugin } from './plugin';
export { CurrentSchema } from '../index';

/**
 * Generates random data from a schema
 * and returns the serialized result.
 *
 * @param schema the schema to generate data from
 * @param extraPlugins additional plugins to use
 */
export async function generateRandomData(
    schema: Schema,
    extraPlugins: Record<string, Plugin> = {}
): Promise<string> {
    return generateRandomDataInternalAsync(
        schema,
        await findPlugins(schema, extraPlugins)
    );
}

export function getJSONSchema(): Schema {
    return getSchema();
}

export function getJSONSchemaAsync(): Promise<Schema> {
    return getSchemaAsync();
}
