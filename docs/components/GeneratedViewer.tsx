import React from 'react';
import MonacoEditor from '@monaco-editor/react';

interface Props {
  data: string | null;
}

const GeneratedViewer: React.FC<Props> = ({ data }) => {
  return (
    <MonacoEditor
      height="70vh"
      language="json"
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
