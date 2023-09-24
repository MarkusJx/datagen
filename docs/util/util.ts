import { Schema } from '@datagen-rs/types';

export const wasmSupported = () => {
  try {
    if (
      typeof WebAssembly === 'object' &&
      typeof WebAssembly.instantiate === 'function'
    ) {
      const module = new WebAssembly.Module(
        Uint8Array.of(0x0, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00)
      );
      if (module instanceof WebAssembly.Module)
        return new WebAssembly.Instance(module) instanceof WebAssembly.Instance;
    }
  } catch (e) {}
  return false;
};

export const webWorkersSupported = () => {
  return typeof Worker !== 'undefined';
};

export const downloadFile = (
  data: string,
  filename: string,
  format: SchemaFormat
) => {
  const blob = new Blob([data], { type: 'text/plain' });
  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');
  link.href = url;
  link.download = `${filename}.${format}`;
  link.click();
};

export const enum SchemaFormat {
  JSON = 'json',
  YAML = 'yaml',
  XML = 'xml',
}

export const getSchemaFormat = (schema: string): SchemaFormat => {
  try {
    const parsed: Schema = JSON.parse(schema);
    switch (parsed?.options?.serializer?.type?.toLowerCase()) {
      case 'yaml':
        return SchemaFormat.YAML;
      case 'xml':
        return SchemaFormat.XML;
      case 'json':
        return SchemaFormat.JSON;
    }
  } catch (_) {}

  return SchemaFormat.JSON;
};
