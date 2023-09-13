import { CurrentSchema, NodePlugin } from '../native';
import { Transform, Array, Schema, Plugin as PluginSchema } from './types';

export interface Plugin {
    generate?(schema: CurrentSchema, args: any): any | Promise<any>;
    transform?(
        schema: CurrentSchema,
        args: any,
        value: any
    ): any | Promise<any>;
    serialize?(args: any, value: any): string | Promise<string>;
}

export type InitFunction = (args: any) => Plugin | Promise<Plugin>;

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

    for (const transform of schema.transform as Transform[]) {
        // @ts-ignore
        if (transform.type === 'plugin') {
            const name = (transform as any).name;
            if (!name) {
                throw Error("Object 'transform' must have a name");
            }

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

function transformError(e: any): Error {
    if (e instanceof Error) {
        return e;
    } else {
        return Error(e.toString());
    }
}

function callPluginFunction<
    N extends keyof Plugin,
    A extends Parameters<Required<Plugin>[N]>,
>(
    plugin: Plugin,
    pluginName: string,
    name: N,
    err: Error | null,
    callback: (res: ReturnType<Required<Plugin>[N]> | Error) => void,
    ...args: A
): void {
    const fn = plugin[name];
    if (err) {
        return callback(err);
    } else if (!fn) {
        return callback(
            Error(`Plugin '${pluginName}' does not support operation '${name}'`)
        );
    }

    try {
        // @ts-ignore
        const res = fn.apply(plugin, args);
        if (res instanceof Promise) {
            res.then(
                (res) => callback(res),
                (err) => callback(transformError(err))
            );
        } else {
            callback(res);
        }
    } catch (e) {
        callback(transformError(e));
    }
}

function createNodePlugin(name: string, plugin: Plugin): NodePlugin {
    return new NodePlugin(
        name,
        (err, callback, schema, args) =>
            callPluginFunction(
                plugin,
                name,
                'generate',
                err,
                callback,
                schema,
                args
            ),
        (err, callback, schema, args, value) =>
            callPluginFunction(
                plugin,
                name,
                'transform',
                err,
                callback,
                schema,
                args,
                value
            ),
        (err, callback, args, value) =>
            callPluginFunction(
                plugin,
                name,
                'serialize',
                err,
                callback,
                args,
                value
            )
    );
}

export async function findPlugins(
    schema: Schema,
    extraPlugins: Record<string, Plugin>
): Promise<Record<string, NodePlugin>> {
    const plugins: Record<string, Plugin> = Object.assign({}, extraPlugins);
    const pluginsObj = schema.options?.plugins;
    if (pluginsObj) {
        for (const [key, value] of Object.entries(pluginsObj)) {
            if (plugins[key]) {
                throw Error(`Duplicate plugin name: ${key}`);
            }

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
        if (name.startsWith('node:') && !plugins[name]) {
            plugins[name] = await loadPlugin(name.substring(5), null);
        }
    }

    await findNestedPlugins(schema, plugins);

    return Object.entries(plugins)
        .map(
            ([name, plugin]) =>
                [name, createNodePlugin(name, plugin)] as [string, NodePlugin]
        )
        .reduce(
            (acc, [name, plugin]) => {
                acc[name] = plugin;
                return acc;
            },
            {} as Record<string, NodePlugin>
        );
}
