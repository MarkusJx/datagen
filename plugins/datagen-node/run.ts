import { generateRandomData } from './ts-src';

(async () => {
    const data = await generateRandomData({
        type: 'plugin',
        pluginName: 'node:../testPlugin.ts',
        args: {
            name: 'test',
        },
        //*
        transform: [
            {
                type: 'plugin',
                name: 'node:../testPlugin.ts',
            },
        ], //*/
    });

    console.log('Generated:', data);
})();
