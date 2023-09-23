import React, { useEffect } from 'react';
import MonacoEditor, { useMonaco } from '@monaco-editor/react';
import { useTheme } from 'nextra-theme-docs';
import jsonSchema from '../../packages/schema/dist/schema.json';

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
          schema: jsonSchema,
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
