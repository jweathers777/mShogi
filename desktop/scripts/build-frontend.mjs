import { spawnSync } from 'node:child_process';
import { readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { build } from 'vite';

const desktop = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const args = process.argv.slice(2);
if (args.some((arg) => arg !== '--verification') || args.length > 1) {
  throw new Error('Usage: node scripts/build-frontend.mjs [--verification]');
}
const verification = args.includes('--verification');
// Ordinary builds override inherited flags; only this explicit argument opts in.
process.env.MSHOGI_FRONTEND_VERIFICATION = verification ? '1' : '0';
const checked = spawnSync(process.execPath, [path.join(desktop, 'node_modules/typescript/bin/tsc'), '--noEmit'], {
  cwd: desktop, stdio: 'inherit', env: process.env,
});
if (checked.error) throw checked.error;
if (checked.status !== 0) process.exit(checked.status ?? 1);

const outDir = verification ? 'dist-verification' : 'dist';
const forbidden = ['mshogi.synthetic.strider16.v1', 'synthetic_snapshot', 'synthetic_command', 'desktop_verification_mode', 'SYNTHETIC DEVELOPMENT FIXTURE'];
await build({
  root: desktop,
  build: { outDir, emptyOutDir: true },
  plugins: verification ? [] : [{
    name: 'exclude-verification-modules',
    generateBundle(_options, bundle) {
      for (const output of Object.values(bundle)) {
        if (output.type === 'chunk' && Object.keys(output.modules).some((id) => id.split(path.sep).join('/').includes('/src/verification/'))) {
          throw new Error(`Verification module entered normal frontend: ${output.fileName}`);
        }
      }
    },
  }],
});
if (!verification) {
  function inspect(directory) {
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      const filename = path.join(directory, entry.name);
      if (entry.isDirectory()) inspect(filename);
      else {
        const bytes = readFileSync(filename);
        for (const marker of forbidden) {
          if (bytes.includes(Buffer.from(marker))) throw new Error(`Verification marker entered normal frontend: ${marker} (${filename})`);
        }
      }
    }
  }
  inspect(path.join(desktop, outDir));
  console.log('Normal frontend excludes verification modules and markers.');
}
