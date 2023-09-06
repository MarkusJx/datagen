import { findPluginsToLoad, unique } from './util';
import Plugin from './Plugin';
import SchemaDefinition from '../../base/schema/schemaDefinition';

export default class Plugins {
    private constructor(private readonly plugins: Record<string, Plugin>) {}

    public static async initializePlugins(
        schema: SchemaDefinition
    ): Promise<Plugins> {
        const pluginsToLoad = findPluginsToLoad(schema).filter(unique);

        const plugins = await Promise.all(
            pluginsToLoad.map((plugin) => Plugin.load(plugin))
        );

        const pluginArgs = schema.options?.plugins || {};
        await Promise.all(
            plugins.map((plugin) => plugin.init(pluginArgs[plugin.name]))
        );

        return new Plugins(
            plugins.reduce(
                (plugins, plugin) => {
                    plugins[plugin.name] = plugin;
                    return plugins;
                },
                {} as Record<string, Plugin>
            )
        );
    }

    public get(name: string): Plugin {
        return this.plugins[name];
    }
}
