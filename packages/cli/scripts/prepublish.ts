import { readFileSync, writeFileSync, rmSync } from 'fs';

const packageJsonPath = new URL('../package.json', import.meta.url);
const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf-8'));

const version = packageJson.version;
console.log(`Updating dependency versions to ${version}`);
packageJson.devDependencies['datagen-rs-types'] = version;
packageJson.dependencies['datagen-rs-node'] = version;

writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2));

console.log('Copying README.md');
const readmePath = new URL('../README.md', import.meta.url);
const readme = readFileSync(readmePath, 'utf-8');
rmSync(readmePath);
writeFileSync(readmePath, readme, 'utf-8');
