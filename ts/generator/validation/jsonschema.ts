import Ajv from 'ajv';
import schema from '../../base/schema.json';
import { ErrorObject } from 'ajv/lib/types';

export type ValidationResult = {
    valid: boolean;
    errors?: ErrorObject[] | null;
};

export function validateJsonSchema(data: any): ValidationResult {
    const ajv = new Ajv({
        allowUnionTypes: true,
    });

    const validate = ajv.compile(schema);
    const valid = validate(data);

    return {
        valid,
        errors: validate.errors,
    };
}
