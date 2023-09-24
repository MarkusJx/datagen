import { readFileSync, rmSync, writeFileSync } from 'fs';
import * as path from 'path';

const packageJson = JSON.parse(
  readFileSync(path.join(__dirname, '..', 'package.json'), 'utf-8')
);

const version = packageJson.version;
console.log(`Updating dependency versions to ${version}`);
packageJson.devDependencies['@datagen-rs/types'] = version;

writeFileSync(
  path.join(__dirname, '..', 'package.json'),
  JSON.stringify(packageJson, null, 2)
);

console.log('Copying README.md');
const readmePath = path.join(__dirname, '..', 'README.md');
const readme = readFileSync(readmePath, 'utf-8');
rmSync(readmePath);
writeFileSync(readmePath, readme, 'utf-8');
