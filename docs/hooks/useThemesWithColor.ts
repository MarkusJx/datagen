import { useTheme } from 'nextra-theme-docs';
import { createDarkLightTheme } from '../util/theme';
import { Theme } from '@mui/material';

const useThemesWithColor = (dark: string, light: string): [Theme] => {
  const { theme } = useTheme();
  const getTheme = createDarkLightTheme(dark, light);

  return [getTheme(theme)];
};

export default useThemesWithColor;
