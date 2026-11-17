"""MSH-185 single-game cloud adapter. Native rules and replay own all results.

Manifest v1: experiment, deadline_epoch, build (source and named engines), bins.
Each bin is a list of {id, pair_id, opponent, opening, swapped} game specs.
Engine entries have path, sha256, source, args and options (native UciOption list).
No retry or resume is automatic. Immutable launch claims prevent duplicate play.
"""
import argparse
import gzip
import json
import math
import os
from pathlib import Path
import re
import resource
import signal
import subprocess
import tempfile
import time
import urllib.request

from tools.experiments.core import decode, digest, encode, environment, file_hash
from tools.experiments.gcp import GCSStore, parse_uri


def require(condition, message):
    if not condition:
        raise ValueError(message)


def config(build, spec):
    names = ['candidate', spec['opponent']]
    if spec['swapped']:
        names.reverse()
    engines = []
    for name in names:
        engine = build['engines'][name]
        engines.append({'path': engine['path'], 'args': engine['args'],
                        'options': engine['options'], 'model_path': None,
                        'model_identity': name + '/' + engine['source'], 'deterministic': False})
    return {'schema_version': 1, 'id': spec['id'], 'engines': engines,
            'openings': [spec['opening']], 'paired': False, 'claim_policy': 'auto_claim',
            'time_control': {'increment': {'initial_ns': [60000000000, 60000000000],
                                           'increment_ns': 600000000}},
            'work': {'depth': None, 'nodes': None, 'movetime_ms': None}, 'max_plies': 512,
            'watchdogs': {'handshake_ms': 10000, 'ready_ms': 10000, 'response_ms': 400000,
                          'write_ms': 2000, 'cleanup_ms': 2000},
            'io': {'line_bytes': 8192, 'queued_lines': 256, 'transcript_bytes': 4194304,
                   'stderr_bytes': 65536, 'command_bytes': 4096}}


def validate_manifest(manifest):
    require(manifest.get('schema') == 1, 'unsupported manifest')
    require(re.fullmatch(r'[a-z0-9][a-z0-9-]{0,80}', manifest.get('experiment', '')), 'invalid experiment')
    deadline = manifest.get('deadline_epoch')
    require(type(deadline) in (int, float) and math.isfinite(deadline), 'finite deadline required')
    bins = manifest.get('bins')
    require(isinstance(bins, list) and 1 <= len(bins) <= 8, 'one to eight worker bins required')
    engines = manifest['build']['engines']
    require('runner' in engines and 'candidate' in engines, 'missing native binaries')
    for engine in engines.values():
        require(Path(engine['path']).is_absolute(), 'absolute binary path required')
        require(re.fullmatch('[a-f0-9]{64}', engine['sha256']), 'invalid binary hash')
        require(bool(engine['source']), 'source required')
    seen = set()
    pairs = {}
    total = 0
    for tasks in bins:
        require(isinstance(tasks, list) and tasks, 'empty worker bin')
        for spec in tasks:
            require(re.fullmatch(r'[a-zA-Z0-9][a-zA-Z0-9_-]{0,95}', spec['id']), 'unsafe game ID')
            require(spec['id'] not in seen, 'duplicate game ID')
            seen.add(spec['id'])
            require(type(spec['swapped']) is bool and bool(spec['pair_id']), 'invalid pair identity')
            pair = pairs.setdefault(spec['pair_id'], [])
            require(not any(old['swapped'] == spec['swapped'] for old in pair), 'duplicate pair color')
            require(all(old['opponent'] == spec['opponent'] and old['opening'] == spec['opening'] for old in pair),
                    'pair opponents or openings differ')
            pair.append(spec)
            require(spec['opponent'] in engines and spec['opponent'] not in ('runner', 'candidate'), 'invalid opponent')
            require(len(spec['opening']['moves']) == 16 and spec['opening'].get('fen') is None,
                    'expected complete standard-start eight-move opening')
            config(manifest['build'], spec)
            total += 1
    require(total <= 400, 'batch exceeds campaign game cap')



