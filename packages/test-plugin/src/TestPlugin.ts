import { CurrentSchema, DatagenPlugin, PluginInitFunction } from 'datagen-rs-types';

interface PluginArgs {
  generate?: boolean;
  transform?: boolean;
  serialize?: boolean;
  [key: string]: any;
}

class TestPlugin implements DatagenPlugin {
  public constructor(private readonly args: PluginArgs) {
  }

  public generate(_schema: CurrentSchema, args: any): any {
    return { ...args, ...this.args };
  }

  public transform(_schema: CurrentSchema, args: any, value: any): any {
    return { ...args, ...this.args, ...value };
  }

  public serialize(args: any, value: any): string | Promise<string> {
    return JSON.stringify({ ...args, ...this.args, ...value });
  }
}

const init: PluginInitFunction = (args: PluginArgs) => {
  return new TestPlugin(args);
};

export default init;
