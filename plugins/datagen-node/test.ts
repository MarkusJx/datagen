import { generateRandomData } from './ts-src';

(async () => {
    const data = await generateRandomData({
        type: 'plugin',
        pluginName: 'node:../testPlugin.ts',
        args: {
            name: 'test',
        },
    });
    
    console.log('Generated:', data);
})();
