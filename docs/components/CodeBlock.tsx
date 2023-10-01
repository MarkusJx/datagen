import React, { useEffect, useRef } from 'react';

interface Props {
  children: React.ReactNode;
  searchText?: string;
  value: string | null | undefined;
}

const CodeBlock: React.FC<Props> = ({ children, searchText, value }) => {
  const ref = useRef<HTMLDivElement>(null);
  const search = searchText ?? 'unknown';

  useEffect(() => {
    if (ref.current) {
      const token = [...ref.current.querySelectorAll('code')].find(
        (el) => el.innerText === search
      );

      if (token) {
        token.innerText = value ?? search;
      }
    }
  }, [search, value]);

  return <div ref={ref}>{children}</div>;
};

export default CodeBlock;
