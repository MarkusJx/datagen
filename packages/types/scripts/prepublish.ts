import * as path from 'path';
import { readFileSync, rmSync, writeFileSync } from 'fs';

console.log('Copying README.md');
const readmePath = path.join(__dirname, '..', 'README.md');
const readme = readFileSync(readmePath, 'utf-8');
rmSync(readmePath);
writeFileSync(readmePath, readme, 'utf-8');
