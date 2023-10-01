import React from 'react';
import DemoGrid from './DemoGrid';
import {
  FormControlLabel,
  FormGroup,
  Switch,
  ThemeProvider,
  Tooltip,
  Typography,
} from '@mui/material';
import useDemoWorker from '../../hooks/useDemoWorker';
import { LoadingButton } from '@mui/lab';
import { createThemeWithColor } from '../../util/theme';

interface Props {
  setGenerated(data: string): void;
  setAutoRefresh(refresh: boolean): void;
  setGenerating(generating: boolean): void;
  generateProgress(progress: number): void;
  disabled?: boolean;
  autoRefresh: boolean;
  schema: string;
}

const theme = createThemeWithColor('#0081ff');

const GenerateButton: React.FC<Props> = ({
  setGenerated,
  schema,
  setAutoRefresh,
  autoRefresh,
  setGenerating,
  generateProgress,
  disabled,
}) => {
  const { generateRandomData } = useDemoWorker();

  return (
    <ThemeProvider theme={theme}>
      <DemoGrid
        sx={{
          width: '40%',
          gridTemplateColumns: 'auto max-content',
          margin: '0 auto',
        }}
      >
        <FormGroup>
          <FormControlLabel
            disabled={disabled}
            sx={{
              userSelect: 'none',
            }}
            control={
              <Switch
                inputProps={{ 'aria-label': 'controlled' }}
                checked={autoRefresh}
                onChange={(ev) => setAutoRefresh(ev.target.checked)}
              />
            }
            label={
              <Tooltip title="Automatically re-generate the data if the schema is a valid JSON document">
                <Typography>Refresh automatically</Typography>
              </Tooltip>
            }
          />
        </FormGroup>
        <LoadingButton
          loading={disabled}
          variant="outlined"
          onClick={() =>
            generateRandomData(
              schema,
              setGenerating,
              setGenerated,
              false,
              generateProgress
            )
          }
        >
          Generate
        </LoadingButton>
      </DemoGrid>
    </ThemeProvider>
  );
};

export default GenerateButton;
