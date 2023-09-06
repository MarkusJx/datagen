import Any from '../../base/schema/any';
import generateDate from './date';
import SchemaPath from './SchemaPath';
import generateString from './string';
import generateBoolean from './boolean';
import generateNumber from './number';
import InvalidSchemaError from '../errors/InvalidSchemaError';
import generateArray from './array';
import generateObject from './object';
import Plugins from '../plugins/Plugins';
import { chanceHit, randomArrayElement, resolveRef } from './util';
import generateOneOf from './oneOf';
import { SchemaOptions } from '../../base/schema/schemaDefinition';
import generateReference from './reference';

export default class Schema {
    private readonly plugins?: Plugins;
    private readonly properties: Map<string, any[]>;
    private readonly options: SchemaOptions;

    public constructor(
        private readonly input: Any,
        private readonly path: SchemaPath,
        private readonly parent?: Schema,
        private readonly current?: any,
        prev?: Schema | null,
        plugins?: Plugins,
        options?: SchemaOptions
    ) {
        this.plugins = plugins ?? parent?.plugins;
        this.properties = prev?.properties ?? new Map();
        this.options = options ?? parent!.options;
    }

    public static root(
        input: Any,
        plugins: Plugins,
        options: SchemaOptions
    ): Schema {
        return new Schema(
            input,
            SchemaPath.root(),
            undefined,
            undefined,
            undefined,
            plugins,
            options
        );
    }

    private async genData(): Promise<any> {
        switch (typeof this.input) {
            case 'string':
                return resolveRef(
                    this.input,
                    (!this.current && this.parent) || this
                );
            case 'number':
            case 'boolean':
                return this.input;
            case 'object':
                if (this.input === null) {
                    return null;
                }
        }

        switch (this.input.type) {
            case 'number':
                return generateNumber(this.input, this.path);
            case 'boolean':
                return generateBoolean(this.input, this.path);
            case 'string':
                return generateString(this.input, this.path, this);
            case 'array':
                return generateArray(this.input, this.path, this);
            case 'object':
                return generateObject(this.input, this.path, this);
            case 'date':
                return generateDate(this.input, this.path);
            case 'oneOf':
                return generateOneOf(this.input, this.path, this);
            case 'reference':
                return generateReference(this.input, this);
            case 'generator':
                if (this.plugins == undefined) {
                    throw new Error('Plugins not defined');
                }

                return this.plugins
                    .get(this.input.name)
                    .generate(this.input.args);
            default:
                throw new InvalidSchemaError(
                    'any',
                    this.path,
                    // @ts-ignore
                    `Unknown type: ${input.type}`
                );
        }
    }

    public async generateData(): Promise<any> {
        const data = await this.genData();
        if (this.input == null || typeof this.input !== 'object') {
            return data;
        }

        if (
            this.input.optional &&
            chanceHit(
                this.input.chanceUnset ?? 0.5,
                this.path.append('chanceUnset')
            )
        ) {
            return undefined;
        } else if (this.input.transform) {
            if (this.plugins == undefined) {
                throw new Error('Plugins not defined');
            }

            return this.plugins
                .get(this.input.transform)
                .transform(data, this.input.transformArgs);
        }

        const commonPath = this.path.toCommonPath();
        const properties = this.globalProperties.get(commonPath);
        if (
            this.input.unique &&
            properties != undefined &&
            properties.includes(data)
        ) {
            return this.generateData();
        }

        this.addProperty(commonPath, data);
        return data;
    }

    private get globalProperties(): Map<string, any[]> {
        return this.parent?.globalProperties ?? this.properties;
    }

    private addProperty(path: string, value: any): void {
        const props = this.properties.get(path);
        if (props == undefined) {
            this.properties.set(path, [value]);
        } else if (!props.includes(value)) {
            props.push(value);
        }

        if (this.parent) {
            this.parent.addProperty(path, value);
        }
    }

    public findAllGlobal(path: string, ignoreNotFound: boolean = false): any[] {
        const properties = this.globalProperties.get(path);
        if (properties == undefined) {
            if (this.options.ignoreNotFoundGlobalRefs || ignoreNotFound) {
                return [];
            }

            throw new Error(`Could not find path ${path}`);
        } else {
            return properties;
        }
    }

    public findGlobal(path: string): any {
        let properties = this.globalProperties.get(path);
        if (this.options.doNotReferenceSelf) {
            let self: any | undefined = undefined;
            try {
                self = this.findLocal(path.split('.').pop()!);
            } catch (ignored) {}

            if (self != undefined) {
                properties = properties?.filter((e) => e !== self);
            }

            if (properties && properties.length === 0) {
                return this.options.notFoundRefValue ?? null;
            }
        }

        if (properties == undefined) {
            if (this.options.ignoreNotFoundGlobalRefs) {
                return this.options.notFoundRefValue ?? null;
            }

            throw new Error(`Could not find path ${path}`);
        } else {
            return randomArrayElement(properties);
        }
    }

    public findLocal(path: string, returnAll: boolean = false): any {
        if (returnAll) {
            return this.properties.get(path) ?? [];
        }

        const parts = path.split('.');
        let current: any = this.current;

        const returnNow = (
            msg: string = ` in current path: ${this.path.toString()}`
        ) => {
            if (this.options.ignoreNotFoundLocalRefs) {
                return this.options.notFoundRefValue ?? null;
            }

            throw new Error(`Could not find path '${path}'${msg}`);
        };

        if (current == undefined) {
            return returnNow(': This schema has currently no value');
        }

        for (const part of parts) {
            if (Array.isArray(current)) {
                const parsed = parseInt(part);
                if (isNaN(parsed)) {
                    const rand = randomArrayElement(current);
                    if (rand != undefined && rand[part] != undefined) {
                        current = rand[part];
                    } else {
                        return returnNow();
                    }
                } else if (current[parsed] == undefined) {
                    return returnNow();
                } else {
                    current = current[parsed];
                }
            } else if (current[part] == undefined) {
                return returnNow();
            } else {
                current = current[part];
            }
        }
        return current;
    }

    public getParent(): Schema {
        if (!this.parent) {
            throw new Error(
                `No parent found in path '${this.path.toString()}'`
            );
        }

        return this.parent;
    }
}
