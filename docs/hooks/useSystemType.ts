import UAParser from 'ua-parser-js';
import arch from 'arch';
import { useEffect } from 'react';

export enum SystemType {
  WindowsX86 = 'Windows 10 or newer 32-bit',
  WindowsX64 = 'Windows 10 or newer 64-bit',
  LinuxX64 = 'Linux 64-bit',
  AppleX64 = 'MacOS 12 or newer 64-bit (Intel)',
  AppleARM = 'MacOS 12 or newer 64-bit (Apple Silicon)',
  Unsupported = 'Unsupported OS',
  Unknown = 'Unknown',
}

const isMacArm = (): boolean => {
  const webgl = document.createElement('canvas').getContext('webgl');
  const debugRendererInfo = webgl?.getExtension('WEBGL_debug_renderer_info');
  const unmaskedRenderer =
    (debugRendererInfo &&
      webgl?.getParameter(debugRendererInfo.UNMASKED_RENDERER_WEBGL)) ||
    '';

  if (unmaskedRenderer.match(/Apple/) && !unmaskedRenderer.match(/Apple GPU/)) {
    return true;
  } else
    return !!(
      unmaskedRenderer.match(/Apple GPU/) &&
      unmaskedRenderer
        .getSupportedExtensions()
        .indexOf('WEBGL_compressed_texture_s3tc_srgb') == -1
    );
};

const getSystemType = (): SystemType => {
  const parser = new UAParser();
  const osVersion = parser.getOS().version;
  const osName = parser.getOS().name;
  const cpuArch = parser.getCPU().architecture;

  if (osName === 'Windows') {
    if (
      (osVersion && Number(osVersion.split('.')[0]) < 10) ||
      (cpuArch && cpuArch !== 'amd64')
    ) {
      return SystemType.Unsupported;
    }

    if (arch() === 'x64') {
      return SystemType.WindowsX64;
    } else {
      return SystemType.WindowsX86;
    }
  } else if (osName === 'Ubuntu' || osName === 'Debian') {
    if (arch() === 'x64') {
      return SystemType.LinuxX64;
    } else {
      return SystemType.Unsupported;
    }
  } else if (osName === 'macOS') {
    if (osVersion && Number(osVersion.split('.')[0]) < 12) {
      return SystemType.Unsupported;
    }

    if (isMacArm()) {
      return SystemType.AppleARM;
    } else if (arch() === 'x64') {
      return SystemType.AppleX64;
    }
  }

  return SystemType.Unsupported;
};

const useSystemType = (setSystemType: (type: SystemType) => void): void => {
  useEffect(() => {
    setSystemType(getSystemType());
  }, []);
};

export default useSystemType;
