"""Compile mShogi Linux x86-64 binaries on Google Cloud Batch (MSH-204).

Heavy compiles do not run on the owner's Mac (John, 2026-11-20). One build is
one single-VM campaign on the existing Batch/GCS path (`anchor_cloud.run`:
cost cap, owner label, terminal record, cleanup check):

  pack     git archive of one commit plus `cargo vendor` (dependency sources,
           so the VM builds offline), uploaded to the build store
  submit   one-VM Batch job with the builder image (pinned Rust compiler,
           Python, Cloud Storage client)
  worker   on the VM: verify the source hash, `cargo build --release --locked
           --offline` with `-C target-cpu=x86-64`, upload the binaries, their
           SHA-256, the compiler version and the commit
  fetch    download the binaries to a local folder for a worker image

No compile runs locally. `pack` only downloads dependency sources.

Usage:
  python3 -m tools.strength.cloud_build pack --revision REV --store URI --targets T --out MANIFEST
  python3 -m tools.strength.cloud_build submit --manifest M --config C --image I --job build-... --out DIR
  python3 -m tools.strength.cloud_build fetch --store URI --dir D
  T: comma list of package:bin:NAME or package:example:NAME
"""
import argparse
import hashlib
import io
import json
import math
import re
import subprocess
import tarfile
import tempfile
import time
from pathlib import Path

PREFIX = 'build'
CAMPAIGN_SECONDS = 3600
COMPUTE_USD = 2
TARGET = re.compile(r'(mshogi-[a-z-]+):(bin|example):([a-z0-9_-]+)')


def policy():
    from tools.strength import anchor_cloud
    return anchor_cloud.CampaignPolicy(PREFIX, CAMPAIGN_SECONDS, COMPUTE_USD, 'tools.strength.cloud_build',
                                       ('worker',))


def validate_manifest(m):
    from tools.strength.anchor_worker import require
    require(m.get('schema') == 1 and re.fullmatch(PREFIX + r'-[a-z0-9-]{1,50}', m.get('experiment', '')),
            'wrong build identity')
    for field in ('campaign_started_epoch', 'deadline_epoch', 'estimated_compute_before_usd'):
        require(type(m.get(field)) in (int, float) and math.isfinite(m[field]), 'finite ' + field)
    require(m['deadline_epoch'] == m['campaign_started_epoch'] + CAMPAIGN_SECONDS, 'fixed build deadline')
    require(0 <= m['estimated_compute_before_usd'] < COMPUTE_USD, 'compute cap reached')
    require(re.fullmatch('[a-f0-9]{40}', m.get('revision', '')) is not None, 'commit revision')
    require(re.fullmatch('[a-f0-9]{64}', m.get('source_sha256', '')) is not None, 'source hash')
    targets = m.get('targets')
    require(isinstance(targets, list) and 1 <= len(targets) <= 16
            and all(isinstance(t, str) and TARGET.fullmatch(t) for t in targets)
            and len({t.rsplit(':', 1)[1] for t in targets}) == len(targets), 'build targets')
    require(m.get('bins') == [[0]], 'one build task')


def build_paths(revision):
    """Workspace files plus every file outside `crates` that the Rust sources
    embed at compile time (`include_str!` / `include_bytes!`), at `revision`."""
    files = subprocess.run(['git', 'ls-tree', '-r', '--name-only', revision, '--', 'crates'], check=True,
                           capture_output=True, text=True).stdout.split()
    embedded = set()
    for name in files:
        if not name.endswith('.rs'):
            continue
        text = subprocess.run(['git', 'show', f'{revision}:{name}'], check=True, capture_output=True,
                              text=True).stdout
        for target in re.findall(r'include_(?:str|bytes)!\(\s*"([^"]+)"', text):
            path = (Path('/root') / name).parent.joinpath(target).resolve()
            relative = path.relative_to('/root')
            if relative.parts[0] != 'crates':
                embedded.add(str(relative))
    return ['Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml', 'crates', *sorted(embedded)]


def pack(args):
    """Upload the commit and its vendored dependencies; write the manifest."""
    from tools.experiments import gcp
    from tools.experiments.core import encode
    from tools.strength.anchor_worker import require
    revision = subprocess.run(['git', 'rev-parse', args.revision + '^{commit}'], check=True,
                              capture_output=True, text=True).stdout.strip()
    targets = args.targets.split(',')
    with tempfile.TemporaryDirectory(prefix='build-') as tmp:
        src = Path(tmp) / 'src'
        src.mkdir()
        archive = subprocess.run(['git', 'archive', revision, '--', *build_paths(revision)], check=True,
                                 capture_output=True).stdout
        subprocess.run(['tar', '-x', '-C', str(src)], input=archive, check=True)
        config = subprocess.run(['cargo', 'vendor', '--locked', '--versioned-dirs', 'vendor'], cwd=src,
                                check=True, capture_output=True, text=True).stdout
        (src / '.cargo').mkdir(exist_ok=True)
        with (src / '.cargo' / 'config.toml').open('a') as stream:
            stream.write('\n' + config)
        buffer = io.BytesIO()
        with tarfile.open(fileobj=buffer, mode='w:gz') as tar:
            tar.add(src, arcname='src')
        data = buffer.getvalue()
    store = gcp.GCSStore(args.store)
    require(store.put('source.tar.gz', data), 'build store already has a source; use a new store prefix')
    start = time.time()
    m = {'schema': 1, 'experiment': f'{PREFIX}-{revision[:12]}', 'campaign_started_epoch': start,
         'deadline_epoch': start + CAMPAIGN_SECONDS, 'estimated_compute_before_usd': 0,
         'revision': revision, 'source_sha256': hashlib.sha256(data).hexdigest(), 'targets': targets,
         'bins': [[0]]}
    validate_manifest(m)
    Path(args.out).write_bytes(encode(m))
    print(f'{len(data) / 1e6:.1f} MB source for {revision}')


