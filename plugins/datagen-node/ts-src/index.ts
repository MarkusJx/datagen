import {
    GenerateProgress,
    generateRandomDataInternalAsync,
    generateRandomDataWithProgressInternal,
    getSchema,
    getSchemaAsync,
} from '../native';
import { findPlugins } from './plugin';
import { DatagenPlugin, Schema } from '@datagen/types';
export { CurrentSchema, GenerateProgress } from '../native';

/**
 * Generates random data from a schema
 * and returns the serialized result.
 *
 * @param schema the schema to generate data from
 * @param extraPlugins additional plugins to use
 */
export async function generateRandomData(
    schema: Schema,
    extraPlugins: Record<string, DatagenPlugin> = {}
): Promise<string> {
    return generateRandomDataInternalAsync(
        schema,
        await findPlugins(schema, extraPlugins)
    );
}

export async function generateRandomDataWithProgress(
    schema: Schema,
    callback: (progress: GenerateProgress) => void,
    extraPlugins: Record<string, DatagenPlugin> = {}
): Promise<string> {
    return generateRandomDataWithProgressInternal(
        schema,
        callback,
        await findPlugins(schema, extraPlugins)
    );
}

export function getJSONSchema(): Schema {
    return getSchema();
}

export function getJSONSchemaAsync(): Promise<Schema> {
    return getSchemaAsync();
}
