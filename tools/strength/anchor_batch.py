"""Prepare and summarize one explicit MSH-185 batch; never submit cloud work."""
import argparse
import gzip
import json
import time
from pathlib import Path
from tools.experiments.core import digest, encode
from tools.strength import anchor_stats
from tools.strength.anchor_worker import audit, config, validate_manifest


def write(path, value):
    with Path(path).open('x') as stream:
        json.dump(value, stream, indent=2, allow_nan=False); stream.write('\n')


def prepare(args):
    build = json.loads(Path(args.build).read_text())
    openings = json.loads(Path(args.openings).read_text())['openings']
    campaign = json.loads(Path(args.campaign).read_text())
    if not 0 <= args.first_pair < args.first_pair + args.pairs <= 200:
        raise ValueError('Pair allocation exceeds frozen 400-game limit')
    if args.phase == 'bracket' and (args.pairs != 4 or len(args.opponents) != 1):
        raise ValueError('Bracket is four pairs against one opponent')
    games = []
    for i in range(args.pairs):
        pair_index = args.first_pair + i
        opponent = args.opponents[i % len(args.opponents)]
        if opponent not in build['engines']:
            raise ValueError('Opponent is not built')
        for swapped in (False, True):
            games.append({'id': f'p{pair_index:03d}-g{int(swapped)}', 'pair_id': f'p{pair_index:03d}',
                          'opponent': opponent, 'opening': openings[pair_index], 'swapped': swapped})
    bins = [games[i::8] for i in range(min(8, len(games)))]
    manifest = {'schema': 1, 'experiment': args.name, 'phase': args.phase, 'build': build,
                'bins': bins, 'campaign_started_epoch': campaign['started_epoch'],
                'deadline_epoch': campaign['started_epoch']+21600,
                'estimated_compute_before_usd': args.compute_before,
                'selection_reason': args.reason,
                'source_files': {name: digest(Path(name).read_bytes()) for name in
                  ('tools/strength/anchor_worker.py','tools/strength/anchor_stats.py','tools/strength/anchor_cloud.py',
                   'tools/strength/anchor_batch.py','docs/m5/chess-anchor-method.md')}}
    validate_manifest(manifest)
    write(args.output, manifest)


def collect(run, clock_reviews=None, shutdown_reviews=None):
    run = Path(run)
    manifest = json.loads((run/'manifest.json').read_text())
    build = manifest['build']
    games, failures = [], []
    for spec in [s for tasks in manifest['bins'] for s in tasks]:
        folder = run/'raw/games'/spec['id']
        path = folder/'receipt.json'
        if not path.exists():
            failures.append({'game_id': spec['id'], 'reason': 'missing receipt'})
            continue
        receipt = json.loads(path.read_text())
        correction = (shutdown_reviews or {}).get(spec['id'])
        if not receipt['complete'] and correction is None:
            failures.append({'game_id': spec['id'], 'reason': 'unscored', 'receipt': receipt})
            continue
        if correction is not None:
            if (receipt['complete'] or correction.get('decision') != 'normal-post-conclusion-shutdown'
                    or correction.get('original_receipt_sha256') != digest(path.read_bytes())
                    or correction.get('report_sha256') != receipt.get('report_sha256')
                    or not correction.get('review')
                    or receipt.get('returncode') != 0
                    or receipt.get('process_group_settled') is not True
                    or receipt.get('residual_process_group') is not False
                    or receipt.get('outer_timeout', False)):
                raise ValueError('Invalid shutdown reclassification evidence')
        raw = gzip.decompress((folder/'report.json.gz').read_bytes())
        if digest(raw) != receipt['report_sha256'] or receipt['replay']['returncode'] != 0:
            raise ValueError('Report hash/replay mismatch')
        observed = audit(json.loads(raw), config(build, spec), build, spec)
        if correction is not None and not observed.get('normal_shutdown_kills'):
            raise ValueError('Shutdown reclassification lacks verified normal termination')
        if not observed['complete'] or (correction is None and observed['score'] != receipt['score']):
            raise ValueError('Receipt disagrees with native report')
        if observed['clock_review_required']:
            decision = (clock_reviews or {}).get(spec['id'])
            if decision not in ('genuine-native-time-loss', 'harness-caused-unscored'):
                raise ValueError('Clock outcome needs recorded timing review: '+spec['id'])
            if decision == 'harness-caused-unscored':
                failures.append({'game_id': spec['id'], 'reason': decision})
                continue
        opponent = build['engines'][spec['opponent']]
        games.append({'game_id': spec['id'], 'pair_id': spec['pair_id'], 'opponent_id': spec['opponent'],
                      'family': opponent['family'], 'anchor': opponent['rating'], 'score': observed['score'],
                      'phase': manifest['phase'], 'report_sha256': digest(raw)})
        if correction is not None:
            games[-1]['shutdown_reclassification'] = correction
    return games, failures


def summarize(args):
    reviews = json.loads(Path(args.clock_reviews).read_text()) if args.clock_reviews else {}
    shutdown_path = getattr(args, 'shutdown_reviews', None)
    shutdown_reviews = json.loads(Path(shutdown_path).read_text()) if shutdown_path else {}
    games, failures = [], []
    for run in args.runs:
        g, f = collect(run, reviews, shutdown_reviews); games.extend(g); failures.extend(f)
    result = {'games': games, 'failures': failures, **anchor_stats.summarize(games)}
    result['opponents'] = {}
    for name in sorted({g['opponent_id'] for g in games}):
        rows = [g for g in games if g['opponent_id'] == name]
        result['opponents'][name] = {'wins':sum(g['score']==1 for g in rows),
            'draws':sum(g['score']==.5 for g in rows),'losses':sum(g['score']==0 for g in rows),
            'score':sum(g['score'] for g in rows)/len(rows), 'fit':anchor_stats.fit(rows)}
    write(args.output, result)
    print(json.dumps({k:result[k] for k in ('joint','family_spread','opponents')}, indent=2))


def main():
    p=argparse.ArgumentParser(description=__doc__); sub=p.add_subparsers(dest='command',required=True)
    a=sub.add_parser('prepare')
    for key in ('build','openings','campaign','name','reason','output'):a.add_argument('--'+key,required=True)
    a.add_argument('--phase',choices=['bracket','main'],required=True)
    a.add_argument('--opponents',nargs='+',required=True)
    a.add_argument('--first-pair',type=int,required=True);a.add_argument('--pairs',type=int,required=True)
    a.add_argument('--compute-before',type=float,required=True);a.set_defaults(func=prepare)
    a=sub.add_parser('summarize');a.add_argument('runs',nargs='+');a.add_argument('--output',required=True)
    a.add_argument('--clock-reviews');a.add_argument('--shutdown-reviews');a.set_defaults(func=summarize)
    args=p.parse_args();args.func(args)

if __name__=='__main__':main()
