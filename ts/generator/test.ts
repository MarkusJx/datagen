import data from '../test.json';
import { generateData } from './index';
import SchemaDefinition from '../base/schema/schemaDefinition';

(async () => {
    console.time();
    const generated = await generateData(data as SchemaDefinition);
    console.timeEnd();
    console.log(generated.length);
    //console.log(JSON.stringify(generated, null, 2));
})();
