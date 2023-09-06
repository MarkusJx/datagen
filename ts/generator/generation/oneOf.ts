import OneOf from '../../base/schema/oneOf';
import SchemaPath from './SchemaPath';
import Schema from './Schema';
import { randomArrayElement } from './util';

export default function generateOneOf(
    args: OneOf,
    path: SchemaPath,
    schema: Schema
): Promise<any> {
    const newSchema = new Schema(
        randomArrayElement(args.elements),
        path,
        schema
    );
    return newSchema.generateData();
}
