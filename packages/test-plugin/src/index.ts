import { CurrentSchema, DatagenPlugin, PluginInitFunction } from 'datagen-rs-types';

class TestPlugin implements DatagenPlugin {
  generate(_schema: CurrentSchema, args: any): any {
    console.log('TestPlugin.generate called with args:', args);
    return 'TestPlugin';
  }

  [Symbol.for('nodejs.util.inspect.custom')](): string {
    return 'TestPlugin';
  }
}

const init: PluginInitFunction = (args) => {
  console.log('Init called with args:', args);
  return new TestPlugin();
};

export default init;
