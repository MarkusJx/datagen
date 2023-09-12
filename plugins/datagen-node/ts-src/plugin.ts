import { CurrentSchema, NodePlugin } from '../index';
import { AnyTransform, Array, Schema, Plugin as PluginSchema } from './types';

export interface Plugin {
    generate?(schema: CurrentSchema, args: any): any;
    transform?(schema: CurrentSchema, args: any, value: any): any;
    serialize?(args: any, value: any): any;
}

export type InitFunction = (args: any) => Promise<Plugin>;

async function loadPlugin(name: string, args: any): Promise<Plugin> {
    let plugin = await import(name);
    if (plugin.default) {
        plugin = plugin.default;
    }

    return (plugin as InitFunction)(args);
}

async function findPluginsInTransform(
    schema: Schema,
    plugins: Record<string, Plugin>
): Promise<void> {
    if (!schema.transform) {
        return;
    }

    for (const transform of schema.transform as AnyTransform[]) {
        if ((transform as any).type === 'plugin') {
            const name = (transform as unknown as PluginSchema).pluginName;
            if (name.startsWith('node:') && !plugins[name]) {
                plugins[name] = await loadPlugin(name.substring(5), null);
            }
        }
    }
}

async function findNestedPlugins(
    schema: Schema,
    plugins: Record<string, Plugin>
): Promise<void> {
    switch (schema.type) {
        case 'array':
            return findNestedPlugins(schema.items as Schema, plugins);
        case 'object': {
            for (const value of Object.values(
                schema.properties as Record<string, Schema>
            )) {
                await findNestedPlugins(value as Schema, plugins);
            }
            break;
        }
        case 'anyOf': {
            for (const value of Object.values(
                (schema as Array).values as Record<string, Schema>
            )) {
                await findNestedPlugins(value as Schema, plugins);
            }
            break;
        }
        case 'plugin': {
            const name = (schema as PluginSchema).pluginName;
            if (name.startsWith('node:') && !plugins[name]) {
                const args = (schema as PluginSchema).args;
                plugins[name] = await loadPlugin(name.substring(5), args);
            }
            break;
        }
        default:
            return findPluginsInTransform(schema, plugins);
    }
}

export async function findPlugins(
    schema: Schema
): Promise<Record<string, NodePlugin>> {
    const plugins: Record<string, Plugin> = {};
    const pluginsObj = schema.options?.plugins;
    if (pluginsObj) {
        for (const [key, value] of Object.entries(pluginsObj)) {
            if (key.startsWith('node:')) {
                if (value && (value as any).path) {
                    plugins[key] = await loadPlugin(
                        (value as any).path,
                        (value as any).args
                    );
                } else {
                    plugins[key] = await loadPlugin(key.substring(5), value);
                }
            }
        }
    }

    if (
        schema.options?.serializer &&
        schema.options.serializer.type === 'plugin'
    ) {
        const name = schema.options.serializer.pluginName;
        if (!plugins[name]) {
            plugins[name] = await loadPlugin(name, null);
        }
    }

    await findNestedPlugins(schema, plugins);
    return Object.entries(plugins)
        .map(
            ([name, plugin]) =>
                [
                    name,
                    new NodePlugin(
                        name,
                        (err, schema, args) => {
                            console.log(err, schema, args);
                            if (!plugin.generate) {
                                throw new Error(
                                    'Plugin does not support generate'
                                );
                            }

                            return plugin.generate(schema, args);
                        },
                        ({ args, schema, value }) => {
                            if (!plugin.transform) {
                                throw new Error(
                                    'Plugin does not support transform'
                                );
                            }

                            return plugin.transform(schema, args, value);
                        },
                        ({ args, value }) => {
                            if (!plugin.serialize) {
                                throw new Error(
                                    'Plugin does not support serialize'
                                );
                            }

                            return plugin.serialize(args, value);
                        }
                    ),
                ] as [string, NodePlugin]
        )
        .reduce(
            (acc, [name, plugin]) => {
                acc[name] = plugin;
                return acc;
            },
            {} as Record<string, NodePlugin>
        );
}
