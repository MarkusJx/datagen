import { PluginExports } from '../../base/plugin';
import OperationNotSupportedError from '../errors/OperationNotSupportedError';

export default class Plugin {
    private constructor(
        public readonly name: string,
        private readonly plugin: PluginExports
    ) {}

    public static async load(path: string): Promise<Plugin> {
        const plugin = await import(path);
        return new Plugin(path, plugin);
    }

    public async init(args: any): Promise<void> {
        if (this.plugin.init != undefined) {
            await this.plugin.init(args);
        }
    }

    public async transform<T = any>(value: T, args: any): Promise<T> {
        if (this.plugin.transform != undefined) {
            return this.plugin.transform(value, args);
        } else {
            throw new OperationNotSupportedError('transform', this.name);
        }
    }

    public async generate(args: any): Promise<any> {
        if (this.plugin.generate != undefined) {
            return this.plugin.generate(args);
        } else {
            throw new OperationNotSupportedError('generate', this.name);
        }
    }
}
