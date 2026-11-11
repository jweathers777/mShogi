#!/usr/bin/env python3
"""Read recorded game-generation progress once; no engines are launched."""
import argparse
import json
from pathlib import Path

DEFAULT_ROOT = Path(__file__).resolve().parents[2] / 'docs/m5/evidence/msh161'

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=DEFAULT_ROOT,
                        help='Study evidence directory (default: this checkout, MSH-161)')
    args = parser.parse_args()
    total = 0
    for path in sorted((args.root / 'main').glob('batch-*/games/batch.json')):
        try:
            data = json.loads(path.read_text())
        except (OSError, ValueError) as exc:
            print(f'{path.parents[1].name}: cannot read state: {exc}')
            continue
        done = sum(p.get('summary', {}).get('games', 0)
                   for p in data['pairs'] if p.get('complete'))
        active = data.get('inflight')
        total += done
        print(f'{path.parents[1].name}: {done}/20 games complete'
              + (f"; recorded active pair: {active['id']}" if active else ''))
    print(f'Main study: {total}/200 games recorded complete')
    print('Game counts exclude subsequent analysis and review; saved state is not a process-liveness check.')

if __name__ == '__main__':
    main()
