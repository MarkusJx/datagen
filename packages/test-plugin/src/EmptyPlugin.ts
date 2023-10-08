import { DatagenPlugin, PluginInitFunction } from 'datagen-rs-types';

class EmptyPlugin implements DatagenPlugin {
}

const init: PluginInitFunction = () => {
  return new EmptyPlugin();
};

export default init;
