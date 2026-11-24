"""Bounded Chu harness measurements on the shared Batch transport.

Each manifest freezes input hashes, executable sources and one list of commands
per VM. No search or game rules live here. Native reports retain all outcomes.
"""
import argparse
import gzip
import hashlib
import json
import os
import signal
import subprocess
import tempfile
import time
from pathlib import Path

from tools.experiments import gcp
from tools.experiments.core import encode
from tools.strength import anchor_cloud

SECONDS = 1800
POLICY = anchor_cloud.CampaignPolicy('msh213', SECONDS, 10, 'tools.strength.chu_cloud', ('worker',))


def validate(m):
    assert m['schema'] == 1
    assert m['deadline_epoch'] == m['campaign_started_epoch'] + SECONDS
    assert 0 <= m['estimated_compute_before_usd'] < 10
    assert 1 <= len(m['bins']) <= 8
    assert all(1 <= len(b) <= 16 for b in m['bins'])
    for name, item in m['files'].items():
        assert Path(name).name == name and name not in ('.', '..')
        gcp.parse_uri(item['uri'])
        assert len(item['sha256']) == 64
    for commands in m['bins']:
        for command in commands:
            assert isinstance(command, list) and all(isinstance(v, str) for v in command)
            assert command[0] in ('helper', 'run', 'match', 'export', 'signature')
            if command[0] == 'run':
                assert len(command) >= 2 and command[1] in m['files']


def fetch(item):
    bucket, key = gcp.parse_uri(item['uri'])
    raw = gcp.GCSStore('gs://' + bucket).get(key)
    assert hashlib.sha256(raw).hexdigest() == item['sha256']
    return raw


def worker(manifest_uri, store_uri):
    bucket, key = gcp.parse_uri(manifest_uri)
    m = json.loads(gcp.GCSStore('gs://' + bucket).get(key))
    validate(m)
    index = int(os.environ['BATCH_TASK_INDEX'])
    assert os.environ.get('BATCH_TASK_RETRY_ATTEMPT', '0') == '0'
    store = gcp.GCSStore(store_uri)
    prefix = f'tasks/{index}/'
    receipt = {'complete': False, 'started': time.time(), 'commands': []}
    assert store.put(prefix + 'started.json', encode(receipt))
    try:
        with tempfile.TemporaryDirectory(prefix='chu-') as tmp:
            root = Path(tmp)
            for name, item in m['files'].items():
                (root / name).write_bytes(fetch(item))
                if name in ('engine', 'baseline', 'runner', 'helper', 'writer'):
                    (root / name).chmod(0o755)
            env = dict(os.environ)
            env['PYTHONPATH'] = '/opt/mshogi'
            for n, command in enumerate(m['bins'][index]):
                if command[0] in ('helper', 'run'):
                    name, arguments = ('helper', command[1:]) if command[0] == 'helper' else (command[1], command[2:])
                    (root / name).chmod(0o755)
                    actual = [str(root / name), *arguments]
                elif command[0] == 'signature':
                    actual = ['python3', '-m', 'tools.strength.native_signature', str(root / 'engine'), str(root / 'baseline'), 'openings.json', 'suite.json', command[1]]
                elif command[0] == 'match':
                    actual = ['python3', '-m', 'tools.strength.native_match', '--game', 'chu',
                              '--candidate', str(root / 'engine'), '--baseline', str(root / 'baseline'),
                              '--runner', str(root / 'runner'), '--config-writer', str(root / 'writer'),
                              '--workers', '1', *command[1:]]
                else:
                    # Export only reports replayed by the native reader. The
                    # shared splitter keeps both colors of each opening together.
                    reports = []
                    for path in sorted(root.glob(command[1] + '/*.report.json.gz')):
                        raw = gzip.decompress(path.read_bytes())
                        target = path.with_suffix('')
                        target.write_bytes(raw)
                        reports.append(str(target))
                    (root / 'reports.json').write_text(json.dumps(reports))
                    (root / 'texel').mkdir(exist_ok=True)
                    actual = [str(root / 'helper'), 'export', 'reports.json', 'texel/games-0.jsonl', '4']
                remaining = m['deadline_epoch'] - time.time() - 90
                assert remaining > 0, 'campaign deadline'
                with (root / f'command-{n}.log').open('w') as log:
                    process = subprocess.Popen(actual, cwd=root, env=env, stdout=log,
                                               stderr=subprocess.STDOUT, start_new_session=True)
                    try:
                        code = process.wait(timeout=remaining)
                    except subprocess.TimeoutExpired:
                        os.killpg(process.pid, signal.SIGKILL)
                        process.wait()
                        code = -signal.SIGKILL
                receipt['commands'].append({'argv': command, 'exit': code})
                if command[0] == 'export' and code == 0:
                    from tools.strength.texel_data import split
                    split(root / 'texel', root / 'texel/manifest.json', game='chu', by_opening=True, allow_empty=True)
                # Preserve failed commands and every unfinished pair for diagnosis.
                for path in sorted(root.rglob('*')):
                    if path.is_file() and path.name not in m['files']:
                        key = prefix + str(path.relative_to(root))
                        store.put(key, path.read_bytes())
                assert code == 0, f'command {n} failed'
            receipt['complete'] = True
    except Exception as error:
        receipt['error'] = repr(error)
        raise
    finally:
        receipt['ended'] = time.time()
        assert store.put(prefix + 'receipt.json', encode(receipt))


def main():
    p = argparse.ArgumentParser(description=__doc__)
    sub = p.add_subparsers(dest='command', required=True)
    submit = sub.add_parser('submit')
    for key in ('manifest', 'config', 'image', 'job', 'out'):
        submit.add_argument('--' + key, required=True)
    submit.add_argument('--subscription', default='projects/mshogi/subscriptions/mshogi-pilot-completion')
    work = sub.add_parser('worker')
    work.add_argument('--manifest', required=True)
    work.add_argument('--store', required=True)
    a = p.parse_args()
    if a.command == 'submit':
        anchor_cloud.run(a, policy=POLICY, manifest_validator=validate)
    else:
        worker(a.manifest, a.store)


if __name__ == '__main__':
    main()
