"""Compare deterministic Chu decisions through two actual native executables.

Usage: python3 -m tools.strength.native_signature ENGINE BASELINE OPENINGS SUITE OUT
This is fixed-work evidence, not a strength test. No time limit enters search.
"""
import hashlib
import json
import subprocess
import sys
from pathlib import Path


def run(engine, record, depth):
    request = dict(protocol=6, context=dict(
        purpose={'kind':'engine_turn'}, request_id=1, session_version=1,
        game=dict(envelope_version=2, game_kind='chu', payload=record),
        limits=dict(depth=depth, nodes=500000)))
    raw = json.dumps(request, separators=(',', ':')).encode()
    done = subprocess.run([engine, 'internal-search'], input=raw+b'\n',
                          capture_output=True, check=True, timeout=60)
    response = json.loads(done.stdout)
    assert response['protocol'] == 6
    assert response['request_digest'] == list(hashlib.sha256(raw).digest())
    result = response['result']
    assert result['kind'] == 'chu' and result['result']['kind'] == 'choice'
    assert result['result']['telemetry']['completed_depth'] > 0
    return result


def main():
    engine, baseline, openings, suite, out = sys.argv[1:]
    roots = json.loads(Path(openings).read_text())['openings'][:16]
    roots += json.loads(Path(suite).read_text())['positions'][::20]
    rows = []
    for row in roots:
        for depth in (1, 3):
            current = run(engine, row['record'], depth)
            previous = run(baseline, row['record'], depth)
            rows.append(dict(id=row['id'], depth=depth, identical=current == previous,
                             candidate=current, baseline=previous))
    with Path(out).open('x') as stream:
        json.dump({'rows': rows, 'identical': all(r['identical'] for r in rows)}, stream)
    assert all(r['identical'] for r in rows), 'fixed-work search differs'


if __name__ == '__main__':
    main()
