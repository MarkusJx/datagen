import { readFileSync, writeFileSync } from 'fs';
import * as path from 'path';

const packageJson = JSON.parse(readFileSync(
  path.join(__dirname, '..', 'package.json'),
  'utf-8'
));

const version = packageJson.version;
packageJson.devDependencies['@datagen-rs/types'] = version;
packageJson.dependencies['@datagen-rs/node'] = version;

writeFileSync(
  path.join(__dirname, '..', 'package.json'),
  JSON.stringify(packageJson, null, 2)
);