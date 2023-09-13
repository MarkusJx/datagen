import { InitFunction, Plugin } from './ts-src/plugin';
import { CurrentSchema } from './index';

class TestPlugin implements Plugin {
    generate(schema: CurrentSchema, args: any): any {
        console.log(schema, args);
        console.log(schema.resolveRef('ref:test123'));
        return {
            test: true,
        };
    }
}

const Init: InitFunction = async (args) => {
    return new TestPlugin();
};

export = Init;
