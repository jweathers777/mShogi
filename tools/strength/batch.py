"""Run a frozen set of strength CLI commands through cost-bounded Cloud Batch."""
import argparse
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
from . import anchor_cloud as controller, internal_match

POLICY = controller.CampaignPolicy('strength', None, 10, 'tools.strength.cli', ('cloud-worker',))
UPLOAD_SECONDS = 90
OPERATIONS = {'openings', 'suite', 'probe', 'match', 'export', 'judge', 'signature', 'texel', 'selfplay', 'tenjiku-work', 'bench', 'blunders'}


def validate(m):
    require = controller.require
    require(m.get('schema') == 1, 'expected strength manifest schema 1')
    for field in ('deadline_epoch', 'campaign_started_epoch', 'estimated_compute_before_usd'):
        if field == 'estimated_compute_before_usd' or field in m:
            require(type(m.get(field)) in (int, float) and math.isfinite(m[field]), 'invalid ' + field)
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


def projected_runtime(manifest):
    """Longest bin of serial match commands, including time to stage and upload evidence."""
    bins = []
    for commands in manifest['bins']:
        seconds = 0
        for operation, *command in commands:
            if operation not in ('match', 'selfplay'):
                return None
            parser = argparse.ArgumentParser(prog='strength ' + operation)
            internal_match.options(parser, selfplay=operation == 'selfplay')
            settings = parser.parse_args(command)
            internal_match.configure(settings, parser)
            seconds += min(settings.wall_seconds, internal_match.projected_seconds(settings))
        bins.append(seconds)
    return max(bins) + UPLOAD_SECONDS


def stop_session(process):
    """Kill the Linux command session, including its independent runner groups."""
    internal_match.signal_group(process.pid, signal.SIGKILL)
    process.wait()
    for entry in Path('/proc').iterdir():
        if not entry.name.isdecimal():
            continue
        pid = int(entry.name)
        try:
            if os.getsid(pid) == process.pid:
                internal_match.signal_group(os.getpgid(pid), signal.SIGKILL)
        except ProcessLookupError:
            continue


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
                    remaining = m['deadline_epoch'] - time.time() - controller.CLEANUP_SECONDS - UPLOAD_SECONDS
                    controller.require(remaining > 0, 'campaign deadline')
                    with (root / f'command-{i}.log').open('x') as log:
                        process = subprocess.Popen([sys.executable, '-m', 'tools.strength.cli', *argv],
                            cwd=root, env=dict(os.environ, PYTHONPATH='/opt/mshogi'),
                            start_new_session=True, stdout=log, stderr=subprocess.STDOUT)
                        try:
                            code = process.wait(timeout=remaining)
                        except subprocess.TimeoutExpired:
                            stop_session(process)
                            code = process.returncode
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
    submit.set_defaults(func=lambda args: controller.run(args, policy=POLICY, manifest_validator=validate,
                                                        project_runtime=projected_runtime))
    work = sub.add_parser('cloud-worker', help='execute a claimed manifest bin on Batch')
    work.add_argument('--manifest', required=True)
    work.add_argument('--store', required=True)
    work.set_defaults(func=worker)
