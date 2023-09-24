import {
  DatagenPlugin,
  PluginInitFunction,
  CurrentSchema,
} from '@datagen/types';

class TestPlugin implements DatagenPlugin {
  generate(_schema: CurrentSchema, _args: any): any {
    return {
      test: true,
    };
  }
}

const Init: PluginInitFunction = async () => {
  return new TestPlugin();
};

export default Init;
