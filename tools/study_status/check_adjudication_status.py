#!/usr/bin/env python3
"""Read saved deeper-adjudication progress once, without changing the study."""
import argparse
import json
from datetime import datetime
from pathlib import Path

DEFAULT_ROOT = Path(__file__).resolve().parents[2] / 'docs/m5/evidence/msh161'

def read(path, default=None):
    try:
        return json.loads(path.read_text())
    except FileNotFoundError:
        return {} if default is None else default
    except (OSError, ValueError) as exc:
        print(f'Cannot read {path}: {exc}')
        return {} if default is None else default

def stamp(epoch):
    return datetime.fromtimestamp(epoch).astimezone().isoformat(timespec='seconds')

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('batch', type=int, nargs='?', default=1)
    parser.add_argument('--root', type=Path, default=DEFAULT_ROOT,
                        help='Study evidence directory (default: this checkout, MSH-161)')
    args = parser.parse_args()
    root = args.root.resolve()
    batch = f'batch-{args.batch:02d}'
    base = root / 'main' / batch
    notification = read(root / 'notifications' / f'main-{batch}-adjudication.json')
    statepath = Path(notification.get('state_artifact', str(base / 'adjudication20m' / 'chain.json')))
    state = read(statepath)
    plan = read(base / 'adjudication-plan.json', [])
    if not state:
        print(f'{batch}: no readable adjudication state yet')
        return
    results = state.get('results', [])
    completed = state.get('completed', [])
    print(f'{batch} deeper adjudication: {state.get("status", "unknown")}')
    print(f'Positions fully processed: {len(results)}/{len(plan)}')
    print('Finished query jobs:', len(completed))
    print('Failed query jobs:', sum(item.get('returncode', 0) != 0 for item in completed))
    print(f'Node limit per query: {state.get("nodes", 0):,}')
    print(f'Recorded finished-job wall time: {sum(item.get("wall_seconds", 0) for item in completed) / 60:.1f} minutes')
    inflight = state.get('inflight', {})
    command = inflight.get('command', [])
    if len(command) >= 4:
        destination = Path(command[3])
        print('Current recorded job:', destination.name)
        if inflight.get('started_epoch'):
            print('Job started:', stamp(inflight['started_epoch']))
        query = read(destination / 'inflight.json')
        if query:
            decision = query.get('decision', {})
            print(f'Current query: game {decision.get("game", "?")}, ply {decision.get("ply", "?")}, move {query.get("forced_move") or "unrestricted"}')
    if state.get('error'):
        print('Recorded error:', state['error'])
    print('Wake-up queue attempted:', notification.get('queue_attempted', 'not recorded'))
    print('Wake-up queue accepted:', notification.get('queue_accepted', 'not recorded'))
    print('State last updated:', stamp(statepath.stat().st_mtime))
    print('Saved progress only; running status does not prove process liveness.')
    print('Processed positions still require coordinator review.')

if __name__ == '__main__':
    main()
