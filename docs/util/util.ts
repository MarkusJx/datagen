import { generateRandomData } from 'datagen-wasm';

export const generateData = async (
  schema: any,
  setGenerating: (generating: boolean) => void,
  setGenerated: (data: string) => void,
  isParsed: boolean
) => {
  try {
    setGenerating(true);
    setGenerated(
      await generateRandomData(isParsed ? schema : JSON.parse(schema))
    );
  } catch (e) {
    setGenerated('Error: ' + e.message);
  } finally {
    setGenerating(false);
  }
};
