import React, { createContext, useContext, useEffect, useState } from 'react';
import Generated from './Generated';
import SourceCode from './SourceCode';
import { getHighlighter, Highlighter } from 'shiki';

interface CodeContext {
  generated: string | null;
  setGenerated: (result: string) => void;
  highlighter: Highlighter | null;
}

const Context = createContext<CodeContext>({
  generated: null,
  setGenerated: () => {},
  highlighter: null,
});

export const useCodeContext = () => {
  return useContext(Context);
};

interface Props {
  children: React.ReactNode;
}

const highlighterPromise = getHighlighter({
  theme: 'css-variables',
  langs: ['json'],
  themes: ['css-variables'],
});

const RunCode: React.FC<Props> = ({ children }) => {
  const [generated, setGenerated] = useState<string | null>(null);
  const [highlighter, setHighlighter] = useState<Highlighter | null>(null);

  useEffect(() => {
    highlighterPromise.then(setHighlighter).catch(console.error);
  }, []);

  return (
    <Context.Provider
      value={{
        generated,
        setGenerated,
        highlighter,
      }}
    >
      <SourceCode>{children}</SourceCode>
      <Generated />
    </Context.Provider>
  );
};

export default RunCode;
