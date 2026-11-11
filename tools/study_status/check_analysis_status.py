#!/usr/bin/env python3
"""Read saved Chess study progress once; does not launch or alter computations."""
import argparse
import json
from datetime import datetime
from pathlib import Path

DEFAULT_ROOT = Path(__file__).resolve().parents[2] / 'docs/m5/evidence/msh161'

def read(path):
    try:
        return json.loads(path.read_text())
    except FileNotFoundError:
        return {}
    except (OSError, ValueError) as exc:
        print(f'  Cannot read {path.name}: {exc}')
        return {}

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('batch', nargs='?', type=int, default=1)
    parser.add_argument('--root', type=Path, default=DEFAULT_ROOT,
                        help='Study evidence directory (default: this checkout, MSH-161)')
    args = parser.parse_args()
    root = args.root.resolve()
    name = f'batch-{args.batch:02d}'
    directory = root / 'main' / name
    chain = read(directory / 'chain.json')
    if not chain:
        print(f'{name}: no readable chain state yet')
        return
    print(f'{name}: {chain.get("status", "unknown")} (saved state, not a process-liveness check)')
    print('Current step:', chain.get('inflight', {}).get('label', 'none recorded'))
    completed = chain.get('completed', [])
    steps = {item['label']: item for item in completed}
    native = steps.get('native20games')
    print('Game generation:', ('finished successfully' if native['returncode'] == 0 else 'FAILED') if native else 'not recorded complete')
    pairs = read(root / 'main' / f'{name}-schedule.json').get('pairs', [])
    finished = 0
    for pair in pairs:
        identifier = pair['id']
        analysis = directory / 'games' / identifier / 'analysis'
        manifest = read(analysis / 'manifest.json')
        if not manifest:
            continue
        usage = read(analysis / 'resources.json')
        total = manifest.get('planned_decisions', 0) * 2
        attempts = usage.get('attempts', 0)
        failures = usage.get('failures', 0)
        step = steps.get(identifier + '-allmove')
        done = step is not None and step['returncode'] == 0
        finished += done
        print(f'  {identifier}: {attempts - failures}/{total} successful screening queries; {failures} failed attempts' + (' [screen finished]' if done else ''))
        inflight = read(analysis / 'inflight.json')
        if inflight:
            decision = inflight.get('decision', {})
            detail = f'game {decision.get("game", "?")}, ply {decision.get("ply", "?")}, move {inflight.get("forced_move") or "unrestricted"}'
            print('    Recorded query:', inflight.get('query', detail))
    print(f'All-move screening jobs finished: {finished}/{len(pairs)}')
    confirmations = read_list(directory / 'confirmation-results.json')
    plan = read_list(directory / 'confirmation-plan.json')
    if (directory / 'confirmation-plan.json').exists():
        print(f'Confirmation decisions recorded: {len(confirmations)}/{len(plan)}')
    else:
        print('Confirmations: plan not recorded yet (prepared after all screening jobs)')
    notification = read(root / 'notifications' / f'main-{name}.json')
    print('Wake-up queue attempted:', notification.get('queue_attempted', 'not recorded'))
    print('Wake-up queue accepted:', notification.get('queue_accepted', 'not recorded'))
    modified = datetime.fromtimestamp((directory / 'chain.json').stat().st_mtime).astimezone()
    print('Chain last updated:', modified.isoformat(timespec='seconds'))

def read_list(path):
    value = read(path)
    return value if isinstance(value, list) else []

if __name__ == '__main__':
    main()
