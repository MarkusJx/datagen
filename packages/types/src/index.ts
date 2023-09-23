export type * from './types';
import SchemaJson from './schema.json';
export { SchemaJson };

export interface CurrentSchema {
    child(path: string, sibling?: CurrentSchema | null): CurrentSchema;
    resolveRef(path: string): any[];
}

export interface DatagenPlugin {
    generate?(schema: CurrentSchema, args: any): any | Promise<any>;
    transform?(
        schema: CurrentSchema,
        args: any,
        value: any
    ): any | Promise<any>;
    serialize?(args: any, value: any): string | Promise<string>;
}

export type PluginInitFunction = (args: any) => DatagenPlugin | Promise<DatagenPlugin>;
