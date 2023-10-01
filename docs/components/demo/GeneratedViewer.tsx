import React, { useEffect, useState } from 'react';
import MonacoEditor from '@monaco-editor/react';
import { getSchemaFormat, SchemaFormat } from '../../util/util';
import { useTheme } from 'nextra-theme-docs';

interface Props {
  data: string;
  schema: string;
}

const GeneratedViewer: React.FC<Props> = ({ data, schema }) => {
  const [language, setLanguage] = useState(SchemaFormat.JSON);
  const { theme } = useTheme();

  useEffect(() => {
    setLanguage(getSchemaFormat(schema));
  }, [data]);

  return (
    <MonacoEditor
      height="70vh"
      language={language}
      value={data}
      theme={theme === 'dark' ? 'vs-dark' : 'light'}
      options={{
        readOnly: true,
        scrollBeyondLastLine: false,
      }}
    />
  );
};

export default GeneratedViewer;
