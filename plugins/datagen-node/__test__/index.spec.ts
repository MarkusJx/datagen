import test from 'ava';
import {
    CurrentSchema,
    GenerateProgress,
    generateRandomData,
    generateRandomDataWithProgress,
} from '../.';

test('generate data', async (t) => {
    const generated = await generateRandomData({
        type: 'string',
        value: 'test',
    });

    t.is(JSON.parse(generated), 'test');
});

test('generate data with plugin', async (t) => {
    const generated = await generateRandomData(
        {
            type: 'plugin',
            pluginName: 'testPlugin',
            args: {
                name: 'test',
            },
        },
        {
            testPlugin: {
                generate(schema: CurrentSchema, args: any): any {
                    t.deepEqual(args, { name: 'test' });
                    t.deepEqual(schema.resolveRef('ref:test'), []);
                    return 'test';
                },
            },
        }
    );

    t.is(JSON.parse(generated), 'test');
});

test('transform data with plugin', async (t) => {
    const generated = await generateRandomData(
        {
            type: 'string',
            value: 'test',
            transform: [
                {
                    type: 'plugin',
                    name: 'testPlugin',
                    args: {
                        name: 'test',
                    },
                },
            ],
        },
        {
            testPlugin: {
                transform(schema: CurrentSchema, args: any, value: any): any {
                    t.is(value, 'test');
                    t.deepEqual(args, { name: 'test' });
                    t.deepEqual(schema.resolveRef('ref:test'), []);
                    return value.toUpperCase();
                },
            },
        }
    );

    t.is(JSON.parse(generated), 'TEST');
});

test('serialize data with plugin', async (t) => {
    const generated = await generateRandomData(
        {
            type: 'string',
            value: 'test',
            options: {
                serializer: {
                    type: 'plugin',
                    pluginName: 'testPlugin',
                    args: {
                        name: 'test',
                    },
                },
            },
        },
        {
            testPlugin: {
                serialize(args: any, value: any): any {
                    t.is(value, 'test');
                    t.deepEqual(args, { name: 'test' });
                    return value.toUpperCase();
                },
            },
        }
    );

    t.is(generated, 'TEST');
});

test('generate data with async plugin', async (t) => {
    const generated = await generateRandomData(
        {
            type: 'plugin',
            pluginName: 'testPlugin',
            args: {
                name: 'test',
            },
            transform: [
                {
                    type: 'plugin',
                    name: 'testPlugin',
                    args: {
                        name1: 'test1',
                    },
                },
            ],
            options: {
                serializer: {
                    type: 'plugin',
                    pluginName: 'testPlugin',
                    args: {
                        name2: 'test2',
                    },
                },
            },
        },
        {
            testPlugin: {
                async generate(schema: CurrentSchema, args: any): Promise<any> {
                    t.deepEqual(args, { name: 'test' });
                    t.deepEqual(schema.resolveRef('ref:test'), []);
                    return 'test';
                },
                async transform(
                    schema: CurrentSchema,
                    args: any,
                    value: any
                ): Promise<any> {
                    t.is(value, 'test');
                    t.deepEqual(args, { name1: 'test1' });
                    t.deepEqual(schema.resolveRef('ref:test'), []);
                    return value.toUpperCase();
                },
                async serialize(args: any, value: any): Promise<any> {
                    t.is(value, 'TEST');
                    t.deepEqual(args, { name2: 'test2' });
                    return value + '!';
                },
            },
        }
    );

    t.is(generated, 'TEST!');
});

test('generate data with invalid plugin', async (t) => {
    await t.throwsAsync(
        generateRandomData(
            {
                type: 'plugin',
                pluginName: 'testPlugin',
            },
            {
                testPlugin: {},
            }
        ),
        {
            code: 'GenericFailure',
            message:
                "Failed to call function 'generate' on plugin 'testPlugin': Plugin 'testPlugin' does not support operation 'generate'",
        }
    );
});

test('transform data with invalid plugin', async (t) => {
    await t.throwsAsync(
        generateRandomData(
            {
                type: 'string',
                value: 'test',
                transform: [
                    {
                        type: 'plugin',
                        name: 'testPlugin',
                    },
                ],
            },
            {
                testPlugin: {},
            }
        ),
        {
            code: 'GenericFailure',
            message:
                "Failed to call function 'transform' on plugin 'testPlugin': Plugin 'testPlugin' does not support operation 'transform'",
        }
    );
});

test('serialize data with invalid plugin', async (t) => {
    await t.throwsAsync(
        generateRandomData(
            {
                type: 'string',
                value: 'test',
                options: {
                    serializer: {
                        type: 'plugin',
                        pluginName: 'testPlugin',
                    },
                },
            },
            {
                testPlugin: {},
            }
        ),
        {
            code: 'GenericFailure',
            message:
                "Failed to call function 'serialize' on plugin 'testPlugin': Plugin 'testPlugin' does not support operation 'serialize'",
        }
    );
});

