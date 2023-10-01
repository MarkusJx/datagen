import React, { createContext, useContext, useState } from 'react';
import { LatestRelease } from '../../hooks/useLatestRelease';
import { SystemType } from '../../hooks/useSystemType';

interface Props {
  children: React.ReactNode;
}

export interface DownloadsContext {
  latestRelease: LatestRelease | null;
  systemType: SystemType;
  osArtifacts: Record<string, string> | null;
  setLatestRelease: (latest: LatestRelease | null) => void;
  setSystemType: (systemType: SystemType) => void;
  setOsArtifacts: (osArtifacts: Record<string, string> | null) => void;
}

const Context = createContext<DownloadsContext>({
  latestRelease: null,
  systemType: SystemType.Unknown,
  osArtifacts: null,
  setLatestRelease: () => {},
  setSystemType: () => {},
  setOsArtifacts: () => {},
});

export const useDownloadsContext = () => {
  return useContext(Context);
};

const DownloadsContext: React.FC<Props> = ({ children }) => {
  const [systemType, setSystemType] = useState<SystemType>(SystemType.Unknown);
  const [latestRelease, setLatestRelease] = useState<LatestRelease | null>(
    null
  );
  const [osArtifacts, setOsArtifacts] = useState<Record<string, string> | null>(
    null
  );

  return (
    <Context.Provider
      value={{
        systemType,
        latestRelease,
        osArtifacts,
        setSystemType,
        setLatestRelease,
        setOsArtifacts,
      }}
    >
      {children}
    </Context.Provider>
  );
};

export default DownloadsContext;
