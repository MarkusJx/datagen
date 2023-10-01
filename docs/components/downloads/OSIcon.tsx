import React from 'react';
import { SystemType } from '../../hooks/useSystemType';
import { BsApple, BsWindows } from 'react-icons/bs';
import { FaLinux } from 'react-icons/fa';
import { useDownloadsContext } from './DownloadsContext';

const OSIcon: React.FC = () => {
  const { systemType } = useDownloadsContext();

  switch (systemType) {
    case SystemType.WindowsX64 || SystemType.WindowsX86:
      return <BsWindows />;
    case SystemType.LinuxX64:
      return <FaLinux />;
    case SystemType.AppleX64 || SystemType.AppleARM:
      return <BsApple />;
    default:
      return null;
  }
};

export default OSIcon;
