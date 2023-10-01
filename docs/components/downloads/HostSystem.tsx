import React from 'react';
import CodeBlock from '../CodeBlock';
import { useDownloadsContext } from './DownloadsContext';

interface Props {
  children: React.ReactNode;
}

const HostSystem: React.FC<Props> = ({ children }) => {
  const { systemType } = useDownloadsContext();

  return <CodeBlock value={systemType}>{children}</CodeBlock>;
};

export default HostSystem;
