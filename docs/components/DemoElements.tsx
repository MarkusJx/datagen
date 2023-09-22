import React, { useEffect, useState } from 'react';
import { Callout } from 'nextra/components';
import { useMonaco } from '@monaco-editor/react';
import DemoGrid from './DemoGrid';
import GenerateButton from './GenerateButton';
import SchemaEditor from './SchemaEditor';
import GeneratedViewer from './GeneratedViewer';
import {
  Button,
  CircularProgress,
  Grid,
  IconButton,
  LinearProgress,
  Tooltip,
} from '@mui/material';
import DownloadIcon from '@mui/icons-material/Download';
import defaultSchema from '../util/defaultSchema';
import useDemoWorker from '../hooks/useDemoWorker';
import Center from './Center';
import useLocalStorage from '../hooks/useLocalStorage';
import { downloadFile, getSchemaFormat } from '../util/util';

const DemoElements: React.FC = () => {
  const [error, setError] = useState<string | null>(null);
  const [warning, setWarning] = useState<string | null>(null);
  const [schemaLoaded, schema, setSchema, resetSchema] = useLocalStorage(
    'schema',
    JSON.stringify(defaultSchema, null, 2)
  );
  const [generated, setGenerated] = useState<string | null>(null);
  const [autoRefresh, setAutoRefresh] = useState(false);
  const [generating, setGenerating] = useState(false);
  const [generateProgress, setGenerateProgress] = useState(0);
  const monaco = useMonaco();
  const { workerInitialized, generateRandomData } = useDemoWorker(() => {
    setWarning(
      'Unable to load demo: Web Workers and/or WebAssembly are not supported ' +
        'in this browser. Please try a different browser.'
    );
  });

  const handleGenerateProgress = (progress: number) => {
    setGenerateProgress(progress * 100);
  };

  useEffect(() => {
    if (schemaLoaded && workerInitialized) {
      generateRandomData(
        schema,
        setGenerating,
        setGenerated,
        false,
        handleGenerateProgress
      ).catch((e) => {
        console.error(e);
        setError(e.message);
      });
    }
  }, [schemaLoaded, workerInitialized]);

  const handleDownload = () => {
    if (generated) {
      const format = getSchemaFormat(schema);
      downloadFile(generated, 'generated', format);
    }
  };

  if (warning) {
    return <Callout type="warning">{warning}</Callout>;
  } else if (error) {
    return (
      <Callout type="error">Failed to initialize datagen: {error}</Callout>
    );
  }

  if (workerInitialized && schemaLoaded && monaco) {
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
                  generateRandomData(
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
          <GeneratedViewer data={generated} schema={schema} />
        </DemoGrid>
        <Grid
          container
          gap={2}
          justifyContent="right"
          sx={{ marginTop: '2rem' }}
        >
          <Button onClick={resetSchema}>Reset schema</Button>
          <Tooltip title="Download the generated data">
            <IconButton
              aria-label="download"
              size="medium"
              color="primary"
              onClick={handleDownload}
            >
              <DownloadIcon fontSize="inherit" />
            </IconButton>
          </Tooltip>
        </Grid>
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
