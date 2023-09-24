import { readFileSync, writeFileSync } from 'fs';
import * as path from 'path';

const packageJson = JSON.parse(readFileSync(
  path.join(__dirname, '..', 'package.json'),
  'utf-8'
));

const version = packageJson.version;
packageJson.devDependencies['@datagen-rs/types'] = version;

writeFileSync(
  path.join(__dirname, '..', 'package.json'),
  JSON.stringify(packageJson, null, 2)
);
