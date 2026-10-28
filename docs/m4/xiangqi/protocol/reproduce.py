#!/usr/bin/env python3
"""Verification-only real-process experiment; no product/runtime dependency.

Supply already built executables and the admitted matching NNUE. Never fetch
weights or overwrite historical evidence. Native runner validates every record.
"""
import argparse
import hashlib
import json
import pathlib
import platform
import subprocess
import sys


def digest(path):
    path = pathlib.Path(path)
    return dict(bytes=path.stat().st_size, sha256=hashlib.sha256(path.read_bytes()).hexdigest())


def main():
    parser = argparse.ArgumentParser()
    for name in ['mshogi', 'runner', 'opponent', 'network', 'out']:
        parser.add_argument('--' + name, required=True, type=pathlib.Path)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    own = dict(path=str(args.mshogi.resolve()), args=['--game', 'xiangqi', 'uci'], options=[], model_path=None,
               model_identity='mshogi.xiangqi.material-advance.v1; mshogi.xiangqi.full-width.v1; sound_fast_only', deterministic=True)
    opponent = dict(path=str(args.opponent.resolve()), args=[], options=[dict(name='Threads', value='1'), dict(name='Hash', value='16'), dict(name='Ponder', value='false'), dict(name='MultiPV', value='1'), dict(name='EvalFile', value=str(args.network.resolve()))], model_path=str(args.network.resolve()), model_identity='Pikafish2026-09-06 NNUE sha256:' + digest(args.network)['sha256'], deterministic=True)
    base = dict(schema_version=1, profile='mshogi.xiangqi.wxf2018.v1', dialect='pikafish_uci0_9', claim_policy='auto_valid_claims', id='msh100', time_control='untimed', process_policy='fresh_per_turn', engines=[own, own], openings=[dict(id='standard-functional', fen=None, canonical=None, moves=[]), dict(id='MSH105-P1-natural-stalemate', fen='5k3/9/4R4/9/9/9/9/9/9/3K5 w - - 0 1', canonical=None, moves=[])], paired=True, work=[dict(depth=1, nodes=128, movetime_ms=None)] * 2, max_actions=4, max_game_ms=30000, watchdogs=dict(handshake_ms=3000, ready_ms=3000, response_ms=5000, write_ms=1000, cleanup_ms=1000), io=dict(line_bytes=4096, queued_lines=32, transcript_bytes=262144, stderr_bytes=4096, command_bytes=4096))
    manifest = dict(format='mshogi.xiangqi.protocol-execution.v1', source_revision=subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(), platform=platform.platform(), python=sys.version, command=sys.argv, engines={str(x.resolve()): digest(x) for x in [args.mshogi, args.runner, args.opponent, args.network]}, results=[])
    reports = []
    for label in ['selfplay', 'opponent', 'selfplay-repeat']:
        config = json.loads(json.dumps(base))
        config['id'] = 'msh100-' + label
        if label == 'opponent':
            config['engines'] = [own, opponent]
        config_path = args.out / (label + '.config.json')
        report_path = args.out / (label + '.report.json')
        with config_path.open('x') as stream:
            json.dump(config, stream, indent=2)
            stream.write('\n')
        command = [str(args.runner.resolve()), 'xiangqi-run', str(config_path), str(report_path)]
        result = subprocess.run(command, capture_output=True, text=True, timeout=125)
        entry = dict(label=label, run_command=command, returncode=result.returncode, stdout=result.stdout, stderr=result.stderr)
        manifest['results'].append(entry)
        print(label, result.returncode, result.stdout, result.stderr, flush=True)
        if result.returncode:
            break
        replay = subprocess.run([str(args.runner.resolve()), 'xiangqi-replay', str(report_path)], capture_output=True, text=True, timeout=15)
        entry['replay'] = dict(returncode=replay.returncode, stdout=replay.stdout, stderr=replay.stderr)
        data = json.loads(report_path.read_text())
        entry['games'] = [dict(stop=g['stop'], conclusion=g['conclusion'], turns=len(g['turns']), failure=g['failure']) for g in data['games']]
        entry['passed'] = replay.returncode == 0 and all(g['failure'] is None for g in data['games'])
        reports.append(data)
        if replay.returncode or any(g['failure'] is not None for g in data['games']):
            break
    if len(reports) == 3:
        def semantics(report):
            return [(g['game'], g['conclusion'], g['score'], [(t['actor'], t['reply'], t['context_sha256']) for t in g['turns']]) for g in report['games']]
        manifest['repeat_equal'] = semantics(reports[0]) == semantics(reports[2])
        manifest['repeat_scope'] = 'Full native canonical games, accepted actions/actors/context digests and conclusions under finite depth1/nodes128, single-thread/fresh-process settings; OS times/PIDs/diagnostics remain observations.'
    with (args.out / 'execution.json').open('x') as stream:
        json.dump(manifest, stream, indent=2)
        stream.write('\n')
    if len(reports) != 3 or not manifest.get('repeat_equal') or not all(r.get('passed', False) for r in manifest['results']):
        raise SystemExit('runtime admission or semantic repeat did not pass; retain the failed artifacts')


if __name__ == '__main__':
    main()
