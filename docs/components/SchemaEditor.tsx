import React from 'react';
import MonacoEditor from '@monaco-editor/react';
import { useTheme } from 'nextra-theme-docs';

interface Props {
  monaco: typeof import('monaco-editor/esm/vs/editor/editor.api');
  schema: string;
  setSchema: (schema: string) => void;
  disabled?: boolean;
}

const SchemaEditor: React.FC<Props> = ({ monaco, schema, setSchema }) => {
  const { theme } = useTheme();

  monaco.languages.json.jsonDefaults.setDiagnosticsOptions({
    validate: true,
    allowComments: false,
    enableSchemaRequest: true,
    schemas: [
      {
        uri: 'https://raw.githubusercontent.com/MarkusJx/datagen/main/datagen-rs/schema.json',
        fileMatch: ['*'],
      },
    ],
  });

  return (
    <MonacoEditor
      height="70vh"
      language="json"
      value={schema}
      theme={theme === 'dark' ? 'vs-dark' : 'light'}
      onChange={(value) => setSchema(value ?? '')}
      options={{
        scrollBeyondLastLine: false,
      }}
    />
  );
};

export default SchemaEditor;