def worker(manifest_uri, store_uri):
    import os
    from tools.experiments import gcp
    from tools.experiments.core import decode, digest, encode
    from tools.strength.anchor_worker import cloud_environment, put_new, require
    bucket, key = gcp.parse_uri(manifest_uri)
    raw = gcp.GCSStore('gs://' + bucket).get(key)
    m = decode(raw)
    validate_manifest(m)
    require(os.environ.get('BATCH_TASK_INDEX') == '0', 'one build task')
    require(os.environ.get('BATCH_TASK_RETRY_ATTEMPT', '0') == '0', 'no worker retries')
    store = gcp.GCSStore(store_uri)
    claim = {'manifest_sha256': digest(raw), 'task_index': 0, 'started_epoch': time.time()}
    put_new(store, 'tasks/0/started.json', encode(claim))
    receipt = dict(claim, complete=False)
    try:
        receipt['environment'] = cloud_environment()
        data = store.get('source.tar.gz')
        require(hashlib.sha256(data).hexdigest() == m['source_sha256'], 'source hash differs')
        with tempfile.TemporaryDirectory(prefix='build-') as tmp:
            tarfile.open(fileobj=io.BytesIO(data)).extractall(tmp, filter='data')
            src = Path(tmp) / 'src'
            command = ['cargo', 'build', '--release', '--locked', '--offline']
            for t in m['targets']:
                package, kind, name = TARGET.fullmatch(t).groups()
                command += ['-p', package, '--' + kind, name]
            env = dict(os.environ, RUSTFLAGS='-C target-cpu=x86-64', CARGO_TARGET_DIR=str(Path(tmp) / 'target'),
                       MSHOGI_SOURCE_REVISION=m['revision'])
            started = time.time()
            done = subprocess.run(command, cwd=src, env=env, capture_output=True, text=True)
            receipt['build_seconds'] = time.time() - started
            receipt['build_log_tail'] = done.stderr[-4000:]
            require(done.returncode == 0, 'cargo build failed')
            rustc = subprocess.run(['rustc', '--version'], check=True, capture_output=True, text=True).stdout
            outputs = {}
            for t in m['targets']:
                _, kind, name = TARGET.fullmatch(t).groups()
                path = Path(tmp) / 'target' / 'release' / ('examples/' + name if kind == 'example' else name)
                binary = path.read_bytes()
                put_new(store, 'out/' + name, binary)
                outputs[name] = hashlib.sha256(binary).hexdigest()
            put_new(store, 'out/source.txt', (m['revision'] + '\n').encode())
            put_new(store, 'out/rustc.txt', rustc.encode())
            receipt['outputs'] = outputs
            receipt['rustc'] = rustc.strip()
        receipt['complete'] = True
    except Exception as error:
        receipt['error'] = repr(error)
        raise
    finally:
        receipt['ended_epoch'] = time.time()
        put_new(store, 'tasks/0/receipt.json', encode(receipt))


def fetch(args):
    """Download the built binaries after checking them against the receipt."""
    from tools.experiments import gcp
    from tools.experiments.core import decode
    from tools.strength.anchor_worker import require
    store = gcp.GCSStore(args.store)
    receipt = decode(store.get('tasks/0/receipt.json'))
    require(receipt.get('complete') is True, 'build did not complete')
    out = Path(args.dir)
    out.mkdir(parents=True, exist_ok=False)
    for name, sha in receipt['outputs'].items():
        data = store.get('out/' + name)
        require(hashlib.sha256(data).hexdigest() == sha, 'binary hash differs: ' + name)
        (out / name).write_bytes(data)
        (out / name).chmod(0o755)
    for name in ('source.txt', 'rustc.txt'):
        (out / name).write_bytes(store.get('out/' + name))
    if args.revision:
        require((out / 'source.txt').read_text().strip() == args.revision, 'binaries are from another commit')
    (out / 'build-receipt.json').write_text(json.dumps(receipt, indent=1))
    print(json.dumps({'build_seconds': round(receipt['build_seconds']), 'outputs': receipt['outputs']}, indent=1))


def submit(args):
    from tools.experiments import gcp
    from tools.experiments.core import decode
    from tools.strength import anchor_cloud
    m = decode(Path(args.manifest).read_bytes())
    validate_manifest(m)
    gcp.validate_config(decode(Path(args.config).read_bytes()))
    anchor_cloud.run(args, policy=policy(), manifest_validator=validate_manifest)


def main():
    p = argparse.ArgumentParser(description=__doc__)
    sub = p.add_subparsers(dest='command', required=True)
    w = sub.add_parser('worker')
    w.add_argument('--manifest', required=True)
    w.add_argument('--store', required=True)
    k = sub.add_parser('pack')
    for name in ('revision', 'store', 'targets', 'out'):
        k.add_argument('--' + name, required=True)
    s = sub.add_parser('submit')
    for name in ('manifest', 'config', 'image', 'job', 'out'):
        s.add_argument('--' + name, required=True)
    s.add_argument('--subscription', default='projects/mshogi/subscriptions/mshogi-pilot-completion')
    f = sub.add_parser('fetch')
    f.add_argument('--store', required=True)
    f.add_argument('--dir', required=True)
    f.add_argument('--revision', help='expected commit of the binaries')
    a = p.parse_args()
    {'worker': lambda: worker(a.manifest, a.store), 'pack': lambda: pack(a), 'submit': lambda: submit(a),
     'fetch': lambda: fetch(a)}[a.command]()


if __name__ == '__main__':
    main()
