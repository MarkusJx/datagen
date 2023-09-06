import SchemaDefinition from '../../base/schema/schemaDefinition';
import Any from '../../base/schema/any';

export function findPluginsToLoad(schema: SchemaDefinition | Any): string[] {
    if (schema == null || typeof schema !== 'object') {
        return [];
    }

    let plugins: string[] = [];
    if ((schema as SchemaDefinition).options?.plugins) {
        plugins = Object.keys((schema as SchemaDefinition).options?.plugins!);
    }

    if (schema.transform) {
        plugins.push(schema.transform);
    }

    switch (schema.type) {
        case 'array':
            return plugins.concat(...findPluginsToLoad(schema.items));
        case 'object':
            return plugins.concat(
                ...Object.keys(schema.properties).reduce((plugins, key) => {
                    return plugins.concat(
                        ...findPluginsToLoad(schema.properties[key])
                    );
                }, [] as string[])
            );
        case 'generator':
            return plugins.concat(schema.name);
        default:
            return plugins;
    }
}

export function unique<T>(value: T, index: number, array: T[]): boolean {
    return array.indexOf(value) === index;
}
