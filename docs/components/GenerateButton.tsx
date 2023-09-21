import React from 'react';
import { generateRandomData } from 'datagen-wasm';
import { Button } from 'nextra/components';
import DemoGrid from './DemoGrid';
import { FormControlLabel, FormGroup, Switch } from '@mui/material';

interface Props {
  setGenerated(data: string): void;
  setAutoRefresh(refresh: boolean): void;
  setGenerating(generating: boolean): void;
  disabled?: boolean;
  autoRefresh: boolean;
  schema: string;
}

const GenerateButton: React.FC<Props> = ({
  setGenerated,
  schema,
  setAutoRefresh,
  autoRefresh,
  setGenerating,
  disabled,
}) => {
  return (
    <DemoGrid sx={{ marginLeft: '60%', gridTemplateColumns: '65% 35%' }}>
      <FormGroup>
        <FormControlLabel
          disabled={disabled}
          control={
            <Switch
              inputProps={{ 'aria-label': 'controlled' }}
              checked={autoRefresh}
              onChange={(ev) => setAutoRefresh(ev.target.checked)}
            />
          }
          label="Refresh automatically"
        />
      </FormGroup>
      <Button
        disabled={disabled}
        onClick={async () => {
          try {
            setGenerating(true);
            setGenerated(await generateRandomData(JSON.parse(schema)));
          } catch (e) {
            setGenerated('Error: ' + e.message);
          } finally {
            setGenerating(false);
          }
        }}
      >
        Generate
      </Button>
    </DemoGrid>
  );
};

export default GenerateButton;
