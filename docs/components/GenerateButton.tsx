import React from 'react';
import { generateRandomData } from 'datagen-wasm';
import { Button } from 'nextra/components';
import DemoGrid from './DemoGrid';
import { FormControlLabel, FormGroup, Switch } from '@mui/material';
import { generateData } from '../util/util';

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
    <DemoGrid
      sx={{ width: '40%', gridTemplateColumns: '65% 35%', margin: '0 auto' }}
    >
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
        onClick={() => generateData(schema, setGenerating, setGenerated, false)}
      >
        Generate
      </Button>
    </DemoGrid>
  );
};

export default GenerateButton;
