import { CurrentSchema, InitFunction, Plugin } from '../.';

class TestPlugin implements Plugin {
    generate(_schema: CurrentSchema, _args: any): any {
        return {
            test: true,
        };
    }
}

const Init: InitFunction = async () => {
    return new TestPlugin();
};

export default Init;
