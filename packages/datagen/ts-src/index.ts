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
 * @param generateCallback a callback to receive generate progress updates
 * @param serializeCallback a callback to receive serialize progress updates
 * @param extraPlugins additional plugins to use
 */
export async function generateRandomData(
  schema: Schema,
  generateCallback?: ((progress: GenerateProgress) => void) | null,
  serializeCallback?: ((progress: GenerateProgress) => void) | null,
  extraPlugins: Record<string, DatagenPlugin> = {}
): Promise<string> {
  if (generateCallback && typeof generateCallback !== 'function') {
    throw new Error('generate callback must be a function');
  } else if (serializeCallback && typeof serializeCallback !== 'function') {
    throw new Error('serialize callback must be a function');
  }

  return generateRandomDataInternal(
    schema,
    generateCallback,
    serializeCallback,
    await findPlugins(schema, extraPlugins)
  );
}

export function getJsonSchema(): Schema {
  return getSchema();
}

export function getJsonSchemaAsync(): Promise<Schema> {
  return getSchemaAsync();
}
