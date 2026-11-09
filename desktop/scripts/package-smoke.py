"""Inspect a real extracted native package and exercise its bundled engine.

This external verifier is never bundled in the application.
"""
import argparse
import copy
import hashlib
import json
from pathlib import Path
import queue
import subprocess
import tempfile
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
host_bytes = hosts[0].read_bytes()
for marker in (b'MSHOGI_DESKTOP_FIXTURE', b'MSHOGI_VERIFICATION_ENGINE', b'Native Cocoa Quit (verification)',
               b'synthetic_snapshot', b'synthetic_command', b'desktop_verification_mode',
               b'native_contract_snapshot', b'native_contract_command', b'native_contract_file',
               b'mshogi.synthetic.strider16.v1', b'mshogi-verification-window-size',
               b'Verification window size', b'xiangqi-exact-evidence', b'MSHOGI_R6_REAL_ENGINE', b'MSHOGI_R6_DELAY_MS', b'xiangqi-capacity', b'xiangqi-warning', b'xiangqi-control-capacity', b'shogi-control-capacity', b'tenjiku-adams-standard', b'tenjiku-muller-standard'):
    assert marker not in host_bytes, f'verification marker entered a normal host: {marker!r}'
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
context = {
    'purpose': {'kind': 'engine_turn'}, 'request_id': 1, 'session_version': 1,
    'process_generation': 1,
    'game': {
        'envelope_version': 1, 'game_kind': 'chess',
        'rules_profile': 'mshogi.chess.match.v1',
        'profile_edition': 'FIDE-2023-01-01+mshogi.chess.match.v1',
        'native_schema_version': 1,
        'payload': {
            'schema_version': 1, 'rules_profile': 'mshogi.chess.match.v1',
            'initial_fen': 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
            'history_origin': 'standard_start', 'claim_policy': 'explicit_only',
            'events': [],
        },
    },
    'limits': {'depth': 1, 'nodes': 1000},
}
request = {'protocol': 4, 'context': context}
internal = subprocess.run([str(engine), 'internal-search'],
    input=json.dumps(request) + '\n', check=True, capture_output=True,
    text=True, timeout=15)
response = json.loads(internal.stdout)
assert response['protocol'] == request['protocol'], 'internal response transport mismatch'
observation = response['observation']
assert observation['schema_version'] == 1
assert observation['context'] == context, 'internal response changed exact request context'
assert observation['result']['kind'] == 'chess', observation
result = observation['result']['result']
assert result['kind'] == 'move', result
# Current classical opening fixture is independently specified by the native
# conventional-search tests (material/placement, not the former material-only tie).
assert result['action'] == 'b1c3', result
assert result['telemetry']['completed_depth'] == 1, result
assert result['telemetry']['nodes'] == 21, result
assert result['telemetry']['score'] == 61, result
assert result['telemetry']['principal_variation'][0] == {'kind': 'move', 'action': 'b1c3'}, result
assert result['telemetry']['status'] == 'complete', result
# Purpose-specific queries exercise the actual shipped advanced solver as well.
target = {'selection_id': 1, 'event_index': 0, 'board_ply': 0, 'session_version': 1,
          'label': 'Package verification: standard start',
          'turn': {'game_kind': 'chess', 'side': 'white'}}
analysis_request = copy.deepcopy(request)
analysis_request['context'].update(request_id=2, process_generation=2,
    purpose={'kind': 'move_analysis', 'target': target})
def query(request, game_kind="chess"):
    process = subprocess.run([str(engine), 'internal-search'],
        input=json.dumps(request) + '\n', check=True, capture_output=True,
        text=True, timeout=15)
    reply = json.loads(process.stdout)
    assert reply['protocol'] == request['protocol']
    assert reply['observation']['schema_version'] == 1
    assert reply['observation']['context'] == request['context'], 'analysis execution context changed'
    assert reply['observation']['result']['kind'] == game_kind, reply
    return reply
analysis_response = query(analysis_request)
assert analysis_response['observation']['result']['result']['kind'] == 'move', analysis_response
assert analysis_response['observation']['result']['result']['telemetry']['nodes'] == 21, analysis_response
full_request = copy.deepcopy(request)
full_request['context'].update(request_id=3, process_generation=3,
    limits={'depth': 1, 'nodes': 0},
    purpose={'kind': 'chess_full_reachability', 'target': target, 'mate': 'either',
             'limits': {'max_positions': 1, 'max_transitions': 1}})