# Reviewed MSH-185 classification correction: this Linux release deliberately
# sleeps for one second after quit; this frozen runner gives 100 ms grace.
# An expected kill after a native conclusion is not a gameplay/cleanup failure.
SUPERNOVA_SHUTDOWN = {
    'source': 'ffd139d1f3e56d39af41cb7cca78a5212bc96a88',
    'sha256': 'ac63e008196b2c61755184e5d2c9701559137957e5086b188ceec978aeff2c5e',
    'runner_source': 'f433ea13ef0c6682d7c84388142dc9c8238e4e16',
    'runner_sha256': '0807bb820f5f43889c2368c453e4cfaf03aa09dade1180182f43366ba6a14c2d',
}


def normal_shutdown_kill(game, cfg, build, observation):
    """Return narrow reviewed provenance, or None; never infer a game result."""
    index = observation.get('engine')
    if index not in (0, 1) or len(game['engines']) != 2:
        return None
    known = SUPERNOVA_SHUTDOWN
    runner = build['engines']['runner']
    matches = [e for e in build['engines'].values() if e['path'] == cfg['engines'][index]['path']]
    if (len(matches) != 1 or matches[0]['source'] != known['source'] or
            matches[0]['sha256'] != known['sha256'] or
            build['source'] != known['runner_source'] or
            runner['source'] != known['runner_source'] or runner['sha256'] != known['runner_sha256']):
        return None
    status = observation['exit'] or {}
    if status.get('signal') != 9 or status.get('code') is not None or status.get('success') is not False:
        return None
    if (game['stop'] != 'chess_conclusion' or not game['conclusion'] or
            game['failure'] or game['diagnostic_failures'] or game['transcript']['truncated']):
        return None
    for other in game['engines']:
        if other['identity_truncated'] or other['cleanup_failures'] or other['stderr']['truncated']:
            return None
        if other is not observation and not (other['exit'] and other['exit']['success']):
            return None
    entries = game['transcript']['entries']
    if any(e.get('truncated', False) for e in entries):
        return None
    try:
        record = json.loads(game['match_record_json'])
        terminal = record['events'][-1]
        if (terminal['reply']['conclusion'] != game['conclusion'] or
                terminal['reply'].get('rejection') is not None):
            return None
        version = terminal['reply']['version']
        bestmoves = [e for e in entries if e['direction'] == 'response' and e['text'].startswith('bestmove ')]
        if not bestmoves:
            return None
        last_move_ns = max(e['at_ns'] for e in bestmoves)
        quits = [e for e in entries if e['engine'] == index and e['direction'] == 'command'
                 and e['text'] == 'quit' and e['phase'] == 'cleanup' and e['write_completed'] is True
                 and e['game_version'] == version and e['at_ns'] >= terminal['request']['at_ns']
                 and e['at_ns'] > last_move_ns]
        if len(quits) != 1:
            return None
    except (KeyError, IndexError, TypeError, ValueError):
        return None
    return {'engine': index, 'source': known['source'], 'sha256': known['sha256'],
            'signal': 9, 'terminal_version': version, 'quit_at_ns': quits[0]['at_ns'],
            'reason': 'reviewed Supernova 2.3 Linux quit sleep exceeds frozen runner grace'}


