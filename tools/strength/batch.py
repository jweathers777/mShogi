"""Run a frozen set of strength CLI commands through cost-bounded Cloud Batch."""
import math
import os
import re
import signal
import subprocess
import sys
import tempfile
import time
from pathlib import Path

from tools.experiments import gcp
from tools.experiments.core import digest
from . import anchor_cloud as controller

SECONDS = 1800
POLICY = controller.CampaignPolicy('strength', SECONDS, 10, 'tools.strength.cli', ('cloud-worker',))
OPERATIONS = {'openings', 'suite', 'probe', 'match', 'export', 'judge', 'signature', 'texel', 'selfplay', 'tenjiku-work', 'bench', 'blunders'}


def validate(m):
    require = controller.require
    require(m.get('schema') == 1, 'expected strength manifest schema 1')
    for field in ('deadline_epoch', 'campaign_started_epoch', 'estimated_compute_before_usd'):
        require(type(m.get(field)) in (int, float) and math.isfinite(m[field]), 'invalid ' + field)
    require(m['deadline_epoch'] == m['campaign_started_epoch'] + SECONDS, 'wrong absolute deadline')
    require(0 <= m['estimated_compute_before_usd'] < POLICY.compute_usd, 'compute cap reached')
    require(isinstance(m.get('files'), dict), 'expected input files')
    for name, item in m['files'].items():
        require(re.fullmatch('[a-zA-Z0-9][a-zA-Z0-9._-]*', name), 'invalid file name')
        gcp.parse_uri(item['uri'])
        require(re.fullmatch('[a-f0-9]{64}', item['sha256']), 'invalid SHA-256')
        require(type(item.get('executable', False)) is bool, 'invalid executable flag')
    bins = m.get('bins')
    require(isinstance(bins, list) and 1 <= len(bins) <= 8, 'expected 1..8 bins')
    for commands in bins:
        require(isinstance(commands, list) and 1 <= len(commands) <= 16, 'expected 1..16 commands')
        for command in commands:
            require(isinstance(command, list) and command and all(isinstance(v, str) for v in command)
                    and command[0] in OPERATIONS, 'expected a strength command')


def worker(args):
    with controller.worker_task(args.manifest, args.store, validate) as (m, store, receipt):
        index = receipt['task_index']
        prefix = f'tasks/{index}/'
        receipt['commands'] = []
        with tempfile.TemporaryDirectory(prefix='strength-') as tmp:
            root = Path(tmp)
            for name, item in m['files'].items():
                raw = controller.read_uri(item['uri'])
                controller.require(digest(raw) == item['sha256'], 'input hash differs: ' + name)
                (root / name).write_bytes(raw)
                if item.get('executable'):
                    (root / name).chmod(0o755)
            try:
                for i, argv in enumerate(m['bins'][index]):
                    remaining = m['deadline_epoch'] - time.time() - 90
                    controller.require(remaining > 0, 'campaign deadline')
                    with (root / f'command-{i}.log').open('x') as log:
                        process = subprocess.Popen([sys.executable, '-m', 'tools.strength.cli', *argv],
                            cwd=root, env=dict(os.environ, PYTHONPATH='/opt/mshogi'),
                            start_new_session=True, stdout=log, stderr=subprocess.STDOUT)
                        try:
                            code = process.wait(timeout=remaining)
                        except subprocess.TimeoutExpired:
                            os.killpg(process.pid, signal.SIGKILL)
                            code = process.wait()
                    receipt['commands'].append({'argv': argv, 'exit': code})
                    controller.require(code == 0, f'command {i} failed')
            finally:
                for path in sorted(root.rglob('*')):
                    relative = str(path.relative_to(root))
                    if path.is_file() and relative not in m['files']:
                        controller.put_new(store, prefix + relative, path.read_bytes())


def options(sub):
    submit = sub.add_parser('cloud-submit', help='submit one digest-pinned bounded batch and verify cleanup')
    for field in ('manifest', 'config', 'image', 'job', 'out'):
        submit.add_argument('--' + field, required=True)
    submit.add_argument('--subscription', default='projects/mshogi/subscriptions/mshogi-pilot-completion')
    submit.set_defaults(func=lambda args: controller.run(args, policy=POLICY, manifest_validator=validate))
    work = sub.add_parser('cloud-worker', help='execute a claimed manifest bin on Batch')
    work.add_argument('--manifest', required=True)
    work.add_argument('--store', required=True)
    work.set_defaults(func=worker)
