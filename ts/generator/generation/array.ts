import Array from '../../base/schema/array';
import SchemaPath from './SchemaPath';
import generateNumber from './number';
import Schema from './Schema';
import InvalidSchemaError from '../errors/InvalidSchemaError';

export default async function generateArray(
    args: Array,
    path: SchemaPath,
    schema: Schema
): Promise<any[]> {
    let numElements: number;
    if (typeof args.length === 'number') {
        numElements = args.length;
    } else {
        numElements = Math.round(
            generateNumber(
                { type: 'number', ...args.length },
                path.append('length')
            )
        );
    }

    if (numElements < 0) {
        throw new InvalidSchemaError(
            'array',
            path,
            `Length must be greater than or equal to 0: ${numElements}`
        );
    }

    const result: any[] = [];
    let currentSchema: Schema | null = null;
    for (let i = 0; i < numElements; i++) {
        currentSchema = new Schema(
            args.items,
            path.append(i.toString()),
            schema,
            result,
            currentSchema
        );
        result.push(await currentSchema.generateData());
    }

    return result;
}
