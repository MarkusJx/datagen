import * as path from 'path';
import { glob } from 'glob';
import { readFile } from 'fs/promises';
import * as toml from 'toml';
import chalk from 'chalk';

interface PackageVersion {
  path: string;
  version: string;
}

async function run() {
  const cwd = path.join(__dirname, '..', '..', '..');

  const packageJsons = await glob('**/package.json', {
    cwd,
    ignore: ['**/node_modules/**', '**/dist/**'],
    follow: true,
  });

  const nodeVersions: PackageVersion[] = await Promise.all(
    packageJsons.map(async (p) => ({
      path: p,
      version: await readFile(path.join(cwd, p), 'utf-8').then(
        (m) => JSON.parse(m).version
      ),
    }))
  );

  const cargoTomls = await glob('**/Cargo.toml', {
    cwd,
    ignore: [
      '**/node_modules/**',
      '**/dist/**',
      '**/target/**',
      'Cargo.toml',
      'plugins/log-plugin/**',
    ],
    follow: true,
  });

  const rustVersions: PackageVersion[] = await Promise.all(
    cargoTomls.map(async (p) => ({
      path: p,
      version: await readFile(path.join(cwd, p), 'utf-8').then(
        (m) => toml.parse(m).package.version
      ),
    }))
  );

  const sorted = Object.entries(
    nodeVersions.concat(rustVersions).reduce(
      (prev, cur) => ({
        ...prev,
        [cur.version]: [...(prev[cur.version] || []), cur.path],
      }),
      {} as Record<string, string[]>
    )
  ).sort(([_1, a], [_2, b]) => a.length - b.length);

  console.log(
    `Found ${chalk.gray(nodeVersions.length)} node.js packages and ${chalk.gray(
      rustVersions.length
    )} rust crates`
  );
  if (sorted.length > 1) {
    const [mostMatch, mostPackages] = sorted.pop();

    throw new Error(
      `${chalk.redBright('Versions mismatched')}:
Most packages (${chalk.gray(
        mostPackages.length
      )}) are using ${chalk.greenBright(mostMatch)},
but ${sorted
        .map(
          ([v, p]) =>
            `${p.map((e) => chalk.gray(e)).join(', ')} are using ${chalk.cyan(
              v
            )}`
        )
        .join(' and \n')}
      `
    );
  }

  console.log(chalk.greenBright('[✔] All versions match'));
}

run().catch((e) => {
  console.error(e);
  process.exit(1);
});
