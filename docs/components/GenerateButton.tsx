import React from 'react';
import DemoGrid from './DemoGrid';
import {
  createTheme,
  FormControlLabel,
  FormGroup,
  Switch,
  ThemeProvider,
  Tooltip,
  Typography,
} from '@mui/material';
import useDemoWorker from '../hooks/useDemoWorker';
import { LoadingButton } from '@mui/lab';

interface Props {
  setGenerated(data: string): void;
  setAutoRefresh(refresh: boolean): void;
  setGenerating(generating: boolean): void;
  generateProgress(progress: number): void;
  disabled?: boolean;
  autoRefresh: boolean;
  schema: string;
}

const theme = createTheme({
  palette: {
    action: {
      disabledBackground: '#535353',
      disabled: '#535353',
    },
    text: {
      disabled: '#535353',
      primary: '#0081ff',
    },
    primary: {
      main: '#0081ff',
    },
  },
});

const GenerateButton: React.FC<Props> = ({
  setGenerated,
  schema,
  setAutoRefresh,
  autoRefresh,
  setGenerating,
  generateProgress,
  disabled,
}) => {
  const generateData = useDemoWorker();

  return (
    <ThemeProvider theme={theme}>
      <DemoGrid
        sx={{ width: '40%', gridTemplateColumns: '65% 35%', margin: '0 auto' }}
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
            generateData(
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