def audit(outer, cfg, build, spec):
    require(outer['protocol'] == 'uci' and outer['schema'] == 1, 'wrong native envelope')
    require(digest(outer['report_json'].encode()) == outer['report_sha256'], 'report hash differs')
    observed = outer['environment']
    require(observed.get('harness_revision') == build['source'], 'wrong runner revision')
    require(observed.get('harness_dirty_sha256') == digest(b''), 'dirty runner build')
    report = json.loads(outer['report_json'])
    require(report['config'] == cfg and len(report['games']) == 1, 'wrong game configuration')
    for index, engine in enumerate(cfg['engines']):
        identities = [e for e in build['engines'].values() if e['path'] == engine['path']]
        require(len(identities) == 1 and report['engine_identities'][index]['executable']['sha256'] == identities[0]['sha256'],
                'wrong participant binary')
    game = report['games'][0]
    require(game['pairing'] == 0 and game['engines_by_color'] == [0, 1], 'wrong game colors')
    healthy = not (game['failure'] or game['diagnostic_failures'] or game['transcript']['truncated'])
    healthy = healthy and len(game['engines']) == 2
    shutdown_kills = []
    for engine in game['engines']:
        healthy = healthy and not (engine['identity_truncated'] or engine['cleanup_failures'] or engine['stderr']['truncated'])
        success = bool(engine['exit'] and engine['exit']['success'])
        if not success:
            admitted = normal_shutdown_kill(game, cfg, build, engine)
            if admitted is not None:
                shutdown_kills.append(admitted)
                success = True
        healthy = healthy and success
    searches = []
    for entry in game['transcript']['entries']:
        if entry['direction'] == 'command' and entry['text'].startswith('go'):
            fields = entry['text'].split()
            require(fields[0] == 'go' and set(fields[1::2]) == {'wtime', 'btime', 'winc', 'binc'},
                    'unexpected work limit in clock game')
            values = dict(zip(fields[1::2], fields[2::2]))
            require(values['winc'] == '600' and values['binc'] == '600', 'wrong clock increment')
            searches.append(entry)
    require(searches, 'opening did not lead to play')
    summary = report['summaries'][0]
    require(summary['games'] == 1, 'wrong summary count')
    concluded = game['stop'] == 'chess_conclusion' and summary['unscored'] == 0
    require(sum(summary['wins']) + summary['draws'] + summary['unscored'] == 1, 'bad summary')
    candidate_index = 1 if spec['swapped'] else 0
    score = summary['wins'][candidate_index] + 0.5 * summary['draws'] if concluded and healthy else None
    return {'healthy': bool(healthy), 'complete': bool(concluded and healthy), 'score': score,
            'clock_review_required': 'flag_fall' in json.dumps(game['conclusion']),
            'summary': summary, 'stop': game['stop'], 'conclusion': game['conclusion'],
            'search_requests': len(searches), 'native_environment': observed,
            'normal_shutdown_kills': shutdown_kills}


def limits():
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    resource.setrlimit(resource.RLIMIT_FSIZE, (64 * 1024 * 1024, 64 * 1024 * 1024))


def group_alive(pid):
    try:
        os.killpg(pid, 0)
        return True
    except ProcessLookupError:
        return False


def stop_group(process):
    try:
        os.killpg(process.pid, signal.SIGKILL)
    except ProcessLookupError:
        pass
    process.wait(timeout=10)
    try:
        os.killpg(process.pid, 0)
        return False
    except ProcessLookupError:
        return True


def run_game(build, spec, dest, deadline_epoch):
    dest.mkdir()
    cfg = config(build, spec)
    (dest / 'config.json').write_bytes(encode(cfg))
    start = time.monotonic()
    usage = resource.getrusage(resource.RUSAGE_CHILDREN)
    receipt = {'id': spec['id'], 'pair_id': spec['pair_id'], 'opponent': spec['opponent'],
               'swapped': spec['swapped'], 'started_epoch': time.time(), 'complete': False, 'score': None}
    process = None
    try:
        for name in ('runner', 'candidate', spec['opponent']):
            engine = build['engines'][name]
            require(file_hash(engine['path']) == engine['sha256'], 'binary changed: ' + name)
        # Reserve time for child cleanup, replay and evidence publication.
        timeout = min(600, deadline_epoch - time.time() - 90)
        require(timeout > 0, 'campaign deadline leaves no execution time')
        command = [build['engines']['runner']['path'], 'experiment-external', 'uci',
                   str(dest / 'config.json'), str(dest / 'report.json')]
        receipt['command'] = command
        with (dest / 'stdout.log').open('wb') as out, (dest / 'stderr.log').open('wb') as err:
            process = subprocess.Popen(command, stdout=out, stderr=err, start_new_session=True, preexec_fn=limits)
            receipt['pid'] = process.pid
            (dest / 'started.json').write_bytes(encode(receipt))
            try:
                receipt['returncode'] = process.wait(timeout=timeout)
            except subprocess.TimeoutExpired:
                receipt['outer_timeout'] = True
            receipt['residual_process_group'] = group_alive(process.pid) and not receipt.get('outer_timeout', False)
            receipt['process_group_settled'] = stop_group(process)
        report = dest / 'report.json'
        if report.exists():
            raw = report.read_bytes()
            receipt['report_sha256'] = digest(raw)
            (dest / 'report.json.gz').write_bytes(gzip.compress(raw, mtime=0))
            receipt.update(audit(decode(raw), cfg, build, spec))
            replay = subprocess.run([build['engines']['runner']['path'], 'experiment-external-replay', str(report)],
                                    capture_output=True, text=True, timeout=60)
            receipt['replay'] = {'returncode': replay.returncode, 'stdout': replay.stdout, 'stderr': replay.stderr}
            receipt['complete'] = bool(receipt['complete'] and replay.returncode == 0 and
                                       receipt.get('returncode') == 0 and receipt['process_group_settled'] and
                                       not receipt.get('outer_timeout') and not receipt.get('residual_process_group'))
            report.unlink()
    except Exception as error:
        receipt.update(error=repr(error), complete=False)
    finally:
        if process is not None and process.poll() is None:
            receipt['process_group_settled'] = stop_group(process)
        if not receipt['complete']:
            receipt['score'] = None
        usage_end = resource.getrusage(resource.RUSAGE_CHILDREN)
        receipt.update(ended_epoch=time.time(), wall_seconds=time.monotonic() - start,
                       cpu_seconds=usage_end.ru_utime + usage_end.ru_stime - usage.ru_utime - usage.ru_stime)
        (dest / 'receipt.json').write_bytes(encode(receipt))
    return receipt


