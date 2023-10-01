import React, { useEffect, useState } from 'react';
import useSystemType, { SystemType } from '../../hooks/useSystemType';
import useLatestRelease from '../../hooks/useLatestRelease';
import { CircularProgress, ThemeProvider } from '@mui/material';
import useThemesWithColor from '../../hooks/useThemesWithColor';
import { blue } from '@mui/material/colors';
import Center from '../Center';
import { Callout } from 'nextra/components';
import StyledA from '../styled/StyledA';
import { useDownloadsContext } from './DownloadsContext';

interface Props {
  children: React.ReactNode;
}

const Downloads: React.FC<Props> = ({ children }) => {
  const {
    systemType,
    setSystemType,
    latestRelease,
    setLatestRelease,
    setOsArtifacts,
  } = useDownloadsContext();
  const [error, setError] = useState<string | null>(null);
  const [theme] = useThemesWithColor(blue['200'], blue['600']);
  useSystemType(setSystemType);
  useLatestRelease(setLatestRelease, setError);

  useEffect(() => {
    if (latestRelease && systemType !== SystemType.Unknown) {
      setOsArtifacts(
        latestRelease.artifacts[systemType]?.reduce(
          (prev, cur) => ({
            ...prev,
            [cur.name]: cur.downloadUrl,
          }),
          {} as Record<string, string>
        ) ?? null
      );
    }
  }, [latestRelease, systemType]);

  if (systemType === SystemType.Unsupported) {
    return (
      <Callout type="warning">
        Your system is not supported by datagen. If you think this is a mistake,
        please{' '}
        <StyledA
          href="https://github.com/MarkusJx/datagen/issues"
          sx={{
            textDecoration: 'underline',
          }}
        >
          create a new issue on GitHub
        </StyledA>
        .
      </Callout>
    );
  } else if (latestRelease) {
    return <ThemeProvider theme={theme}>{children}</ThemeProvider>;
  } else if (error) {
    return <Callout type="error">Failed to fetch latest release</Callout>;
  } else {
    return (
      <>
        <Callout type="info">Fetching latest release...</Callout>
        <Center
          sx={{
            marginTop: '20vh',
          }}
        >
          <CircularProgress />
        </Center>
      </>
    );
  }
};

export default Downloads;
