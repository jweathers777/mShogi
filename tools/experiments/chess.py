"""Chess study adapter over the frozen native runner and reference interpreter.

Execution/storage stay outside this module. No game outcome or score interpretation
is invented here: the frozen helpers remain the authorities for this study.
"""
import gzip
import hashlib
import importlib.util
import json
import math
from pathlib import Path
import subprocess
import sys
from collections import defaultdict


def _save(path, value):
    path.write_text(json.dumps(value, indent=2, sort_keys=True, allow_nan=False) + '\n')


def _sha(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()


def _modules(repo):
    directory = repo / 'docs/m5/evidence/msh161'
    freeze = json.loads((directory / 'main-freeze.json').read_text())
    names = ('run_batch', 'analysis', 'screen_v1', 'phase_tags', 'summarize_analysis')
    # Source hashes are the study's own immutable declarations. Caller also pins
    # the repository revision and job input hashes; no machine-local build used.
    for name in names:
        if _sha(directory / (name + '.py')) != freeze['files'][name + '.py']:
            raise ValueError('Frozen study helper changed: ' + name)
    modules = {}
    previous = {name: sys.modules.get(name) for name in names}
    try:
        for name in names:
            spec = importlib.util.spec_from_file_location(name, directory / (name + '.py'))
            module = importlib.util.module_from_spec(spec)
            sys.modules[name] = module
            spec.loader.exec_module(module)
            modules[name] = module
    finally:
        for name, old in previous.items():
            if old is None:
                sys.modules.pop(name, None)
            else:
                sys.modules[name] = old
    return modules


def _check_build(build, required):
    if not isinstance(build.get('source'), str) or not build['source']:
        raise ValueError('Missing native source revision')
    for key in required:
        engine = build['engines'][key]
        if not Path(engine['path']).is_absolute() or _sha(engine['path']) != engine['sha256']:
            raise ValueError('Executable identity mismatch: ' + key)


def _claim_policy(job, repo):
    """Bind the owner-authorized policy amendment without rewriting old evidence."""
    policy = job.get('claim_policy', 'auto_claim')
    ref = job.get('study_amendment')
    if ref is None:
        if policy != 'auto_claim':
            raise ValueError('Changed claim policy requires a bound study amendment')
        return policy
    path = (Path(repo) / ref['path']).resolve()
    path.relative_to(Path(repo).resolve())
    if job['source_files'].get(ref['path']) != ref['sha256'] or _sha(path) != ref['sha256']:
        raise ValueError('Study amendment hash mismatch')
    amendment = json.loads(path.read_text())
    if (amendment.get('schema') != 'msh161-automatic-thresholds-v1'
            or policy != 'explicit_only' or amendment.get('claim_policy') != policy
            or amendment.get('uci_optional_claims_available') is not False
            or amendment.get('original_study_freeze_sha256') != job['study_freeze_sha256']):
        raise ValueError('Unsupported study policy amendment')
    source = amendment['native_source']
    if job['build']['source'] != source or any(
            job['build']['engines'][name]['source'] != source for name in ('candidate', 'runner')):
        raise ValueError('Native source differs from policy amendment')
    return policy


def _configure_native(native, policy):
    # _modules supplies a fresh, hash-verified module for each attempt. Retain its
    # supervision/audit code; only the versioned emitted configuration changes.
    if policy == 'explicit_only':
        original = native.config
        def config(build, spec):
            result = original(build, spec)
            result['claim_policy'] = policy
            return result
        native.config = config


def _replay(report, runner, output):
    # Native replay expects uncompressed JSON; retain exact bytes in the attempt.
    replay_input = output / 'replay-input.json'
    opener = gzip.open if str(report).endswith('.gz') else open
    with opener(report, 'rb') as stream:
        raw = stream.read(128 * 1024 * 1024 + 1)
    if len(raw) > 128 * 1024 * 1024:
        raise ValueError('Native report exceeds bounded replay input')
    replay_input.write_bytes(raw)
    with (output / 'replay.stdout').open('wb') as stdout, (output / 'replay.stderr').open('wb') as stderr:
        result = subprocess.run([runner, 'experiment-external-replay', str(replay_input)],
                                stdout=stdout, stderr=stderr, timeout=60, check=False)
    _save(output / 'replay.json', {'returncode': result.returncode, 'input_sha256': _sha(report),
                                 'uncompressed_sha256': _sha(replay_input)})
    if result.returncode:
        raise RuntimeError('Native replay rejected the input report')
    return replay_input


def confirmation_plan(pair, screen, decisions, games, phase_tags):
    """Exact frozen main_batch selection: all flags plus stratified 10% controls."""
    origins = {d['game']: d.get('history_origin', 'unknown') for d in decisions}
    phases = {g['game']: phase_tags.tag_history(g['initial_fen'], g['complete_history'],
                                               origins.get(g['game'], 'unknown')) for g in games}
    cells, selected = defaultdict(list), {}
    for row in screen['decisions']:
        dec = row['decision']
        if dec['origin'] != 'played':
            continue
        key = (dec['game'], dec['ply'])
        phase = phases[dec['game']]['positions'][dec['ply']]['phase']
        if row['followup_required']:
            selected[key] = (row, 'v1-followup', phase)
        else:
            cells[(dec['game'], dec['engine_index'], phase)].append(row)
    for rows in cells.values():
        ranked = sorted(rows, key=lambda r: hashlib.sha256(
            ('msh161-main-audit-v1/' + pair + '/' + str(r['decision']['game']) + '/' +
             str(r['decision']['ply'])).encode()).hexdigest())
        for row in ranked[:max(1, math.ceil(len(rows) * .10))]:
            dec = row['decision']
            phase = phases[dec['game']]['positions'][dec['ply']]['phase']
            selected[(dec['game'], dec['ply'])] = (row, 'stratified-nonflagged-audit', phase)
    result = []
    for _, (row, reason, phase) in sorted(selected.items()):
        dec = row['decision']
        result.append({'pair': pair, 'game': dec['game'], 'ply': dec['ply'], 'played': dec['played'],
                       'alternative': row['queries'].get('unrestricted', {}).get('bestmove'),
                       'phase': phase, 'reason': reason, 'initial_flags': row['flags'],
                       'initial_raw_loss': row['raw_expected_score_loss']})
    return result, phases


def execute(job, output, inputs, repo):
    """Execute a fresh attempt; failed work remains evidence, never a result."""
    if job.get("package_only"):
        raise ValueError("Package-only job cannot execute")
    output, repo = Path(output), Path(repo)
    output.mkdir(parents=True, exist_ok=True)
    modules = _modules(repo)
    native, analysis, screen = (modules[k] for k in ('run_batch', 'analysis', 'screen_v1'))
    policy = _claim_policy(job, repo)
    _configure_native(native, policy)
    kind, build = job['kind'], job['build']
    if kind == 'game_pair':
        spec = job['spec']
        _check_build(build, ('runner', 'candidate', spec['opponent']))
        receipt = native.run_pair(build, spec, output / 'pair')
        if not receipt.get('complete'):
            raise RuntimeError('Native pair incomplete; preserved failed attempt')
        return {'complete': True, 'report': 'pair/report.json.gz', 'native_receipt': receipt}
    if kind not in ('reference_screening', 'confirmation'):
        raise ValueError('Unsupported Chess job kind')
    _check_build(build, ('runner', 'stockfish19'))
    nodes = job.get('nodes', 500000 if kind == 'reference_screening' else 5000000)
    if type(nodes) is not int or not 1 <= nodes <= 20000000:
        raise ValueError('Invalid frozen analysis node limit')
    if kind == 'reference_screening' and nodes != 500000:
        raise ValueError('Screening must preserve the frozen 500000-node setting')
    report = Path(inputs['report'])
    replay = _replay(report, build['engines']['runner']['path'], output)
    native_report = analysis.load_report(replay)
    if native_report['config'].get('claim_policy', 'auto_claim') != policy:
        raise ValueError('Native report claim policy differs from job')
    # Replay legality alone is not complete competitive evidence.
    if len(native_report['games']) != 2 or any(
        g.get('failure') or g.get('diagnostic_failures') or g.get('stop') != 'chess_conclusion'
        for g in native_report['games']
    ):
        raise ValueError('Analysis requires a complete healthy native pair')
    decisions, games = analysis.extract_decisions(native_report)
    if kind == 'confirmation':
        if type(job.get('game')) is not int or type(job.get('ply')) is not int:
            raise ValueError('Confirmation requires exact game and ply')
        decisions = [d for d in decisions if d['game'] == job['game'] and d['ply'] == job['ply']]
        if len(decisions) != 1:
            raise ValueError('Confirmation selector must identify exactly one decision')
        if job.get('move') is not None:
            analysis.check_move(job['move'])
    if not decisions or any(g.get('missing_match_record') for g in games):
        raise ValueError('Missing complete native histories')
    directory = output / 'analysis'
    directory.mkdir()
    engine = build['engines']['stockfish19']
    import chess as python_chess
    manifest = {'python_chess_version': python_chess.__version__, 'schema': 1, 'report_sha256': _sha(report), 'source_revision': build['source'],
                'engine_sha256': engine['sha256'], 'engine_path': engine['path'], 'nodes': nodes,
                'options': analysis.OPTIONS, 'timeout_seconds': 60, 'game': job.get('game'),
                'ply': job.get('ply'), 'move': job.get('move'), 'roots_input': False,
                'native_replay_required': True, 'games': games, 'planned_decisions': len(decisions),
                'script_sha256': _sha(repo / 'docs/m5/evidence/msh161/analysis.py')}
    _save(directory / 'manifest.json', manifest)
    receipts = []
    for decision in decisions:
        queries = ([('alternative', job['move'])] if job.get('move') else
                   [('unrestricted', None), ('played', decision['played'])])
        for label, forced in queries:
            # Recheck before every process launch; never accept replaced binaries.
            _check_build(build, ('stockfish19',))
            stem = f'g{decision["game"]:03d}-p{decision["ply"]:03d}-{label}'
            inflight = directory / 'inflight.json'
            value = analysis.query(Path(engine['path']), decision, nodes, forced, inflight)
            value.update(query=stem, engine_sha256=engine['sha256'], nodes_requested=nodes)
            _save(directory / (stem + '-attempt0000.json'), value)
            if value.get('process_group_settled'):
                inflight.unlink(missing_ok=True)
            receipts.append(value)
            _save(directory / 'resources.json', {
                'attempts': len(receipts), 'wall_seconds': sum(r['wall_seconds'] for r in receipts),
                'child_cpu_seconds': sum(r['child_cpu_seconds'] for r in receipts),
                'failures': sum(r['status'] != 'complete' for r in receipts)})
            if value['status'] != 'complete' or not value.get('process_group_settled'):
                raise RuntimeError('Reference query failed; preserved failed attempt')
    _save(output / 'summary-v0.json', modules['summarize_analysis'].summarize(directory))
    summary = screen.summarize(directory)
    _save(output / 'screen-v1.json', summary)
    if not summary['complete_query_coverage']:
        raise RuntimeError('Incomplete all-move query coverage')
    if kind == 'reference_screening':
        plan, phases = confirmation_plan(native_report['config']['id'], summary, decisions, games,
                                         modules['phase_tags'])
        _save(output / 'confirmation-plan.json', plan)
        _save(output / 'phases.json', phases)
    return {'complete': True, 'analysis': 'analysis', 'screen': 'screen-v1.json',
            'decisions': len(decisions), 'queries': len(receipts),
            'confirmation_plan': 'confirmation-plan.json' if kind == 'reference_screening' else None}
