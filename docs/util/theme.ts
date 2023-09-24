import { createTheme, ThemeOptions } from '@mui/material';
import { deepmerge } from '@mui/utils';

export const disabledThemeOptions = {
  palette: {
    action: {
      disabledBackground: '#535353',
      disabled: '#535353',
    },
    text: {
      disabled: '#535353',
    },
  },
};

export const createThemeWithDisabled = (options: ThemeOptions) =>
  createTheme(deepmerge(options, disabledThemeOptions));
export const createThemeWithColor = (color: string) =>
  createThemeWithDisabled({
    palette: {
      text: {
        primary: color,
      },
      primary: {
        main: color,
      },
    },
  });
