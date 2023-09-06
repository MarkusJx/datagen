import Schema from './Schema';
import SchemaPath from './SchemaPath';
import InvalidSchemaError from '../errors/InvalidSchemaError';

export function randomArrayElement<T>(arr: T[]): T {
    return arr[Math.floor(Math.random() * arr.length)];
}

export function chanceHit(chance: number, path: SchemaPath): boolean {
    if (chance < 0 || chance > 1) {
        throw new InvalidSchemaError(
            'chance',
            path,
            'Chance must be between 0 and 1'
        );
    }

    return Math.random() < chance;
}

export function resolveRef<T extends string | number | undefined>(
    str: T,
    schema: Schema,
    resolveAll?: false
): T;
export function resolveRef<T extends string | number | undefined>(
    str: T,
    schema: Schema,
    resolveAll: true
): T | T[];
export function resolveRef<T extends string | number | undefined>(
    str: T,
    schema: Schema,
    resolveAll = false
): T | T[] {
    if (str == undefined) {
        return undefined as T;
    }

    if (typeof str === 'string' && str.startsWith('ref:')) {
        let ref = str.substring(4);

        if (ref.startsWith('./')) {
            return schema.findLocal(ref.substring(2), resolveAll);
        } else if (ref.startsWith('../')) {
            while (ref.startsWith('../')) {
                ref = ref.substring(3);
                schema = schema.getParent();
            }

            return schema.findLocal(ref, resolveAll);
        } else if (resolveAll) {
            return schema.findAllGlobal(ref);
        } else {
            return schema.findGlobal(ref);
        }
    } else {
        return str;
    }
}
