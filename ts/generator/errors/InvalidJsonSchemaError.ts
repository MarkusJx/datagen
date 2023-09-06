import { ErrorObject } from 'ajv/lib/types';

export default class InvalidJsonSchemaError extends Error {
    public constructor(public readonly errors: ErrorObject[] | null) {
        super('Invalid JSON schema');
    }
}
