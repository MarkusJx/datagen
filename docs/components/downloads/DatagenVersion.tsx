import React from 'react';
import CodeBlock from '../CodeBlock';
import { useDownloadsContext } from './DownloadsContext';

interface Props {
  children: React.ReactNode;
}

const DatagenVersion: React.FC<Props> = ({ children }) => {
  const { latestRelease } = useDownloadsContext();

  return <CodeBlock value={latestRelease?.version}>{children}</CodeBlock>;
};

export default DatagenVersion;