full_response = query(full_request)
assert full_response['observation']['result']['result']['kind'] == 'reachability', full_response
unknown = full_response['observation']['result']['result']['result']['unknown']
assert unknown['reason'] in ('position_limit', 'transition_limit'), unknown
assert unknown['work']['positions'] <= 1 and unknown['work']['transitions'] <= 1, unknown
# Each selected Tenjiku profile is exercised through its real bundled engine.
# Inputs are native-generated independent diagram documents, external to the
# normal package; this is process/transport evidence, not host GUI evidence.
tenjiku_responses = []
for index, profile in enumerate(('adams', 'muller')):
    source = Path(__file__).with_name('fixtures') / f'tenjiku-{profile}-royal-engine.mshogi'
    document = json.loads(source.read_text(encoding='utf-8'))
    native = document['payload']['current']
    turn = native['payload']['current_position']['side']
    assert turn == 'sente' and native['payload']['conclusion'] is None
    ctx = {'purpose': {'kind': 'engine_turn'}, 'request_id': 100 + index * 2,
        'session_version': 1, 'process_generation': 100 + index * 2,
        'game': native, 'limits': {'depth': 1, 'nodes': 1024}}
    move_request = {'protocol': 4, 'context': ctx}
    move = query(move_request, 'tenjiku')
    result = move['observation']['result']['result']; telemetry = result['telemetry']
    assert result['kind'] == 'choice' and telemetry['completed_depth'] == 1
    assert telemetry['score'] == 999999 and 0 < telemetry['nodes'] <= 1024
    assert telemetry['perspective'] == 'sente' and telemetry['principal_variation'][0] == result['action']
    assert telemetry['evaluator_id'] == 'mshogi.tenjiku.material.v1' and telemetry['search_id'] == 'mshogi.tenjiku.full-width.v1'
    analysis = copy.deepcopy(move_request)
    analysis['context'].update(request_id=101+index*2,process_generation=101+index*2,
        purpose={'kind':'move_analysis','target':{'selection_id':1,'event_index':0,'board_ply':0,'session_version':1,
            'label':'Packaged Tenjiku independent royal diagram','turn':{'game_kind':'tenjiku','side':turn}}})
    analyzed = query(analysis, 'tenjiku')
    assert analyzed['observation']['result']['result']['action'] == result['action']
    tenjiku_responses.append({'profile':document['rules_profile'],'source_fixture':str(source),
        'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'engine_turn':move,'analysis':analyzed})
report = {
    'package_root': str(args.package_root),
    'engine': str(engine.relative_to(args.package_root.resolve())),
    'sha256': hashlib.sha256(engine.read_bytes()).hexdigest(),
    'host_sha256': hashlib.sha256(hosts[0].read_bytes()).hexdigest(),
    'version': version.stdout.strip(),
    'uci_observations': observations,
    'internal_response': response,
    'analysis_response': analysis_response,
    'full_response': full_response,
    'tenjiku_responses': tenjiku_responses,
    'files': [str(path.relative_to(args.package_root)) for path in files],
    'scope': 'Package inventory and actual UCI, engine-turn, move-analysis and bounded Full reachability queries. Both Tenjiku whole profiles additionally execute real diagram engine-turn and read-only analysis queries. Host acceptance and installed UI interaction are separate evidence.',
}
# Replay the real returned actions/PVs through the public native owner. This
# external development verifier is not a packaged executable or Python rules
# model; score alone cannot distinguish royal capture from a tied bare-royal win.
with tempfile.TemporaryDirectory(prefix='mshogi-package-native-replay-') as directory:
    source_report = Path(directory) / 'observations.json'
    source_report.write_text(json.dumps(report), encoding='utf-8')
    native_replay = subprocess.run(['cargo', 'run', '--release', '--locked', '-q',
        '-p', 'mshogi-desktop-session', '--example', 'write_tenjiku_gui_fixtures', '--',
        '--verify-package-report', str(source_report)],
        cwd=Path(__file__).resolve().parents[2], check=True, capture_output=True,
        text=True, timeout=300)
    report['tenjiku_native_replay'] = json.loads(native_replay.stdout)
args.report.write_text(json.dumps(report, indent=2) + '\n')
print(json.dumps(report, indent=2))
