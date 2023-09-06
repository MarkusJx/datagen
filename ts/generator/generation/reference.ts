import Reference from '../../base/schema/reference';
import Schema from './Schema';
import { randomArrayElement, resolveRef } from './util';

export default function generateReference(
    args: Reference,
    schema: Schema
): any {
    const refName = args.ref.startsWith('ref:') ? args.ref : `ref:${args.ref}`;
    const ref = resolveRef(refName, schema, true);
    if (!Array.isArray(ref)) {
        return ref ?? null;
    }

    let except: (string | number)[] = [];
    if (args.except) {
        except = args.except
            .flatMap((e) => resolveRef(e, schema, true))
            .filter((e) => e != null);
    }

    const filtered = ref.filter((e) => !except.includes(e));
    return randomArrayElement(filtered) ?? null;
}
