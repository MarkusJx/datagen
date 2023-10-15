import { Button } from 'nextra/components';
import React, { useEffect, useRef } from 'react';
import { renderChild } from '../../util/renderChild';
import { BsPlay } from 'react-icons/bs';
import { getCodeBlockCode } from '../../util/util';

interface Props {
  children: React.ReactNode;
  printUgly?: boolean;
}

const RunCode: React.FC<Props> = ({ children, printUgly }) => {
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const parent = ref.current?.querySelectorAll('button')?.item(0)
      ?.parentElement;
    if (!parent) {
      return;
    }

    parent.querySelectorAll('.run-button').forEach((button) => {
      button.remove();
    });

    let codeObj = JSON.parse(getCodeBlockCode(ref.current));
    if (!printUgly) {
      codeObj = {
        options: {
          serializer: {
            type: 'json',
            pretty: true,
          },
        },
        ...codeObj,
      };
    }

    const url = `${document.location.origin}/demo/?code=${encodeURIComponent(
      btoa(JSON.stringify(codeObj, null, 2))
    )}`;

    parent.appendChild(
      renderChild(
        <Button
          title="Run example"
          onClick={() => (document.location.href = url)}
        >
          <BsPlay size={22} />
        </Button>,
        'div',
        ['run-button']
      )
    );
  }, []);

  return <div ref={ref}>{children}</div>;
};

export default RunCode;
