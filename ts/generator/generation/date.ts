import GeneratedDate, { ConstantDate } from '../../base/schema/date';
import SchemaPath from './SchemaPath';
import InvalidSchemaError from '../errors/InvalidSchemaError';

function tryParseDate(date: string, path: SchemaPath): Date {
    const res = Date.parse(date);

    if (isNaN(res)) {
        throw new InvalidSchemaError('date', path, `Invalid date: ${date}`);
    } else {
        return new Date(res);
    }
}

export default function generateDate(
    args: GeneratedDate,
    path: SchemaPath
): Date {
    if ((args as ConstantDate).value != undefined) {
        return tryParseDate((args as ConstantDate).value, path);
    } else {
        let min: Date | undefined;
        if ((args as any).min != undefined) {
            min = tryParseDate((args as any).min, path);
        }

        let max: Date | undefined;
        if ((args as any).max != undefined) {
            max = tryParseDate((args as any).max, path);
        }

        if (min != undefined && max != undefined) {
            if (min > max) {
                throw new InvalidSchemaError(
                    'date',
                    path,
                    `min (${min}) must be less than max (${max})`
                );
            } else {
                return new Date(
                    min.getTime() +
                        Math.random() * (max.getTime() - min.getTime())
                );
            }
        } else if (min != undefined) {
            return new Date(
                min.getTime() + Math.random() * (Date.now() - min.getTime())
            );
        } else if (max != undefined) {
            return new Date(
                Date.now() - Math.random() * (Date.now() - max.getTime())
            );
        } else {
            return new Date(Date.now() * Math.random());
        }
    }
}
