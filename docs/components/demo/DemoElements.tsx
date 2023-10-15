import React, { useEffect, useState } from 'react';
import { Callout } from 'nextra/components';
import DemoGrid from './DemoGrid';
import GenerateButton from './GenerateButton';
import SchemaEditor from './SchemaEditor';
import GeneratedViewer from './GeneratedViewer';
import { CircularProgress, LinearProgress } from '@mui/material';
import defaultSchema from '../../util/defaultSchema';
import useDemoWorker from '../../hooks/useDemoWorker';
import Center from '../Center';
import useLocalStorage from '../../hooks/useLocalStorage';
import EditorActions from './EditorActions';
import { useRouter } from 'next/router';

const DemoElements: React.FC = () => {
  const [error, setError] = useState<string | null>(null);
  const [schemaLoaded, schema, setSchema, resetSchema] = useLocalStorage(
    'schema',
    defaultSchema
  );
  const [generated, setGenerated] = useState<string>('Generating...');
  const [autoRefresh, setAutoRefresh] = useState(false);
  const [generating, setGenerating] = useState(false);
  const [generateProgress, setGenerateProgress] = useState(0);
  const {
    workerInitialized,
    workerSupported,
    workerError,
    generateRandomData,
  } = useDemoWorker();
  const router = useRouter();

  const handleGenerateProgress = (progress: number) => {
    setGenerateProgress(progress * 100);
  };

  const handleError = (e: any) => {
    console.error(e);
    setError(e.message);
  };

  useEffect(() => {
    if (schemaLoaded && workerInitialized) {
      let codeQuery = router.query['code'] as string | undefined;
      if (codeQuery) {
        codeQuery = atob(codeQuery);
        setSchema(codeQuery);
      }

      generateRandomData(
        codeQuery ?? schema,
        setGenerating,
        setGenerated,
        false,
        handleGenerateProgress
      ).catch(handleError);
    }
  }, [schemaLoaded, workerInitialized]);

  const handleUpdateSchema = (schema: string) => {
    setSchema(schema);
    if (autoRefresh && !generating) {
      try {
        generateRandomData(
          JSON.parse(schema),
          setGenerating,
          setGenerated,
          true,
          handleGenerateProgress
        ).catch(handleError);
      } catch (_) {}
    }
  };

  if (!workerSupported) {
    return (
      <Callout type="warning">
        Unable to load demo: Web Workers and/or WebAssembly are not supported in
        this browser. Please try a different browser.
      </Callout>
    );
  } else if (error ?? workerError) {
    return (
      <Callout type="error">
        Failed to initialize datagen: {error ?? workerError}
      </Callout>
    );
  } else if (workerInitialized && schemaLoaded) {
    return (
      <DemoGrid maxWidth="80vw" sx={{ paddingTop: '1.5rem' }}>
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
          sx={{ marginTop: '1.5rem' }}
        />
        <DemoGrid
          sx={{
            gridTemplateColumns: '40vw 40vw',
            '@media (max-width: 1550px)': {
              gridTemplateColumns: '100%',
            },
          }}
        >
          <SchemaEditor schema={schema} setSchema={handleUpdateSchema} />
          <GeneratedViewer data={generated} schema={schema} />
        </DemoGrid>
        <EditorActions
          resetSchema={resetSchema}
          schema={schema}
          generated={generated}
          disabled={generating}
        />
      </DemoGrid>
    );
  } else {
    return (
      <DemoGrid sx={{ rowGap: '1.5rem' }}>
        <Callout type="info">Loading demo</Callout>
        <Center>
          <CircularProgress />
        </Center>
      </DemoGrid>
    );
  }
};

export default DemoElements;
