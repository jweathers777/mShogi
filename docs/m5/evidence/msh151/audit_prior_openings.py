#!/usr/bin/env python3
"""Offline audit of preserved M5 experiment roots. No engine processes or mutation."""
from pathlib import Path
from collections import defaultdict
import gzip
import hashlib
import json
import sys
import chess

REPO = Path('/Users/johnweathers/Development/Personal/mShogi')
BOOK = Path('/private/tmp/msh139/roots.json.gz')
sources = {}
def read_bytes(path):
    raw = path.read_bytes()
    sources[str(path)] = hashlib.sha256(raw).hexdigest()
    return raw

def key(root):
    board = chess.Board(root.get('fen') or chess.STARTING_FEN)
    for uci in root.get('moves', []):
        board.push_uci(uci)
    return ' '.join(board.fen(en_passant='legal').split()[:4])

book_raw = read_bytes(BOOK)
assert hashlib.sha256(book_raw).hexdigest() == '512b4ca59b50b8678169fdf3d73a87457962acd19b39483eaf5a33a111810a22'
book = json.loads(gzip.decompress(book_raw))
book_keys = {key(root): root for root in book}
assert len(book_keys) == len(book) == 34699
records = defaultdict(list)
def add(task, opening, path):
    records[task].append({'source': str(path), 'opening_id': opening['id'],
                          'final_root_key': key(opening)})

for task in ['msh49-2', 'msh139', 'msh144']:
    for path in sorted((REPO/'docs/m5/evidence'/task).rglob('config.json')):
        for root in json.loads(read_bytes(path)).get('openings', []):
            add(task, root, path)
for task in ['msh49', 'msh49-1']:
    for path in sorted((REPO/'docs/m5/evidence'/task).rglob('*.external.json.gz')):
        outer = json.loads(gzip.decompress(read_bytes(path)))
        assert hashlib.sha256(outer['report_json'].encode()).hexdigest() == outer['report_sha256']
        for root in json.loads(outer['report_json'])['config']['openings']:
            add(task, root, path)
for path in sorted((REPO/'docs/m5/evidence/msh48').glob('*report.json.gz')):
    for opening in json.loads(gzip.decompress(read_bytes(path)))['config']['openings']:
        record = opening['root']['record']
        if record['game_kind'] != 'chess':
            continue
        assert record['payload']['events'] == []
        add('msh48', {'id': opening['id'], 'fen': record['payload']['initial_fen']}, path)

ledger_reconciliation = {}
for task in ['msh139', 'msh144']:
    path = REPO/'docs/m5/evidence'/task/'verification.json'
    ledger = json.loads(read_bytes(path))
    ids = []
    for run in ledger['runs']:
        config_path = path.parent/run['folder']/'config.json'
        raw = read_bytes(config_path)
        assert hashlib.sha256(raw).hexdigest() == run['config_sha256']
        config = json.loads(raw)
        assert run['opening'] == config['openings'][0]['id']
        ids.append(run['opening'])
    assert ledger.get('inflight') is None
    ledger_reconciliation[task] = {'attempts': len(ids), 'unique_root_ids': sorted(set(ids)),
                                    'all_config_hashes_and_ids_match': True, 'inflight': None}

zero_additional_matches = {}
for task in ['msh136', 'msh137', 'msh138']:
    path = REPO/'docs/m5/evidence'/task/'verification.json'
    zero_additional_matches[task] = json.loads(read_bytes(path))['matches']

# Freeze provenance of the methods actually inspected, too.
for path in [REPO/'docs/m5/evidence/msh139/assessment.py',
             REPO/'docs/m5/chess-strength-assessment.md',
             REPO/'docs/m5/evidence/msh144/assessment.py',
             REPO/'docs/m5/chess-progress-assessment.md']:
    read_bytes(path)

unique = {task: {r['final_root_key'] for r in rows} for task, rows in records.items()}
pre139 = set().union(*(keys for task, keys in unique.items() if task not in ['msh139', 'msh144']))
all_keys = set().union(*unique.values())
excluded = {book_keys[k]['id'] for k in all_keys if k in book_keys}
remaining = [r for r in book if r['id'] not in excluded]
print(json.dumps({
    'command': f'{sys.executable} /private/tmp/msh151/audit_prior_openings.py',
    'python_chess_version': chess.__version__,
    'scope': 'Preserved M5 experiment initial roots, including smoke/incomplete and conservatively every configured MSH48 Chess root; excludes ordinary regression fixtures and later game descendants.',
    'normalization': 'Complete opening replay; board, side to move, castling rights, legally capturable en-passant; excludes counters. Shared opening prefixes are not identical final roots.',
    'source_sha256': sources,
    'canonical_book_roots': len(book),
    'msh139_method_pre139_exclusion': False,
    'per_task': {task: {'root_records': len(records[task]), 'unique_root_keys': len(keys),
                       'canonical_book_overlap_ids': sorted(book_keys[k]['id'] for k in keys if k in book_keys)}
                 for task, keys in unique.items()},
    'pre139_distinct_root_keys': len(pre139),
    'pre139_book_overlap': len(pre139 & book_keys.keys()),
    'all_distinct_root_keys': len(all_keys),
    'excluded_root_ids': sorted(excluded),
    'remaining_roots': len(remaining),
    'remaining_phases': {phase: sum(r['phase'] == phase for r in remaining) for phase in ['discovery', 'confirmation']},
    'ledger_reconciliation': ledger_reconciliation,
    'zero_additional_match_receipts': zero_additional_matches,
    'root_provenance': records,
}, indent=2))
