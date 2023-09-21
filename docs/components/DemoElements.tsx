import React, { useEffect, useState } from 'react';
import { Callout } from 'nextra/components';
import { useMonaco } from '@monaco-editor/react';
import init from 'datagen-wasm';
import DemoGrid from './DemoGrid';
import GenerateButton from './GenerateButton';
import SchemaEditor from './SchemaEditor';
import GeneratedViewer from './GeneratedViewer';
import { CircularProgress } from '@mui/material';
import defaultSchema from '../util/defaultSchema';
import { generateData } from '../util/util';

const DemoElements: React.FC = () => {
  const [initialized, setInitialized] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [schema, setSchema] = useState<string>(
    JSON.stringify(defaultSchema, null, 2)
  );
  const [generated, setGenerated] = useState<string | null>(null);
  const [autoRefresh, setAutoRefresh] = useState(false);
  const [generating, setGenerating] = useState(false);
  const monaco = useMonaco();

  const initDatagen = async () => {
    try {
      await init();
      setInitialized(true);
      await generateData(schema, setGenerating, setGenerated, false);
    } catch (e) {
      setError(e.message);
    }
  };

  useEffect(() => {
    initDatagen().then();
  }, []);

  if (error) {
    return (
      <Callout type="error">Failed to initialize datagen: {error}</Callout>
    );
  }

  if (initialized && monaco) {
    return (
      <DemoGrid maxWidth="80vw">
        <GenerateButton
          schema={schema}
          setGenerated={setGenerated}
          autoRefresh={autoRefresh}
          setAutoRefresh={setAutoRefresh}
          setGenerating={setGenerating}
          disabled={generating}
        />
        <DemoGrid
          sx={{
            gridTemplateColumns: '40vw 40vw',
            '@media (max-width: 1550px)': {
              gridTemplateColumns: '100%',
            },
          }}
        >
          <SchemaEditor
            monaco={monaco}
            schema={schema}
            disabled={generating}
            setSchema={(schema) => {
              setSchema(schema);
              if (autoRefresh && !generating) {
                try {
                  return generateData(
                    JSON.parse(schema),
                    setGenerating,
                    setGenerated,
                    true
                  );
                } catch (_) {}
              }
            }}
          />
          <GeneratedViewer data={generated} />
        </DemoGrid>
      </DemoGrid>
    );
  } else {
    return <CircularProgress />;
  }
};

export default DemoElements;
