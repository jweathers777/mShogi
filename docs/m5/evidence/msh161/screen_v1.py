#!/usr/bin/env python3
"""Read-only versioned screen; earlier aligned nonbound reports are NOT findings.
Usage: python screen_v1.py ANALYSIS_DIRECTORY OUTPUT_JSON
Preserves v0, all raw receipts and every uncertain/negative comparison.
"""
import argparse
from collections import Counter
from pathlib import Path
from analysis import decode, digest, expected_score, save

STABILITY = .025
CP_CONFLICT = 20


def describe(value):
    assessment = value.get('assessment') or {}
    selected = assessment.get('score')
    best = assessment.get('bestmove')
    flags = list(assessment.get('uncertainty', []))
    if value.get('status') != 'complete': flags.append('query_not_complete')
    if not value.get('pv_independent_legal', False): flags.append('pv_not_verified_legal')
    infos = value.get('score_updates', [])
    aligned = [x for x in infos if x.get('pv', [None])[0] == best
               and not x.get('lowerbound') and not x.get('upperbound') and 'wdl' in x]
    latest, depths = [], set()
    for x in reversed(aligned):
        if x.get('depth') is not None and x['depth'] not in depths:
            latest.append(x); depths.add(x['depth'])
        if len(latest) == 2: break
    estimates = [expected_score(x['wdl']) for x in latest]
    spread = max(estimates) - min(estimates) if len(estimates) == 2 else None
    numeric = (expected_score(selected['wdl']) if selected and 'wdl' in selected
               and selected.get('pv', [None])[0] == best
               and not selected.get('lowerbound') and not selected.get('upperbound') else None)
    if numeric is None: flags.append('no_aligned_nonbound_numeric_estimate')
    conflicts = []
    final = infos[-1] if infos else None
    if final and final.get('pv', [None])[0] == best and selected:
        bound = 'upper' if final.get('upperbound') else 'lower' if final.get('lowerbound') else None
        if bound:
            if numeric is not None and 'wdl' in final:
                edge = expected_score(final['wdl'])
                if (bound == 'upper' and edge < numeric-STABILITY) or (bound == 'lower' and edge > numeric+STABILITY):
                    conflicts.append('final_same_move_bound_conflicts_with_earlier_expected_score')
            oldscore, newscore = selected.get('score', {}), final.get('score', {})
            if oldscore.get('kind') == newscore.get('kind') == 'cp':
                old, new = oldscore['value'], newscore['value']
                if (bound == 'upper' and new < old-CP_CONFLICT) or (bound == 'lower' and new > old+CP_CONFLICT):
                    conflicts.append('final_same_move_bound_conflicts_with_earlier_cp')
            elif oldscore != newscore:
                conflicts.append('bound_mate_or_score_kind_change_requires_review')
    if spread is None: flags.append('fewer_than_two_distinct_depth_numeric_reports')
    elif spread > STABILITY: flags.append('last_two_report_estimates_unstable')
    flags.extend(conflicts)
    return {'bestmove': best, 'expected_score_mover': numeric, 'selected_report': selected,
            'last_two_distinct_depth_reports': list(reversed(latest)),
            'last_two_expected_score_range': [min(estimates), max(estimates)] if estimates else None,
            'last_two_spread': spread, 'stable_report_heuristic': spread is not None and spread <= STABILITY,
            'final_report': final, 'directional_bound_conflicts': conflicts,
            'flags': sorted(set(flags)), 'status': value.get('status')}


def summarize(directory):
    directory = Path(directory).resolve()
    if (directory/'inflight.json').exists(): raise ValueError('Inflight query; wait for completed batch')
    manifest = decode((directory/'manifest.json').read_text())
    latest = {}
    for path in sorted(directory.glob('g*-attempt*.json')):
        value = decode(path.read_text())
        if value['engine_sha256'] != manifest['engine_sha256'] or value['nodes_requested'] != manifest['nodes'] or value['options'] != manifest['options']:
            raise ValueError('Query identity does not match manifest')
        latest[value['query']] = (path, value)
    grouped = {}
    for query, (path, value) in latest.items():
        d = value['decision']; key = (d['game'], d['ply'])
        group = grouped.setdefault(key, {'decision': d, 'queries': {}})
        if group['decision'] != d: raise ValueError('Different roots/history for same game/ply')
        group['queries'][query.rsplit('-', 1)[1]] = (path, value)
    rows = []
    for key, group in sorted(grouped.items()):
        queries = group['queries']; detail = {kind: describe(value) for kind, (_, value) in queries.items()}
        a = detail.get('unrestricted', {}).get('expected_score_mover')
        b = detail.get('played', {}).get('expected_score_mover')
        loss = a-b if a is not None and b is not None else None
        flags = sorted(set(flag for d in detail.values() for flag in d['flags']))
        reasons = []
        if loss is not None and loss >= .05: reasons.append('raw_loss_at_least_005')
        if loss is not None and loss < 0: reasons.append('negative_raw_loss')
        if flags: reasons.append('uncertain_report_or_depth_evidence')
        if not manifest.get('roots_input') and loss is None: reasons.append('numeric_comparison_unavailable')
        rows.append({'decision': group['decision'], 'query_refs': {kind: {'path': str(path), 'sha256': digest(path)} for kind, (path, _) in queries.items()},
                     'queries': detail, 'raw_expected_score_loss': loss,
                     'flags': flags, 'followup_reasons': reasons,
                     'followup_required': bool(reasons), 'confirmed': False,
                     'interpretation': 'Finite screening estimate only; retained uncertainty is not cleared.'})
    counts = Counter()
    for row in rows:
        counts['all_decisions_with_receipts'] += 1
        counts['prepared_decisions' if row['decision']['origin']=='prepared' else 'supplied_roots' if row['decision']['origin']=='supplied_root' else 'played_decisions'] += 1
        counts['numeric_comparisons' if row['raw_expected_score_loss'] is not None else 'numeric_comparisons_unavailable'] += 1
        counts['followup_required'] += row['followup_required']
        for reason in row['followup_reasons']: counts[reason] += 1
    expected = manifest['planned_decisions'] * (1 if manifest.get('roots_input') or manifest.get('move') else 2)
    return {'schema': 1, 'interpretation_version': 'pilot-screen-v1',
            'script_sha256': digest(__file__), 'manifest_ref': {'path': str(directory/'manifest.json'), 'sha256': digest(directory/'manifest.json')},
            'thresholds': {'raw_loss_followup': .05, 'distinct_depth_expected_score_stability': STABILITY, 'supporting_cp_bound_conflict': CP_CONFLICT},
            'planned_decisions': manifest['planned_decisions'], 'planned_queries': expected,
            'observed_latest_queries': len(latest), 'complete_query_coverage': len(latest)==expected and all(v['status']=='complete' for _,v in latest.values()),
            'counts': dict(counts),
            'limitations': ['No flags are removed from raw receipts or v0 summaries.', 'Nonbound UCI reports do not prove completed iterations or game-theoretic values.', 'Last-two-report stability and directional-bound conflict tests are heuristics, not statistical intervals.', 'All threshold, uncertain and negative rows enter follow-up; no row is confirmed or assigned a chess weakness here.', 'Prepared moves and supplied roots are never attributed to competitors.'],
            'decisions': rows}


def main():
    parser = argparse.ArgumentParser(description=__doc__); parser.add_argument('directory');parser.add_argument('output');args=parser.parse_args()
    destination = Path(args.output)
    if destination.exists(): raise FileExistsError('Preserve versioned summaries; choose a fresh output path')
    save(destination, summarize(args.directory))

if __name__ == '__main__': main()