def put_new(store, key, data):
    require(store.put(key, data), 'existing immutable artifact: ' + key)


def cloud_environment():
    result = environment()
    require(result['architecture'] == 'x86_64' and result['platform'].startswith('Linux'), 'expected Linux x86_64')
    result['gcp'] = {}
    for field in ('machine-type', 'zone', 'id', 'cpu-platform'):
        request = urllib.request.Request('http://metadata.google.internal/computeMetadata/v1/instance/' + field,
                                         headers={'Metadata-Flavor': 'Google'})
        with urllib.request.urlopen(request, timeout=5) as response:
            result['gcp'][field] = response.read(4096).decode()
    require(result['gcp']['machine-type'].rsplit('/', 1)[-1] == 'c3-standard-4', 'wrong VM type')
    require(result['gcp']['zone'].rsplit('/', 1)[-1].startswith('us-central1-'), 'wrong VM region')
    result['batch'] = {key: os.environ.get(key) for key in
                       ('BATCH_TASK_INDEX', 'BATCH_TASK_COUNT', 'BATCH_JOB_UID', 'BATCH_TASK_RETRY_ATTEMPT')}
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--manifest', required=True)
    parser.add_argument('--store', required=True)
    args = parser.parse_args()
    bucket, key = parse_uri(args.manifest)
    raw = GCSStore('gs://' + bucket).get(key)
    manifest = decode(raw)
    validate_manifest(manifest)
    store = GCSStore(args.store)
    index = int(os.environ['BATCH_TASK_INDEX'])
    require(0 <= index < len(manifest['bins']), 'invalid task index')
    claim = {'manifest_sha256': digest(raw), 'task_index': index, 'started_epoch': time.time()}
    task_prefix = f'tasks/{index}'
    put_new(store, task_prefix + '/started.json', encode(claim))
    receipt = dict(claim, complete=False, games=[])
    try:
        receipt['environment'] = cloud_environment()
        with tempfile.TemporaryDirectory(prefix='msh185-') as directory:
            for spec in manifest['bins'][index]:
                require(time.time() < manifest['deadline_epoch'] - 90, 'campaign deadline reached')
                prefix = 'games/' + spec['id']
                put_new(store, prefix + '/launch.json', encode(claim | {'spec': spec}))
                dest = Path(directory) / spec['id']
                result = run_game(manifest['build'], spec, dest, manifest['deadline_epoch'])
                # Receipt is last: its presence commits the artifact upload.
                files = sorted(p for p in dest.iterdir() if p.name != 'receipt.json') + [dest / 'receipt.json']
                for path in files:
                    put_new(store, prefix + '/' + path.name, path.read_bytes())
                receipt['games'].append(result)
                require(result['complete'], 'unscored game; stop worker and retain evidence')
        receipt['complete'] = True
    except Exception as error:
        receipt['error'] = repr(error)
        raise
    finally:
        receipt['ended_epoch'] = time.time()
        put_new(store, task_prefix + '/receipt.json', encode(receipt))


if __name__ == '__main__':
    main()
