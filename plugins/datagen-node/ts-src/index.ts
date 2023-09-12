import { Schema } from './types';
import { generateRandomDataInternalAsync } from '../index';
import { findPlugins } from './plugin';

export async function generateRandomData(schema: Schema): Promise<object> {
    const data = JSON.stringify(schema);
    const generated = await generateRandomDataInternalAsync(
        data,
        await findPlugins(schema)
    );
    return JSON.parse(generated);
}
