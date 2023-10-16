import React from 'react';
import { useCodeContext } from './RunCode';
import SyntaxHighlighter from '../util/SyntaxHighlighter';

const Generated: React.FC = () => {
  const { generated } = useCodeContext();

  if (generated) {
    return <SyntaxHighlighter code={generated} language="json" />;
  } else {
    return null;
  }
};

export default Generated;
