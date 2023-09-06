import Plugins from './plugins/Plugins';
import SchemaDefinition from '../base/schema/schemaDefinition';
import Schema from './generation/Schema';
import { validateJsonSchema } from './validation/jsonschema';
import InvalidJsonSchemaError from './errors/InvalidJsonSchemaError';

export async function generateData(schema: SchemaDefinition): Promise<any> {
    const validationResult = validateJsonSchema(schema);
    if (!validationResult.valid) {
        throw new InvalidJsonSchemaError(validationResult.errors ?? null);
    }

    const plugins = await Plugins.initializePlugins(schema);
    const schemaConverter = Schema.root(schema, plugins, schema.options ?? {});
    return schemaConverter.generateData();
}
