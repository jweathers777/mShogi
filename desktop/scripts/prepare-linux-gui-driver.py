"""Build a pinned verification-only Linux proxy; never used by product builds.

The sole upstream change disables downstream HTTP connection pooling. Hyper's
retry_canceled_requests(false) remains intact: failed requests are never replayed.
"""
import argparse
import hashlib
import io
import json
from pathlib import Path, PurePosixPath
import platform
import shutil
import subprocess
import tarfile
import urllib.request

VERSION = '2.0.6'
URL = f'https://static.crates.io/crates/tauri-driver/tauri-driver-{VERSION}.crate'
SHA256 = '24dc39bd26a65361c1c8e067636bbff1d9dd7e2fc58ff874edfba33acab0e6d0'
ANCHOR = '    .retry_canceled_requests(false)\n'
REPLACEMENT = '    .pool_max_idle_per_host(0)\n' + ANCHOR


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def patch_source(source):
    if source.count(ANCHOR) != 1 or '.pool_max_idle_per_host(' in source:
        raise ValueError('Unexpected tauri-driver source; refusing an unverified patch')
    return source.replace(ANCHOR, REPLACEMENT)


def extract(archive, output):
    if hashlib.sha256(archive).hexdigest() != SHA256:
        raise ValueError('tauri-driver archive checksum mismatch')
    with tarfile.open(fileobj=io.BytesIO(archive), mode='r:gz') as tar:
        for member in tar.getmembers():
            name = PurePosixPath(member.name)
            if (name.is_absolute() or '..' in name.parts or not name.parts
                    or name.parts[0] != f'tauri-driver-{VERSION}'
                    or not (member.isfile() or member.isdir())):
                raise ValueError(f'Unsafe archive member: {member.name}')
        tar.extractall(output)
    return output / f'tauri-driver-{VERSION}'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    if platform.system() != 'Linux':
        parser.error('This external driver correction is Linux-only')
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    with urllib.request.urlopen(URL, timeout=60) as response:
        archive = response.read()
    (output / 'upstream.crate').write_bytes(archive)
    source = extract(archive, output)
    server = source / 'src/server.rs'
    original = server.read_text()
    command = ['cargo', '+1.90.0', 'build', '--release', '--locked',
               '--manifest-path', str(source / 'Cargo.toml'),
               '--target-dir', str(output / 'target')]
    report = {'upstream_url': URL, 'upstream_version': VERSION,
              'upstream_archive_sha256': SHA256, 'source_revision': subprocess.check_output(
                  ['git', 'rev-parse', 'HEAD'], text=True).strip(),
              'original_server_sha256': digest(server), 'command': command,
              'rustc': subprocess.check_output(['rustc', '+1.90.0', '-Vv'], text=True),
              'cargo_lock_sha256': digest(source / 'Cargo.lock'),
              'policy': 'No pooled downstream connections; no request replay; upstream retry_canceled_requests(false) retained.'}
    subprocess.run(command, check=True)
    shutil.copy2(output / 'target/release/tauri-driver', output / 'tauri-driver-original')
    server.write_text(patch_source(original))
    report['patched_server_sha256'] = digest(server)
    subprocess.run(command, check=True)
    shutil.copy2(output / 'target/release/tauri-driver', output / 'tauri-driver')
    report['binaries'] = {name: digest(output / name)
                          for name in ('tauri-driver', 'tauri-driver-original')}
    (output / 'provenance.json').write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
