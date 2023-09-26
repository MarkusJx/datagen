import {
  GenerateProgress,
  generateRandomDataInternal,
  getSchema,
  getSchemaAsync,
} from '../native';
import { findPlugins } from './plugin';
import { DatagenPlugin, Schema } from 'datagen-rs-types';
export { CurrentSchema, GenerateProgress } from '../native';

/**
 * Generates random data from a schema
 * and returns the serialized result.
 *
 * @param schema the schema to generate data from
 * @param callback a callback to receive progress updates
 * @param extraPlugins additional plugins to use
 */
export async function generateRandomData(
  schema: Schema,
  callback?: ((progress: GenerateProgress) => void) | null,
  extraPlugins: Record<string, DatagenPlugin> = {}
): Promise<string> {
  if (callback && typeof callback !== 'function') {
    throw new Error('callback must be a function');
  }

  return generateRandomDataInternal(
    schema,
    callback,
    await findPlugins(schema, extraPlugins)
  );
}

export function getJsonSchema(): Schema {
  return getSchema();
}

export function getJsonSchemaAsync(): Promise<Schema> {
  return getSchemaAsync();
}
