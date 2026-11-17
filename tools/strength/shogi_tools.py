"""Shogi measurements through the shared native protocol, never Python rules."""
import gzip
import json
import os
from pathlib import Path
import signal
import statistics
import subprocess
import tempfile

from .cli import DATA, engine_identity, environment, load_json, write_new
from .uci import file_sha256

HELPER = 'target/release/examples/shogi_strength'


def native(mode, path, helper=HELPER, engine=None, time_ms=0, timeout=900):
    with tempfile.TemporaryDirectory(prefix='mshogi-strength-') as temp:
        output = Path(temp)/'result.json'
        command = [str(Path(helper).resolve()), mode, str(Path(path).resolve()), str(output)]
        if mode == 'probe':
            command += [str(Path(engine).resolve()), str(time_ms)]
        process = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                   text=True, start_new_session=True)
        failure = None
        try:
            stdout, stderr = process.communicate(timeout=timeout)
        except BaseException as error:
            os.killpg(process.pid, signal.SIGKILL)
            stdout, stderr = process.communicate(timeout=5)
            failure = repr(error)
        # The helper owns its search child. A surviving process group is never
        # a successful probe, even when the helper has already exited.
        try:
            os.killpg(process.pid, 0)
        except ProcessLookupError:
            pass
        except PermissionError as error:
            failure = failure or f'cannot verify probe process cleanup: {error}'
        else:
            os.killpg(process.pid, signal.SIGKILL)
            failure = failure or 'residual native probe process group'
        if process.returncode or failure:
            folder = DATA.parents[2]/'target/strength/failures'
            folder.mkdir(parents=True, exist_ok=True)
            with tempfile.NamedTemporaryFile(mode='w', prefix='shogi-', suffix='.json', dir=folder, delete=False) as log:
                json.dump({'command':command,'returncode':process.returncode,'failure':failure,
                           'stdout_tail':stdout[-8000:],'stderr_tail':stderr[-8000:]}, log, indent=2)
            raise RuntimeError(f'native {mode} failed; retained {log.name}: {stderr[-8000:]}')
        return output.read_text()


def bench(args):
    path = args.positions or str(DATA/'shogi-bench-v1.json')
    spec = load_json(path)
    if not 1 <= args.repeat <= 20:
        raise ValueError('repeat must be 1..20')
    if not 0 < args.timeout <= 900:
        raise ValueError('timeout must be positive and at most 900 seconds per catalog round')
    time_ms = getattr(args, 'movetime', None) or 0
    if getattr(args, 'movetime', None) is not None and not 1 <= time_ms <= 60_000:
        raise ValueError('movetime must be 1..60000 ms')
    native('validate', path, args.helper, timeout=args.timeout)
    rounds = [json.loads(native('probe', path, args.helper, args.engine, time_ms, timeout=args.timeout)) for _ in range(args.repeat)]
    positions = []
    for i, p in enumerate(spec['positions']):
        rows = [r['positions'][i] for r in rounds]
        positions.append({'id': p['id'], 'depth': p['depth'] if not time_ms else None,
                          'median_completed_depth': statistics.median(r['depth'] for r in rows),
                          'median_seconds_to_depth': None if time_ms else statistics.median(r['wall_seconds'] for r in rows),
                          'median_request_seconds': statistics.median(r['wall_seconds'] for r in rows),
                          'nodes': [r['nodes'] for r in rows],
                          'median_nps': statistics.median(r['nodes']/r['wall_seconds'] for r in rows)})
    seconds = [sum(p['wall_seconds'] for p in r['positions']) for r in rounds]
    nodes = [sum(p['nodes'] for p in r['positions']) for r in rounds]
    result = {'kind':'mshogi-strength-bench', 'game':'shogi', 'schema':1, 'set':spec['name'],
              'mode':'time-budget' if time_ms else 'fixed-depth', 'movetime_ms':time_ms or None,
              'positions_sha256':file_sha256(path), 'engine':engine_identity(args.engine),
              'helper':engine_identity(args.helper), 'environment':environment(),
              'summary':{'bench_nodes':nodes, 'median_seconds':statistics.median(seconds),
                         'median_nps':statistics.median(n/t for n,t in zip(nodes,seconds)),
                         'deterministic_nodes':(all(len({r['positions'][i]['nodes'] for r in rounds})==1
                                                    for i in range(len(positions)))
                                                if len(rounds) > 1 else None)},
              'timing':rounds[0]['timing'], 'per_position':positions, 'rounds':rounds}
    if args.out:
        write_new(args.out, result)
    print(json.dumps(result['summary'],indent=2))
    return result


