"""Inspect a real extracted native package and exercise its bundled engine.

This external verifier is never bundled in the application.
"""
import argparse
import hashlib
import json
from pathlib import Path
import queue
import subprocess
import threading
import time

parser = argparse.ArgumentParser()
parser.add_argument('package_root', type=Path)
parser.add_argument('--report', type=Path, default=Path('package-smoke.json'))
args = parser.parse_args()
files = sorted(path for path in args.package_root.rglob('*') if path.is_file())
for path in files:
    lowered = path.name.lower()
    assert not any(marker in lowered for marker in ['fixture', 'test-driver', 'test_driver', 'mshogi-match']), path
engines = [path for path in files if path.name in ('mshogi', 'mshogi.exe')]
assert len(engines) == 1, f'expected one packaged engine, got {engines}'
hosts = [path for path in files if path.name in ('mshogi-desktop', 'mshogi-desktop.exe')]
assert len(hosts) == 1, f'expected one packaged desktop host, got {hosts}'
assert b'MSHOGI_DESKTOP_FIXTURE' not in hosts[0].read_bytes(), 'verification initializer entered a normal host'
engine = engines[0].resolve()
version = subprocess.run([str(engine), '--version'], check=True, capture_output=True, text=True, timeout=10)
child = subprocess.Popen([str(engine), 'uci'], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
lines = queue.Queue()
def read_stdout():
    for line in child.stdout:
        lines.put(line.rstrip())
    lines.put(None)
threading.Thread(target=read_stdout, daemon=True).start()
observations = []
try:
    child.stdin.write('uci\nisready\nposition startpos\ngo depth 1 nodes 1000\n')
    child.stdin.flush()
    deadline = time.monotonic() + 15
    while True:
        remaining = deadline - time.monotonic()
        assert remaining > 0, 'packaged engine response timeout'
        line = lines.get(timeout=remaining)
        assert line is not None, 'packaged engine exited before a move'
        observations.append(line)
        if line.startswith('bestmove '):
            assert line.split()[1] in {f'{file}2{file}{rank}' for file in 'abcdefgh' for rank in '34'} | {'b1a3','b1c3','g1f3','g1h3'}, line
            break
    assert 'uciok' in observations and 'readyok' in observations
    child.stdin.write('quit\n')
    child.stdin.flush()
    assert child.wait(timeout=10) == 0
finally:
    if child.poll() is None:
        child.kill()
        child.wait(timeout=5)
# Exercise the same lossless interface used by the desktop, independently of
# UCI's compatibility mapping. This verifier is external to normal packages.
request = {
    'protocol': 1, 'request_id': 1, 'session_version': 1,
    'process_generation': 1,
    'game': {
        'schema_version': 1, 'rules_profile': 'mshogi.chess.match.v1',
        'initial_fen': 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
        'history_origin': 'standard_start', 'claim_policy': 'explicit_only',
        'events': [],
    },
    'limits': {'depth': 1, 'nodes': 1000},
}
internal = subprocess.run([str(engine), 'internal-search'],
    input=json.dumps(request) + '\n', check=True, capture_output=True,
    text=True, timeout=15)
response = json.loads(internal.stdout)
assert response['context'] == request, 'internal response changed exact request context'
result = response['result']
assert result['kind'] == 'move', result
assert result['action'] == 'a2a3', result
assert result['telemetry']['completed_depth'] == 1, result
assert result['telemetry']['nodes'] == 21, result
assert result['telemetry']['score'] == 0, result
assert result['telemetry']['status'] == 'complete', result
report = {
    'package_root': str(args.package_root),
    'engine': str(engine.relative_to(args.package_root.resolve())),
    'sha256': hashlib.sha256(engine.read_bytes()).hexdigest(),
    'host_sha256': hashlib.sha256(hosts[0].read_bytes()).hexdigest(),
    'version': version.stdout.strip(),
    'uci_observations': observations,
    'internal_response': response,
    'files': [str(path.relative_to(args.package_root)) for path in files],
    'scope': 'Package inventory and real bundled-engine UCI and structured searches. Installed UI interaction is MSH-38.',
}
args.report.write_text(json.dumps(report, indent=2) + '\n')
print(json.dumps(report, indent=2))
