"""Read saved progress concurrently. Do not verify artifacts or run engines."""
import concurrent.futures
from collections import Counter
from datetime import datetime, timezone
from .core import decode, prefix, validate_job, validate_manifest


def snapshot(manifest, store, workers=12):
    jobs = dict(validate_manifest(manifest))
    root = manifest['experiment'] + '/jobs/'
    keys = set(store.list(root))
    # Only published plans are counted. Future confirmation totals are unknown.
    plans = {prefix(j) + '/derived-jobs.json': j for j in jobs.values()
             if j['kind'] == 'reference_screening' and manifest.get('automatic_confirmations')}
    success_keys = {k for k in keys if k.endswith('/success.json') and k.count('/') == root.count('/') + 1}
    selected = plans.keys() & keys
    with concurrent.futures.ThreadPoolExecutor(max_workers=workers) as pool:
        records = dict(zip(sorted(selected), pool.map(lambda k: decode(store.get(k)), sorted(selected))))
        for key in plans.keys() & keys:
            for job in records[key]:
                validate_job(job)
                if (job['experiment'] != manifest['experiment'] or job['kind'] != 'confirmation'
                        or job.get('selection', {}).get('screen_job') != plans[key]['id']):
                    raise ValueError('Invalid saved confirmation plan: ' + key)
                if job['id'] in jobs and jobs[job['id']] != job:
                    raise ValueError('Conflicting saved job identity')
                jobs[job['id']] = job
        # A success marker is published only after the successful attempt receipt.
        # Count names, not receipt/artifact contents. Full verification is separate.
        completed = {job_id for job_id, job in jobs.items()
                     if prefix(job) + '/success.json' in success_keys}
        attempts = {}
        for key in keys:
            parts = key[len(root):].split('/')
            if (len(parts) == 4 and parts[0] in jobs and parts[1] == 'attempts'
                    and parts[3] == 'receipt.json'):
                attempts.setdefault(parts[0], []).append(key)
        # A sole attempt plus success is the ordinary completed case. Only
        # retries or attempts without success need receipt reads to count failures.
        attempt_keys = [key for job_id, entries in attempts.items()
                        if job_id not in completed or len(entries) > 1 for key in entries]
        failed = 0
        for key, receipt in zip(attempt_keys, pool.map(lambda k: decode(store.get(k)), attempt_keys)):
            job_id = key[len(root):].split('/')[0]
            if receipt.get('job') != jobs[job_id] or receipt.get('status') not in ('failed', 'complete'):
                raise ValueError('Invalid attempt record: ' + key)
            failed += receipt['status'] == 'failed'
    totals = Counter(j['kind'] for j in jobs.values())
    counts = Counter(jobs[k]['kind'] for k in completed)
    claims = sum(prefix(job) + '/claim.json' in keys for job in jobs.values() if job['id'] not in completed)
    return {'experiment': manifest['experiment'], 'observed_at_utc': datetime.now(timezone.utc).isoformat(),
            'completed': dict(counts), 'known_totals': dict(totals),
            'games_complete': 2 * counts['game_pair'], 'games_total': 2 * totals['game_pair'],
            'confirmation_plans_published': len(plans.keys() & keys), 'confirmation_plans_expected': len(plans),
            'claims_without_success': claims, 'failed_attempts': failed,
            'unlisted_completion_records': len(success_keys) - len(completed),
            'verification': 'saved markers only; success records and artifacts not checked',
            'limitations': 'Snapshot can change during reads. Claims can be stale. Confirmation totals grow as plans are published. Use verify for artifact integrity.'}


def format_report(value):
    done, totals = value['completed'], value['known_totals']
    lines = [f"Experiment: {value['experiment']}", f"Checked: {value['observed_at_utc']}"]
    if 'batch' in value:
        batch = value['batch']
        lines.append(f"Batch job: {batch['name']}")
        lines.append(f"Batch: {batch['state']} (run time: {batch['run_duration']})")
        lines.append('Batch tasks: ' + ', '.join(f'{k}={v}' for k, v in sorted(batch['task_counts'].items())))
    lines += [f"Games complete: {value['games_complete']}/{value['games_total']}",
              f"Opening pairs complete: {done.get('game_pair', 0)}/{totals.get('game_pair', 0)}",
              f"Screening jobs complete: {done.get('reference_screening', 0)}/{totals.get('reference_screening', 0)}",
              f"Confirmation queries complete: {done.get('confirmation', 0)}/{totals.get('confirmation', 0)} known",
              f"Confirmation plans published: {value['confirmation_plans_published']}/{value['confirmation_plans_expected']}",
              f"Claims without success (running or interrupted): {value['claims_without_success']}",
              f"Failed attempts recorded: {value['failed_attempts']}"]
    if value['unlisted_completion_records']:
        lines.append(f"Other completion records outside known plans: {value['unlisted_completion_records']}")
    return '\n'.join(lines + ['Saved completion markers only. Results were not verified.', value['limitations']])
