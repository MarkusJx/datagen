import React, { useEffect, useState } from 'react';
import MonacoEditor from '@monaco-editor/react';
import { getSchemaFormat, SchemaFormat } from '../util/util';

interface Props {
  data: string;
  schema: string;
}

const GeneratedViewer: React.FC<Props> = ({ data, schema }) => {
  const [language, setLanguage] = useState(SchemaFormat.JSON);

  useEffect(() => {
    setLanguage(getSchemaFormat(schema));
  }, [data]);

  return (
    <MonacoEditor
      height="70vh"
      language={language}
      value={data}
      theme="vs-dark"
      options={{
        readOnly: true,
        scrollBeyondLastLine: false,
      }}
    />
  );
};

export default GeneratedViewer;
