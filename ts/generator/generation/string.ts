import String, {
    ConstantString,
    Email,
    Format,
    GeneratedString,
    Username,
} from '../../base/schema/string';
import SchemaPath from './SchemaPath';
import InvalidSchemaError from '../errors/InvalidSchemaError';
import Fakerator from 'fakerator';
import { resolveRef } from './util';
import Schema from './Schema';
import Any from '../../base/schema/any';

const fakerator = Fakerator();

async function formatString(
    format: string,
    args: Any[],
    path: SchemaPath,
    schema: Schema
): Promise<string> {
    const regex = /{\d+}/gm;

    let array = regex.exec(format);
    if (array != null) {
        const split = format.split(/{\d+}/gm);
        let res = '';

        while (array != null) {
            for (const match of array) {
                const index =
                    parseInt(match.substring(1, match.length - 1)) - 1;
                if (index < 0 || index >= args.length) {
                    throw new InvalidSchemaError(
                        'string',
                        path,
                        `Index out of bounds: ${index + 1}`
                    );
                } else {
                    const newSchema = new Schema(
                        args[index],
                        path.append(index.toString()),
                        schema
                    );

                    res +=
                        split.shift() +
                        (await newSchema.generateData()).toString();
                }
            }

            array = regex.exec(format);
        }

        return res + split.join('');
    } else {
        throw new InvalidSchemaError(
            'string',
            path,
            `Invalid format: ${format}`
        );
    }
}

export default async function generateString(
    args: String,
    path: SchemaPath,
    schema: Schema
): Promise<string> {
    if ((args as ConstantString).value != undefined) {
        return resolveRef((args as ConstantString).value, schema);
    } else if (
        typeof (args as GeneratedString).generator === 'object' &&
        (args as GeneratedString).generator != undefined
    ) {
        switch ((args as GeneratedString).generator.type) {
            case 'uuid':
                return fakerator.misc.uuid();
            case 'address':
                return (
                    fakerator.address.street() +
                    ' ' +
                    fakerator.address.buildingNumber() +
                    ', ' +
                    fakerator.address.postCode() +
                    ' ' +
                    fakerator.address.city() +
                    ', ' +
                    fakerator.address.country()
                );
            case 'email': {
                const gen = (args as GeneratedString).generator as Email;
                return fakerator.internet.email(
                    resolveRef(gen.firstName, schema)!,
                    resolveRef(gen.lastName, schema)!,
                    resolveRef(gen.domain, schema)!
                );
            }
            case 'firstName':
                return fakerator.names.firstName();
            case 'lastName':
                return fakerator.names.lastName();
            case 'fullName':
                return fakerator.names.name();
            case 'companyName':
                return fakerator.company.name();
            case 'website':
                return fakerator.internet.domain();
            case 'phoneNumber':
                return fakerator.phone.number();
            case 'country':
                return fakerator.address.country();
            case 'city':
                return fakerator.address.city();
            case 'zipCode':
                return fakerator.address.postCode();
            case 'latitude':
                return fakerator.address.geoLocation().latitude.toString();
            case 'longitude':
                return fakerator.address.geoLocation().longitude.toString();
            case 'color':
                return fakerator.internet.color();
            case 'title':
                return fakerator.names.prefix();
            case 'username': {
                const gen = (args as GeneratedString).generator as Username;
                return fakerator.internet.userName(
                    resolveRef(gen.firstName, schema)!,
                    resolveRef(gen.lastName, schema)!
                );
            }
            case 'format': {
                const gen = (args as GeneratedString).generator as Format;
                return formatString(gen.format, gen.args, path, schema);
            }
            default:
                throw new InvalidSchemaError(
                    'string',
                    path,
                    `Unknown generator type: ${
                        (args as GeneratedString).generator.type
                    }`
                );
        }
    } else {
        throw new InvalidSchemaError(
            'string',
            path,
            "Expected either 'value' or 'generator' to be defined"
        );
    }
}
