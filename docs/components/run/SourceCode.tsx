import React, { useEffect, useRef, useState } from 'react';
import { useCodeContext } from './RunCode';
import { getCodeBlockCode } from '../../util/util';
import { renderChild } from '../../util/renderChild';
import { Button } from 'nextra/components';
import { BsPlay } from 'react-icons/bs';
import { Schema } from 'datagen-rs-types';
import { useGlobalContext } from '../util/GlobalContext';

interface Props {
  children: React.ReactNode;
}

const SourceCode: React.FC<Props> = ({ children }) => {
  const ref = useRef<HTMLDivElement>(null);
  const [generating, setGenerating] = useState(false);
  const { setGenerated } = useCodeContext();
  const {
    workerError,
    workerInitialized,
    workerSupported,
    generateRandomData,
  } = useGlobalContext();

  const handleRun = (code: Schema) => {
    if (generating) {
      return;
    }

    if (workerError) {
      setGenerated(workerError);
    } else {
      setGenerated('Generating...');
      generateRandomData(code, setGenerating, setGenerated, true).catch((e) =>
        console.error('Failed to generate data', e)
      );
    }
  };

  useEffect(() => {
    if (!ref.current || !workerInitialized || !workerSupported) {
      return;
    }

    const parent = ref.current
      ?.querySelectorAll('button')
      ?.item(0)?.parentElement;
    if (!parent) {
      return;
    }

    parent.querySelectorAll('.run-button').forEach((button) => {
      button.remove();
    });

    const code = {
      options: {
        serializer: {
          type: 'json',
          pretty: true,
        },
      },
      ...JSON.parse(getCodeBlockCode(ref.current)),
    };

    parent.appendChild(
      renderChild(
        <Button title="Run example" onClick={() => handleRun(code)}>
          <BsPlay size={22} />
        </Button>,
        'div',
        ['run-button']
      )
    );
  }, [workerInitialized, workerSupported, ref.current]);

  return (
    <div ref={ref} style={{ marginTop: '1.5rem' }}>
      {children}
    </div>
  );
};

export default SourceCode;
