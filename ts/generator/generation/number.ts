import SchemaPath from './SchemaPath';
import Number, { ConstantNumber, RandomNumber } from '../../base/schema/number';
import InvalidSchemaError from '../errors/InvalidSchemaError';

export default function generateNumber(args: Number, path: SchemaPath): number {
    if ((args as ConstantNumber).value != undefined) {
        return (args as ConstantNumber).value;
    } else {
        let min: number | undefined;
        if ((args as any).min != undefined) {
            min = (args as any).min;
        }

        let max: number | undefined;
        if ((args as any).max != undefined) {
            max = (args as any).max;
        }

        let res: number;
        if (min != undefined && max != undefined) {
            if (min > max) {
                throw new InvalidSchemaError(
                    'number',
                    path,
                    `min (${min}) must be less than max (${max})`
                );
            } else {
                res = min + Math.random() * (max - min);
            }
        } else if (min != undefined) {
            res = min + Math.random() * (Number.MAX_SAFE_INTEGER - min);
        } else if (max != undefined) {
            res =
                Number.MIN_SAFE_INTEGER +
                Math.random() * (max - Number.MIN_SAFE_INTEGER);
        } else {
            res =
                Number.MIN_SAFE_INTEGER +
                Math.random() *
                    (Number.MAX_SAFE_INTEGER - Number.MIN_SAFE_INTEGER);
        }

        if ((args as RandomNumber).decimalPlaces != undefined) {
            const decimalPlaces = (args as RandomNumber).decimalPlaces!;
            const multiplier = Math.pow(10, decimalPlaces);
            return Math.round(res * multiplier) / multiplier;
        } else {
            return res;
        }
    }
}
