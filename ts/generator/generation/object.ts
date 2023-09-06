import ObjectSchema from '../../base/schema/object';
import SchemaPath from './SchemaPath';
import Schema from './Schema';

export default async function generateObject(
    args: ObjectSchema,
    path: SchemaPath,
    schema: Schema
): Promise<Record<string, any>> {
    const res = {} as Record<string, any>;
    let currentSchema: Schema | null = null;
    for (const key of Object.keys(args.properties)) {
        currentSchema = new Schema(
            args.properties[key],
            path.append(key),
            schema,
            res,
            currentSchema
        );
        res[key] = await currentSchema.generateData();
    }

    return res;
}
