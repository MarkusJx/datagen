import React from 'react';
import {
  Button,
  Grid,
  IconButton,
  ThemeProvider,
  Tooltip,
} from '@mui/material';
import DownloadIcon from '@mui/icons-material/Download';
import { downloadFile, getSchemaFormat } from '../util/util';
import { useTheme } from 'nextra-theme-docs';
import { createThemeWithColor } from '../util/theme';

interface Props {
  resetSchema(): void;
  generated: string;
  schema: string;
  disabled?: boolean;
}

const lightTheme = createThemeWithColor('#333333');
const darkTheme = createThemeWithColor('#d2d2d2');

const EditorActions: React.FC<Props> = ({
  resetSchema,
  generated,
  schema,
  disabled,
}) => {
  const { theme } = useTheme();

  const handleDownload = () => {
    const format = getSchemaFormat(schema);
    downloadFile(generated, 'generated', format);
  };

  return (
    <ThemeProvider theme={theme === 'dark' ? darkTheme : lightTheme}>
      <Grid container gap={2} justifyContent="right" sx={{ marginTop: '1rem' }}>
        <Button onClick={resetSchema} disabled={disabled}>
          Reset schema
        </Button>
        <Tooltip title="Download the generated data">
          <IconButton
            aria-label="download"
            size="medium"
            color="primary"
            onClick={handleDownload}
            disabled={disabled}
          >
            <DownloadIcon fontSize="inherit" />
          </IconButton>
        </Tooltip>
      </Grid>
    </ThemeProvider>
  );
};

export default EditorActions;
