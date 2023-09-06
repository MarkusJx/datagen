import Boolean, {
    ConstantBoolean,
    RandomBoolean,
} from '../../base/schema/boolean';
import InvalidSchemaError from '../errors/InvalidSchemaError';
import SchemaPath from './SchemaPath';

export default function generateBoolean(args: Boolean, path: SchemaPath) {
    if ((args as RandomBoolean).chance != undefined) {
        return Math.random() < (args as RandomBoolean).chance;
    } else if ((args as ConstantBoolean).value != undefined) {
        return (args as ConstantBoolean).value;
    } else {
        throw new InvalidSchemaError(
            'boolean',
            path,
            "Expected either 'chance' or 'value' to be defined"
        );
    }
}
