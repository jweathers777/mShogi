import { execFileSync } from 'node:child_process';
import { copyFileSync, chmodSync, mkdirSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

const desktop = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const root = path.resolve(desktop, '..');
const host = execFileSync('rustc', ['--print', 'host-tuple'], { cwd: root, encoding: 'utf8' }).trim();
execFileSync('cargo', ['build', '--locked', '--release', '--bin', 'mshogi'], { cwd: root, stdio: 'inherit' });
const suffix = process.platform === 'win32' ? '.exe' : '';
const source = path.join(root, 'target', 'release', `mshogi${suffix}`);
const binaries = path.join(desktop, 'src-tauri', 'binaries');
mkdirSync(binaries, { recursive: true });
copyFileSync(source, path.join(binaries, `mshogi-${host}${suffix}`));
// Development and cargo-run hosts use the same sibling lookup as a packaged app.
for (const profile of ['debug', 'release']) {
 const target = path.join(desktop, 'src-tauri', 'target', profile);
 mkdirSync(target, { recursive: true });
 const destination = path.join(target, `mshogi${suffix}`);
 copyFileSync(source, destination);
 if (process.platform !== 'win32') chmodSync(destination, 0o755);
}
console.log(`Prepared bundled engine for ${host}`);
