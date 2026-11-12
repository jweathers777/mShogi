#!/usr/bin/env python3
"""One bounded Linux real-runner policy smoke, never a scored study pair.

Usage: validate_policy_match.py --build BUILD_JSON --output OUTPUT_DIR [--store gs://bucket]
       [--prefix unique/validation-prefix]
Creates fresh output only. Optional immutable upload includes failure evidence.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import signal
import subprocess
import sys
import time

STUDY = Path(__file__).resolve().parents[2]
REPO = STUDY.parents[3]
sys.path.insert(0, str(STUDY))
sys.path.insert(0, str(REPO))
import run_batch
import analysis


def bounded_process(command, dest, label, deadline):
    """Own and settle every process group, including native replay."""
    process = None
    result = {'command': command, 'started_utc': run_batch.utc()}
    try:
        remaining = deadline - time.monotonic()
        if remaining <= 0:
            raise TimeoutError('180-second validation process budget exhausted')
        with (dest / (label + '.stdout.log')).open('wb') as out, (dest / (label + '.stderr.log')).open('wb') as err:
            process = subprocess.Popen(command, stdout=out, stderr=err,
                                       start_new_session=True, preexec_fn=run_batch.limits)
            result['pid'] = process.pid
            run_batch.save(dest / (label + '.process.json'), result)
            try:
                result['returncode'] = process.wait(timeout=remaining)
            except subprocess.TimeoutExpired:
                result['timeout'] = True
                raise TimeoutError('180-second validation process budget exhausted')
    finally:
        if process is not None:
            try:
                os.killpg(process.pid, 0)
            except ProcessLookupError:
                result['process_group_settled'] = True
            else:
                result['residual_process_group'] = True
                os.killpg(process.pid, signal.SIGKILL)
                process.wait(timeout=10)
                # Reap the owned process; descendants must disappear before success.
                for _ in range(50):
                    try:
                        os.killpg(process.pid, 0)
                    except ProcessLookupError:
                        result['process_group_settled'] = True
                        break
                    time.sleep(.02)
                result.setdefault('process_group_settled', False)
        result['ended_utc'] = run_batch.utc()
        run_batch.save(dest / (label + '.process.json'), result)
    if result.get('returncode') != 0 or not result.get('process_group_settled') or result.get('residual_process_group'):
        raise RuntimeError(label + ' failed or left a process group; see process receipt')
    return result


def validate_report(outer, cfg, build):
    audit = run_batch.audit(outer, cfg, build)
    assert audit['healthy'] and not audit['complete'], audit
    assert audit['stops'] == ['ply_limit', 'ply_limit'], audit
    assert audit['searched_plies'] == 8, audit
    assert [len(x) for x in audit['response_seconds_by_engine']] == [4, 4], audit
    assert audit['summary']['unscored'] == 2
    assert sum(audit['summary']['wins']) == 0 and audit['summary']['draws'] == 0
    report = json.loads(outer['report_json'])
    decisions, _ = analysis.extract_decisions(report)
    assert len(decisions) == 8 and all(d['origin'] == 'played' for d in decisions)
    own_pvs = []
    for game in report['games']:
        assert game['conclusion'] is None
        record = json.loads(game['match_record_json'])
        events = record['events']
        assert len(events) == 5, 'expected accepted start plus four played moves'
        assert events[0]['request']['command'] == 'start'
        assert events[0]['request']['actor'] == 'black', 'imported root starts with Black'
        assert all(not event['reply']['rejection'] for event in events)
        assert all(isinstance(event['request']['command'], dict)
                   and set(event['request']['command']) == {'move'} for event in events[1:])
        assert not any(isinstance(event['request']['command'], dict)
                       and 'claim' in event['request']['command'] for event in events)
        pending = {}
        completed = []
        for entry in game['transcript']['entries']:
            engine = entry['engine']
            text = entry['text']
            if entry['direction'] == 'command' and text.startswith('go '):
                assert engine not in pending
                pending[engine] = []
            elif entry['direction'] == 'response' and engine in pending:
                if text.startswith('info '):
                    pending[engine].append(text)
                elif text.startswith('bestmove '):
                    move = text.split()[1]
                    infos = pending.pop(engine)
                    completed.append((engine, move))
                    if engine == 0:
                        lines = [line.split() for line in infos if ' pv ' in line]
                        assert lines, 'mShogi bestmove lacks searched PV'
                        words = lines[-1]
                        assert int(words[words.index('depth') + 1]) >= 1
                        assert int(words[words.index('nodes') + 1]) > 0
                        assert words[words.index('pv') + 1] == move
                        own_pvs.append({'game': game['pairing'], 'move': move, 'info': infos[-1]})
        assert not pending and len(completed) == 4
        expected = [(d['engine_index'], d['played']) for d in decisions if d['game'] == game['pairing']]
        assert completed == expected, 'searched bestmoves differ from accepted native actions'
        for engine in game['engines']:
            assert 'fallback' not in engine['stderr']['text']
    assert len(own_pvs) == 4
    return {'audit': audit, 'candidate_PV_backed_moves': own_pvs,
            'native_accepted_moves': 8, 'claims': 0,
            'interpretation': 'Intentional four-ply cap in each color; healthy but incomplete/unscored. Validation only, not study games.'}


def upload_artifacts(dest, uri, prefix):
    from tools.experiments.cli import store_at
    store = store_at(uri)
    local_manifest = [{'name': path.name, 'bytes': path.stat().st_size,
                       'sha256': run_batch.sha(path)}
                      for path in sorted(dest.iterdir())
                      if path.is_file() and path.name not in ('upload.json', 'artifact-manifest.json')]
    run_batch.save(dest / 'artifact-manifest.json', {'schema': 1, 'artifacts': local_manifest})
    manifest = []
    for path in sorted(dest.iterdir()):
        if not path.is_file() or path.name == 'upload.json':
            continue
        data = path.read_bytes()
        key = prefix.rstrip('/') + '/' + path.name
        if not store.put(key, data):
            if store.get(key) != data:
                raise RuntimeError('Immutable upload collision: ' + key)
        manifest.append({'name': path.name, 'key': key, 'bytes': len(data),
                         'sha256': hashlib.sha256(data).hexdigest()})
    receipt = {'status': 'uploaded', 'store': uri, 'artifacts': manifest, 'finished_utc': run_batch.utc()}
    run_batch.save(dest / 'upload.json', receipt)
    data = (dest / 'upload.json').read_bytes()
    key = prefix.rstrip('/') + '/upload.json'
    if not store.put(key, data) and store.get(key) != data:
        raise RuntimeError('Immutable upload receipt collision')


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--build', required=True)
    ap.add_argument('--output', required=True)
    ap.add_argument('--store')
    ap.add_argument('--prefix')
    args = ap.parse_args()
    if bool(args.store) != bool(args.prefix):
        ap.error('--store and --prefix must be supplied together')
    if args.prefix and (args.prefix.startswith('/') or any(x in ('', '.', '..') for x in args.prefix.split('/'))):
        ap.error('prefix must be a nonempty relative object prefix without dot segments')
    dest = Path(args.output).resolve()
    dest.mkdir(parents=True, exist_ok=False)
    start = time.monotonic()
    cpu = resource.getrusage(resource.RUSAGE_CHILDREN)
    receipt = {'started_utc': run_batch.utc(), 'status': 'running', 'smoke_pass': False,
               'helper_sha256': run_batch.sha(__file__), 'process_budget_seconds': 180}
    exitcode = 1
    try:
        assert sys.platform.startswith('linux'), 'This helper is for the pinned Linux worker, not local Mac matches'
        build_path = Path(args.build).resolve()
        build = run_batch.read(build_path)
        run_batch.save(dest / 'build-input.json', build)
        receipt['build_sha256'] = run_batch.sha(build_path)
        for key in ['candidate', 'runner', 'oxidation060']:
            engine = build['engines'][key]
            assert run_batch.sha(engine['path']) == engine['sha256'], key + ' binary identity mismatch'
        spec = {'id': 'msh161-automatic-threshold-policy-smoke', 'opponent': 'oxidation060',
                'opening': {'id': 'kr-k-optional-fifty', 'fen': '7k/8/8/8/8/8/8/KR6 b - - 100 1', 'moves': []}}
        cfg = run_batch.config(build, spec)
        cfg['claim_policy'] = 'explicit_only'
        cfg['max_plies'] = 4
        run_batch.save(dest / 'config.json', cfg)
        runner = build['engines']['runner']['path']
        receipt['runner'] = bounded_process([runner, 'experiment-external', 'uci', str(dest / 'config.json'), str(dest / 'report.json')], dest, 'runner', start + 180)
        receipt['replay'] = bounded_process([runner, 'experiment-external-replay', str(dest / 'report.json')], dest, 'replay', start + 180)
        receipt.update(validate_report(run_batch.read(dest / 'report.json'), cfg, build))
        receipt['report_sha256'] = run_batch.sha(dest / 'report.json')
        receipt['status'] = 'passed'
        receipt['smoke_pass'] = True
        exitcode = 0
    except BaseException as exc:
        receipt['status'] = 'failed'
        receipt['error'] = repr(exc)
    finally:
        endcpu = resource.getrusage(resource.RUSAGE_CHILDREN)
        receipt.update(ended_utc=run_batch.utc(), wall_seconds=time.monotonic() - start,
                       cpu_seconds=endcpu.ru_utime + endcpu.ru_stime - cpu.ru_utime - cpu.ru_stime)
        run_batch.save(dest / 'receipt.json', receipt)
        if args.store:
            try:
                upload_artifacts(dest, args.store, args.prefix)
            except BaseException as exc:
                run_batch.save(dest / 'upload-error.json', {'error': repr(exc), 'finished_utc': run_batch.utc()})
                exitcode = 1
        print(json.dumps({'status': receipt['status'], 'smoke_pass': receipt['smoke_pass'], 'output': str(dest), 'exitcode': exitcode}), flush=True)
    return exitcode


if __name__ == '__main__':
    raise SystemExit(main())
