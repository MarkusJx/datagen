import React, { useEffect } from 'react';
import MonacoEditor, { useMonaco } from '@monaco-editor/react';
import { useTheme } from 'nextra-theme-docs';
import { SchemaJson } from '@datagen-rs/types';

interface Props {
  schema: string;
  setSchema: (schema: string) => void;
}

const SchemaEditor: React.FC<Props> = ({ schema, setSchema }) => {
  const { theme } = useTheme();
  const monaco = useMonaco();

  useEffect(() => {
    if (!monaco) return;

    monaco.languages.json.jsonDefaults.setDiagnosticsOptions({
      validate: true,
      allowComments: false,
      enableSchemaRequest: true,
      schemas: [
        {
          uri: '',
          schema: SchemaJson,
          fileMatch: ['*'],
        },
      ],
    });
  }, [monaco]);

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
