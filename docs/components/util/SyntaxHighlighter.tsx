import React, { useEffect } from 'react';
import { Lang, setCDN } from 'shiki';
import HtmlBlock from './HtmlBlock';
import { Code, Pre } from 'nextra/components';
import { useCodeContext } from '../run/RunCode';

interface Props {
  code: string;
  language: Lang;
}

setCDN('https://unpkg.com/shiki/');

const SyntaxHighlighter: React.FC<Props> = ({ code, language }) => {
  const { highlighter } = useCodeContext();
  const [rendered, setRendered] = React.useState<string | null>(null);

  useEffect(() => {
    if (highlighter) {
      const tokens = highlighter.codeToHtml(code, {
        lang: 'json',
        theme: 'css-variables',
      });

      const div = document.createElement('div');
      div.innerHTML = tokens;

      setRendered(div.querySelector('code')?.innerHTML ?? tokens);
    }
  }, [highlighter, code, language]);

  return (
    <Pre hasCopyCode={true} {...{ 'data-theme': 'default' }}>
      <Code>
        <HtmlBlock html={rendered ?? 'Generating...'} />
      </Code>
    </Pre>
  );
};

export default SyntaxHighlighter;
