import fs from 'fs';
import path from 'path';

const srcPath = path.join('backends', 'wae-https', 'src', 'lib.rs.temp');
const dstPath = path.join('backends', 'wae-https', 'src', 'lib.rs');

console.log(`Copying ${srcPath} to ${dstPath}...`);

try {
  const content = fs.readFileSync(srcPath, 'utf8');
  fs.writeFileSync(dstPath, content, 'utf8');
  console.log('Success!');
} catch (err) {
  console.error('Error:', err);
  process.exit(1);
}
