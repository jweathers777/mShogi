"""Extract the MSH-161 confirmed level-to-lost mShogi positions (one-time, saved evidence only).

Selection follows the independent review (docs/m5/evidence/msh161/cloud-primary/
independent-review-fable/scripts/build.py and a13.py): an mShogi move with
reference root E in [.30, .70], played E < .15 (500k screening), whose 20M
normalized status is ``supported_finite``. Prepared and unaided games are both kept.

Run from the repository root:
    python3 tools/strength/extract_blunders.py tools/strength/data/chess-blunders-msh161.json
The Git-ignored local evidence mirror (cloud-primary/local-cache) is required.
"""
import gzip
import hashlib
import json
import sys
from pathlib import Path

ROOT = Path('docs/m5/evidence/msh161')
PRIMARY = ROOT / 'cloud-primary'


def load(path):
    path = Path(path)
    return json.load(gzip.open(path)) if path.suffix == '.gz' else json.load(open(path))


def cp(score):
    if not score:
        return None
    kind, value = (score['kind'], score['value']) if isinstance(score, dict) else score
    return value if kind == 'cp' else (30000 if value > 0 else -30000)


def query(q):
    if q is None:
        return {}
    report = q.get('selected_report') or q.get('final_report') or {}
    return {'E': q.get('expected_score_mover'), 'best': q.get('bestmove'), 'cp': cp(report.get('score'))}


def main(output):
    coverage = load(PRIMARY / 'final-analysis/played-coverage.json.gz')
    phases = load(PRIMARY / 'final-analysis/phase-summary.json.gz')
    cumulative = load(PRIMARY / 'primary-cumulative.json')
    selection = load(ROOT / 'sources/main-selection.json')
    family = {r['id']: r['family'] for r in selection['selected_roots']}
    phase = {(r['pair'], r['game'], r['ply']): r for r in phases['played_roots']}
    cov = {(r['pair'], r['game'], r['ply']): r for r in coverage['played_roots']}
    results = {(g['pair'], g['game']): g for g in cumulative['native_games']}
    sources = sorted({(r['pair'], r['screen_source']['source'], r['screen_source']['sha256'])
                      for r in coverage['played_roots']})
    positions = []
    for pair, source, sha in sources:
        raw = (PRIMARY / source).read_bytes()
        if hashlib.sha256(raw).hexdigest() != sha:
            raise SystemExit('screen artifact hash mismatch: ' + source)
        for d in json.loads(raw)['decisions']:
            dd = d['decision']
            key = (pair, dd['game'], dd['ply'])
            if dd['origin'] != 'played':
                continue
            ph = phase[key]
            if not ph['candidate']:
                continue
            played, best = query(d['queries'].get('played')), query(d['queries'].get('unrestricted'))
            if best.get('E') is None or played.get('E') is None:
                continue
            if not (.30 <= best['E'] <= .70 and played['E'] < .15):
                continue
            norm = (cov.get(key) or {}).get('normalized_selected_row') or {}
            if norm.get('status') != 'supported_finite':
                continue
            game = results[(pair, dd['game'])]
            reference = norm.get('alternative') or best['best']
            if reference == dd['played']:
                raise SystemExit('reference equals blunder at %s' % (key,))
            positions.append({
                'id': '%s/g%d/p%d' % key,
                'pair': pair, 'game': dd['game'], 'ply': dd['ply'], 'mover': dd['mover'],
                'family': family.get(dd.get('opening_id'), 'unaided'),
                'phase': ph['phase'], 'material': ph['material'],
                'initial_fen': dd['initial_fen'], 'history': dd['history'],
                'blunder_move': dd['played'], 'reference_move': reference,
                'screen_500k': {'root_E': best['E'], 'played_E': played['E'],
                                'root_cp': best['cp'], 'played_cp': played['cp'],
                                'screen_best': best['best']},
                'confirmation_20m': {'status': norm.get('status'), 'severity': norm.get('severity'),
                                     'margin': norm.get('margin'), 'alternative': norm.get('alternative')},
                'game_result_for_mshogi': game['candidate_result'],
                'native_report': cov[key]['native_report'],
                'screen_source': {'source': source, 'sha256': sha},
            })
    positions.sort(key=lambda p: (p['pair'], p['game'], p['ply']))
    suite = {
        'schema': 1,
        'name': 'chess-blunders-msh161',
        'status': 'development only: positions are exposed by MSH-161 and are not held-out evidence',
        'selection': 'mShogi move, 500k root E in [.30,.70], played E < .15, 20M status supported_finite',
        'sources': {
            'played_coverage_sha256': hashlib.sha256((PRIMARY / 'final-analysis/played-coverage.json.gz').read_bytes()).hexdigest(),
            'phase_summary_sha256': hashlib.sha256((PRIMARY / 'final-analysis/phase-summary.json.gz').read_bytes()).hexdigest(),
            'primary_cumulative_sha256': hashlib.sha256((PRIMARY / 'primary-cumulative.json').read_bytes()).hexdigest(),
        },
        'count': len(positions),
        'positions': positions,
    }
    Path(output).write_text(json.dumps(suite, indent=1) + '\n')
    print(len(positions), 'positions')


if __name__ == '__main__':
    main(sys.argv[1])