test('generate data with invalid plugin name', async (t) => {
    await t.throwsAsync(
        generateRandomData(
            {
                type: 'plugin',
                pluginName: 'testPlugin',
            },
            {}
        ),
        {
            code: 'GenericFailure',
            message: /^Failed to load plugin 'testPlugin': .+/gm,
        }
    );
});

test('transform data with invalid plugin name', async (t) => {
    await t.throwsAsync(
        generateRandomData(
            {
                type: 'string',
                value: 'test',
                transform: [
                    {
                        type: 'plugin',
                        name: 'testPlugin',
                    },
                ],
            },
            {}
        ),
        {
            code: 'GenericFailure',
            message: /^Failed to load plugin 'testPlugin': .+/gm,
        }
    );
});

test('serialize data with invalid plugin name', async (t) => {
    await t.throwsAsync(
        generateRandomData(
            {
                type: 'string',
                value: 'test',
                options: {
                    serializer: {
                        type: 'plugin',
                        pluginName: 'testPlugin',
                    },
                },
            },
            {}
        ),
        {
            code: 'GenericFailure',
            message: /^Failed to load plugin 'testPlugin': .+/gm,
        }
    );
});

test('generate data with throwing plugin', async (t) => {
    await t.throwsAsync(
        generateRandomData(
            {
                type: 'plugin',
                pluginName: 'testPlugin',
            },
            {
                testPlugin: {
                    generate(): any {
                        throw Error('test');
                    },
                },
            }
        ),
        {
            code: 'GenericFailure',
            message:
                "Failed to call function 'generate' on plugin 'testPlugin': test",
        }
    );
});

test('transform data with throwing plugin', async (t) => {
    await t.throwsAsync(
        generateRandomData(
            {
                type: 'string',
                value: 'test',
                transform: [
                    {
                        type: 'plugin',
                        name: 'testPlugin',
                    },
                ],
            },
            {
                testPlugin: {
                    transform(): any {
                        throw Error('test');
                    },
                },
            }
        ),
        {
            code: 'GenericFailure',
            message:
                "Failed to call function 'transform' on plugin 'testPlugin': test",
        }
    );
});

test('serialize data with throwing plugin', async (t) => {
    await t.throwsAsync(
        generateRandomData(
            {
                type: 'string',
                value: 'test',
                options: {
                    serializer: {
                        type: 'plugin',
                        pluginName: 'testPlugin',
                    },
                },
            },
            {
                testPlugin: {
                    serialize(): any {
                        throw Error('test');
                    },
                },
            }
        ),
        {
            code: 'GenericFailure',
            message:
                "Failed to call function 'serialize' on plugin 'testPlugin': test",
        }
    );
});

test('generate data with throwing plugin (async)', async (t) => {
    await t.throwsAsync(
        generateRandomData(
            {
                type: 'plugin',
                pluginName: 'testPlugin',
            },
            {
                testPlugin: {
                    async generate(): Promise<any> {
                        throw Error('test');
                    },
                },
            }
        ),
        {
            code: 'GenericFailure',
            message:
                "Failed to call function 'generate' on plugin 'testPlugin': test",
        }
    );
});

test('transform data with throwing plugin (async)', async (t) => {
    await t.throwsAsync(
        generateRandomData(
            {
                type: 'string',
                value: 'test',
                transform: [
                    {
                        type: 'plugin',
                        name: 'testPlugin',
                    },
                ],
            },
            {
                testPlugin: {
                    async transform(): Promise<any> {
                        throw Error('test');
                    },
                },
            }
        ),
        {
            code: 'GenericFailure',
            message:
                "Failed to call function 'transform' on plugin 'testPlugin': test",
        }
    );
});

test('serialize data with throwing plugin (async)', async (t) => {
    await t.throwsAsync(
        generateRandomData(
            {
                type: 'string',
                value: 'test',
                options: {
                    serializer: {
                        type: 'plugin',
                        pluginName: 'testPlugin',
                    },
                },
            },
            {
                testPlugin: {
                    async serialize(): Promise<any> {
                        throw Error('test');
                    },
                },
            }
        ),
        {
            code: 'GenericFailure',
            message:
                "Failed to call function 'serialize' on plugin 'testPlugin': test",
        }
    );
});

test('generate data with imported plugin', async (t) => {
    const generated = await generateRandomData({
        type: 'plugin',
        pluginName: `node:${__dirname}/testPlugin.ts`,
    });

    t.deepEqual(JSON.parse(generated), {
        test: true,
    });
});

test('generate data with progress', async (t) => {
    const progresses: GenerateProgress[] = [];
    await generateRandomDataWithProgress(
        {
            type: 'array',
            length: {
                value: 10,
            },
            items: {
                type: 'string',
                value: 'test',
            },
        },
        (progress) => {
            progresses.push(progress);
        }
    );

    const expectedProgresses: GenerateProgress[] = [];
    for (let i = 1; i <= 10; i++) {
        expectedProgresses.push({
            current: i,
            total: 10,
        });
    }

    t.is(progresses.length, 10);
    t.deepEqual(progresses, expectedProgresses);
});
