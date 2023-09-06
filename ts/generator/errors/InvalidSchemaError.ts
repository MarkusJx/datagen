import SchemaPath from '../generation/SchemaPath';

export default class InvalidSchemaError extends Error {
    constructor(name: string, path: SchemaPath, message: string) {
        super(`Invalid ${name} schema at ${path.toString()}: ${message}`);
        this.name = 'InvalidArgs';
    }
}
