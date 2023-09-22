import React from 'react';
import MonacoEditor from '@monaco-editor/react';

interface Props {
  monaco: typeof import('monaco-editor/esm/vs/editor/editor.api');
  schema: string;
  setSchema: (schema: string) => void;
  disabled?: boolean;
}

const SchemaEditor: React.FC<Props> = ({ monaco, schema, setSchema }) => {
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
      theme="vs-dark"
      onChange={(value) => setSchema(value ?? '')}
      options={{
        scrollBeyondLastLine: false,
      }}
    />
  );
};

export default SchemaEditor;
