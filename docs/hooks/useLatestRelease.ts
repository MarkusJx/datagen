import { useEffect } from 'react';
import { Octokit } from '@octokit/rest';
import { SystemType } from './useSystemType';

export interface Artifact {
  name: string;
  downloadUrl: string;
}

export interface LatestRelease {
  version: string;
  artifacts: Record<SystemType, Artifact[]>;
}

const triplets = [
  'x86_64-unknown-linux-gnu',
  'x86_64-pc-windows-msvc',
  'i686-pc-windows-msvc',
  'x86_64-apple-darwin',
  'aarch64-apple-darwin',
];

const getArtifactSystemType = (name: string): SystemType => {
  if (name.includes('x86_64-unknown-linux-gnu')) {
    return SystemType.LinuxX64;
  } else if (name.includes('x86_64-pc-windows-msvc')) {
    return SystemType.WindowsX64;
  } else if (name.includes('i686-pc-windows-msvc')) {
    return SystemType.WindowsX86;
  } else if (name.includes('x86_64-apple-darwin')) {
    return SystemType.AppleX64;
  } else if (name.includes('aarch64-apple-darwin')) {
    return SystemType.AppleARM;
  } else {
    return SystemType.Unsupported;
  }
};

const getArtifactName = (name: string): string => {
  const triplet = triplets.find((t) => name.includes(t));

  if (!triplet) {
    return name;
  }

  return name.replace(`-${triplet}`, '').split('.')[0] ?? 'unknown';
};

const useLatestRelease = (
  setLatestRelease: (release: LatestRelease | null) => void,
  setError: (error: string | null) => void
): void => {
  useEffect(() => {
    const octokit = new Octokit();

    octokit.repos
      .getLatestRelease({
        owner: 'MarkusJx',
        repo: 'datagen',
      })
      .then((response) => {
        const artifacts = response.data.assets
          .map((asset) => ({
            name: asset.name,
            downloadUrl: asset.browser_download_url,
          }))
          .reduce(
            (prev, cur) => ({
              ...prev,
              [getArtifactSystemType(cur.name)]: [
                ...(prev[getArtifactSystemType(cur.name)] || []),
                {
                  ...cur,
                  name: getArtifactName(cur.name),
                },
              ],
            }),
            {} as Record<SystemType, Artifact[]>
          );

        setLatestRelease({
          version: response.data.tag_name,
          artifacts,
        });
      })
      .catch((e) => setError(e.toString()));
  }, []);
};

export default useLatestRelease;
