import React from 'react';
import PluginDownload from './PluginDownload';
import { useDownloadsContext } from './DownloadsContext';

interface Props {
  children: React.ReactNode;
}

const PluginsDownloads: React.FC<Props> = ({ children }) => {
  const { osArtifacts } = useDownloadsContext();

  return (
    <div>
      {children}
      {osArtifacts &&
        Object.entries(osArtifacts)
          .filter(([name]) => name !== 'datagen')
          .map(([name, url]) => (
            <PluginDownload
              name={name.replace('_', '-')}
              url={url}
              key={name}
            />
          ))}
    </div>
  );
};

export default PluginsDownloads;