def tactics(args):
    if not 1 <= args.movetime <= 60_000:
        raise ValueError('movetime must be 1..60000 ms')
    path = args.suite or str(DATA/'shogi-tactics-v1.json')
    spec = load_json(path)
    validation = json.loads(native('validate',path,args.helper))
    measured = json.loads(native('probe',path,args.helper,args.engine,args.movetime))
    for p,r in zip(spec['positions'],measured['positions']):
        r.update(expected_moves=p['expected_moves'],matched_expected=r['bestmove'] in p['expected_moves'],
                 geometry=p['geometry'],phase=p['phase'])
    result={'kind':'mshogi-strength-tactics','game':'shogi','schema':1,'suite':spec['name'],
            'suite_sha256':file_sha256(path),'engine':engine_identity(args.engine),
            'helper':engine_identity(args.helper),'environment':environment(),'movetime_ms':args.movetime,
            'metric':'matches an independently authored, native-proven short checking mate; not all winning moves or held-out strength',
            'summary':{'positions':len(measured['positions']),
                       'matched_expected':sum(r['matched_expected'] for r in measured['positions']),
                       'independent_geometries':len({p['geometry'] for p in spec['positions']}),
                       'median_depth':statistics.median(r['depth'] for r in measured['positions'])},
            'validation':validation,'results':measured['positions'],'timing':measured['timing']}
    if args.out:
        write_new(args.out,result)
    print(json.dumps(result['summary'],indent=2))
    return result


def validate_data(args):
    paths = [args.path] if args.path else [str(DATA/f'shogi-{part}-v1.json') for part in ('bench','tactics','openings')]
    results=[{'path':p,'sha256':file_sha256(p),**json.loads(native('validate',p,args.helper))} for p in paths]
    result={'kind':'shogi-data-validation','environment':environment(),'helper':engine_identity(args.helper),'sets':results}
    if args.out:
        write_new(args.out,result)
    print(json.dumps({p['path']:len(p['validated']) for p in results},indent=2))
    return result


def export(args):
    source=Path(args.reports)
    reports=sorted(source.glob('pair-*.report.json.gz'))
    if not reports:
        raise ValueError('no native paired reports found')
    out=Path(args.out);out.mkdir(parents=True,exist_ok=False)
    inputs={};games=[]
    for i,path in enumerate(reports):
        raw=gzip.decompress(path.read_bytes())
        with tempfile.TemporaryDirectory(prefix='mshogi-texel-') as temp:
            report=Path(temp)/'report.json';report.write_bytes(raw)
            data=native('export',report,args.helper)
        target=out/f'games-{i:04}.jsonl';target.write_text(data)
        inputs[str(path)]={'sha256':file_sha256(path),'output_sha256':file_sha256(target)}
        games.extend(json.loads(line) for line in data.splitlines())
    manifest={'kind':'shogi-native-texel-export','schema':1,'environment':environment(),
              'helper':engine_identity(args.helper),'inputs':inputs,'games':len(games),
              'labeled_games':sum(g['sente_outcome'] is not None for g in games),
              'positions':sum(len(g['samples']) for g in games),
              'conditions':'Native replay before export; one before-position every four accepted events; native sente result labels only; incomplete games have no samples.',
              'rights':'Project-generated native self-play, BSD-LICENSE; retain opening source notices',
              'training_status':'generation demonstration, not a fitted evaluator or a strength claim'}
    write_new(out/'export-manifest.json',manifest)
    print(json.dumps({k:manifest[k] for k in ('games','labeled_games','positions')},indent=2))
    return manifest
