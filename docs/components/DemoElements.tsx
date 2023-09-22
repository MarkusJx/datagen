import React, { useState } from 'react';
import { Callout } from 'nextra/components';
import { useMonaco } from '@monaco-editor/react';
import init from 'datagen-wasm';
import DemoGrid from './DemoGrid';
import GenerateButton from './GenerateButton';
import SchemaEditor from './SchemaEditor';
import GeneratedViewer from './GeneratedViewer';
import { CircularProgress, LinearProgress } from '@mui/material';
import defaultSchema from '../util/defaultSchema';
import useDemoWorker from '../hooks/useDemoWorker';
import Center from './Center';

const DemoElements: React.FC = () => {
  const [initialized, setInitialized] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [warning, setWarning] = useState<string | null>(null);
  const [schema, setSchema] = useState<string>(
    JSON.stringify(defaultSchema, null, 2)
  );
  const [generated, setGenerated] = useState<string | null>(null);
  const [autoRefresh, setAutoRefresh] = useState(false);
  const [generating, setGenerating] = useState(false);
  const [generateProgress, setGenerateProgress] = useState(100);
  const monaco = useMonaco();
  const generateData = useDemoWorker(
    async () => {
      try {
        await init();
        setInitialized(true);
        await generateData(schema, setGenerating, setGenerated, false);
      } catch (e: any) {
        console.error(e);
        setError(e.message);
      }
    },
    () => {
      setWarning(
        'Unable to load demo: Web Workers and/or WebAssembly are not supported ' +
          'in this browser. Please try a different browser.'
      );
    }
  );

  const handleGenerateProgress = (progress: number) => {
    setGenerateProgress(progress * 100);
  };

  if (warning) {
    return <Callout type="warning">{warning}</Callout>;
  } else if (error) {
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
          generateProgress={handleGenerateProgress}
          disabled={generating}
        />
        <LinearProgress
          variant="determinate"
          value={generateProgress}
          sx={{ marginTop: '2rem' }}
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
                  generateData(
                    JSON.parse(schema),
                    setGenerating,
                    setGenerated,
                    true,
                    handleGenerateProgress
                  ).catch(console.error);
                } catch (_) {}
              }
            }}
          />
          <GeneratedViewer data={generated} />
        </DemoGrid>
      </DemoGrid>
    );
  } else {
    return (
      <DemoGrid sx={{ rowGap: '2rem' }}>
        <Callout type="info">Loading demo</Callout>
        <Center>
          <CircularProgress />
        </Center>
      </DemoGrid>
    );
  }
};

export default DemoElements;
