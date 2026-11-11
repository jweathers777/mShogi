"""Run with python -m tools.experiments.cli --help."""
import argparse
import concurrent.futures
import copy
import fcntl
import json
import os
import signal
import subprocess
import sys
import tempfile
import time
from pathlib import Path
from .core import (SCHEMA, LocalStore, decode, encode, file_hash, digest, seal, validate_manifest,
                   run_job, prefix, verify_result, status)

REPO = Path(__file__).resolve().parents[2]
STUDY = REPO/'docs/m5/evidence/msh161'

def store_at(uri):
    if uri.startswith('gs://'):
        from .gcp import GCSStore
        return GCSStore(uri)
    return LocalStore(uri)

def read_manifest(path):
    if path.startswith('gs://'):
        bucket, _, key = path[5:].partition('/')
        return decode(store_at('gs://'+bucket).get(key))
    return decode(Path(path).read_bytes())

def publish_manifest(manifest, store):
    validate_manifest(manifest)
    key = manifest['experiment']+'/manifest.json'; data=encode(manifest)
    if not store.put(key, data) and store.get(key) != data:
        raise ValueError('experiment ID already binds a different manifest')
    return key

def make_manifest(args):
    build = decode(Path(args.build).read_bytes())
    original_build=decode((STUDY/'build.json').read_bytes())
    if build['source'] != original_build['source']: raise ValueError('native source differs from frozen study')
    for name in ('candidate','runner'):
        if build['engines'][name]['source'] != original_build['source']: raise ValueError('native executable source differs from frozen study')
    if build['engines']['oxidation060']['source'] != original_build['engines']['oxidation060']['source']: raise ValueError('opponent source differs from frozen study')
    freeze = decode((STUDY/'main-freeze.json').read_bytes())
    source_files = {}
    for name, sha in freeze['files'].items():
        path = (STUDY/name).resolve()
        if file_hash(path) != sha: raise ValueError('frozen input changed: '+name)
        source_files[str(path.relative_to(REPO))]=sha
    for name in ('main-freeze.json',): source_files[str((STUDY/name).relative_to(REPO))]=file_hash(STUDY/name)
    # The infrastructure itself is also part of the executable job identity.
    for path in sorted((REPO/'tools/experiments').glob('*.py')):
        source_files[str(path.relative_to(REPO))] = file_hash(path)
    cloud=None
    if args.cloud_config or args.image:
        if not args.cloud_config or not args.image: raise ValueError('cloud config and image must be supplied together')
        from .gcp import validate_config, batch_config
        cloud=validate_config(decode(Path(args.cloud_config).read_bytes()))
        batch_config(cloud['store_uri']+'/manifest.json',args.image,cloud,1)
    jobs=[]; pipelines=[]
    for schedule_ref in freeze['schedules']:
        path = STUDY/'main'/Path(schedule_ref['path']).name
        if file_hash(path) != schedule_ref['sha256']: raise ValueError('frozen schedule differs')
        source_files[str(path.relative_to(REPO))]=file_hash(path)
    common = {'experiment':args.experiment, 'source_revision':args.source_revision,
              'build':build, 'source_files':source_files,
              'environment':{'architecture':build['architecture'], 'system':build['system'], 'machine_type':args.machine_type},
              'study_freeze_sha256':file_hash(STUDY/'main-freeze.json')}
    if cloud: common['environment']['cloud']={'config':cloud,'image':args.image}
    pairs=[]
    for entry in freeze['schedules']:
        pairs.extend(decode((STUDY/'main'/Path(entry['path']).name).read_bytes())['pairs'])
    if args.pilot_pairs:
        pairs=pairs[:args.pilot_pairs]
    for spec in pairs:
        pair=seal(dict(common, kind='game_pair', spec=spec, inputs={}, expected_outputs=['pair/report.json.gz','pair/receipt.json']))
        screen=seal(dict(common, kind='reference_screening', spec=spec, nodes=500000,
                         inputs={'report':{'job':pair['id'],'path':'pair/report.json.gz'}},
                         expected_outputs=['analysis/manifest.json','screen-v1.json','confirmation-plan.json']))
        jobs += [pair,screen];pipelines.append(pair['id'])
    manifest={'schema':SCHEMA,'experiment':args.experiment,'jobs':jobs,'pipelines':pipelines,
              'automatic_confirmations':True,'cohort':('gcp-pilot' if args.pilot_pairs else 'gcp-primary') if cloud else 'local-execution-validation',
              'pilot_pairs':args.pilot_pairs,'full_study_requires_coordinator_review':True}
    validate_manifest(manifest)
    with Path(args.output).open('xb') as out: out.write(encode(manifest))
    return {'manifest':args.output,'pairs':len(pairs),'jobs':len(jobs),'sha256':file_hash(args.output)}

def derived_confirmations(screen_job, result, store):
    artifact=result['artifacts']['confirmation-plan.json']
    plan=decode(store.get(artifact['key']))
    jobs=[]
    for item in plan:
        for move in dict.fromkeys([item['played'], item.get('alternative')]):
            if not move: continue
            job={k:copy.deepcopy(v) for k,v in screen_job.items() if k not in ('id','schema','expected_outputs')}
            job.update(kind='confirmation', nodes=5000000, game=item['game'], ply=item['ply'], move=move,
                       selection={'screen_job':screen_job['id'],'screen_result_sha256':digest(encode(result)),'decision':item},
                       expected_outputs=['analysis/manifest.json','screen-v1.json'])
            jobs.append(seal(job))
    return jobs

def pipeline(manifest, store, root_id):
    jobs=validate_manifest(manifest)
    root=jobs[root_id]
    run_job(root,store,REPO)
    screens=[j for j in jobs.values() if any(r['job']==root_id for r in j.get('inputs',{}).values())]
    for screen in screens:
        result=run_job(screen,store,REPO)
        if manifest.get('automatic_confirmations') and screen['kind']=='reference_screening':
            derived=derived_confirmations(screen,result,store)
            key=prefix(screen)+'/derived-jobs.json';data=encode(derived)
            if not store.put(key,data) and store.get(key)!=data:raise ValueError('confirmation plan conflict')
            for job in derived:run_job(job,store,REPO)

def all_jobs(manifest,store):
    jobs=list(manifest['jobs'])
    for job in manifest['jobs']:
        if job['kind'] != 'reference_screening' or not manifest.get('automatic_confirmations'):
            continue
        if not store.exists(prefix(job)+'/success.json'):
            continue
        result=verify_result(store,job)
        expected=derived_confirmations(job,result,store)
        key=prefix(job)+'/derived-jobs.json'
        if store.exists(key) and decode(store.get(key)) != expected:
            raise ValueError('derived confirmation jobs differ from verified screen')
        jobs.extend(expected)
    return jobs


def expanded(manifest,store):
    result=copy.deepcopy(manifest); result['jobs']=all_jobs(manifest,store);return result

def install_signal_handlers():
    def interrupted(signum, frame):
        raise RuntimeError(f'worker received signal {signum}')
    for sig in (signal.SIGINT, signal.SIGTERM): signal.signal(sig, interrupted)


def local_pipeline(manifest, store_path, root_id, cpus):
    install_signal_handlers()
    if cpus is not None:
        os.sched_setaffinity(0, cpus)
    pipeline(manifest, LocalStore(store_path), root_id)
    return root_id


def local_cpu_groups(workers):
    if workers == 1:
        return [None]
    if not hasattr(os, 'sched_setaffinity'):
        raise ValueError('parallel competitive jobs require Linux CPU affinity; use one worker on macOS')
    # Keep physical-core siblings in the same worker group to avoid SMT competition.
    groups = {}
    for cpu in sorted(os.sched_getaffinity(0)):
        topology = Path('/sys/devices/system/cpu') / f'cpu{cpu}' / 'topology'
        key = ((topology/'physical_package_id').read_text().strip(),
               (topology/'core_id').read_text().strip())
        groups.setdefault(key, []).append(cpu)
    if workers > len(groups):
        raise ValueError('more workers than independent physical cores')
    result = [set() for _ in range(workers)]
    for index, group in enumerate(groups.values()): result[index % workers].update(group)
    return result


def local_run(args):
    manifest=read_manifest(args.manifest);store=store_at(args.store)
    if args.store.startswith('gs://'):raise ValueError('local executor requires a local result store')
    publish_manifest(manifest,store)
    if not 1 <= args.workers <= 16: raise ValueError('workers must be in [1,16]')
    cpu_groups = local_cpu_groups(args.workers)
    # Host-wide lock prevents two local studies competing for processors.
    with open(Path(tempfile.gettempdir())/'msh-experiment-host.lock','a') as lock:
        fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
        roots=iter(manifest['pipelines'])
        with concurrent.futures.ProcessPoolExecutor(max_workers=args.workers) as pool:
            pending={}
            for cpus in cpu_groups:
                root=next(roots,None)
                if root is not None: pending[pool.submit(local_pipeline,manifest,args.store,root,cpus)]=cpus
            while pending:
                done,_=concurrent.futures.wait(pending,return_when=concurrent.futures.FIRST_COMPLETED)
                for future in done:
                    cpus=pending.pop(future);future.result()
                    root=next(roots,None)
                    if root is not None: pending[pool.submit(local_pipeline,manifest,args.store,root,cpus)]=cpus
    return status(expanded(manifest,store),store)

def notify(args, operation):
    """Notification acceptance is separate from computation outcome."""
    result=1; record={'started_epoch':time.time(),'notification_accepted':False}
    try:
        value=operation(); result=0; return value
    finally:
        record.update(returncode=result,ended_epoch=time.time())
        if getattr(args,'thread',None):
            try:
                proc=subprocess.run(['codex','queue','--thread',args.thread,'--message',
                    'Parallel experiment execution exited '+('successfully' if result==0 else 'with failure')+'. Inspect the durable result store; do not start the full study without coordinator review.'],capture_output=True,text=True,timeout=30)
                record.update(notification_accepted=proc.returncode==0,notification_returncode=proc.returncode,notification_stderr=proc.stderr)
            except (OSError,subprocess.TimeoutExpired) as exc:
                record['notification_error']=repr(exc)
        if getattr(args,'store',None):
            store=store_at(args.store);store.put('notifications/'+str(time.time_ns())+'.json',encode(record))

def main():
    install_signal_handlers()
    parser=argparse.ArgumentParser(description=__doc__);subs=parser.add_subparsers(dest='command',required=True)
    make=subs.add_parser('prepare',help='Create jobs from every frozen opening, or an explicit pilot subset')
    make.add_argument('--build',required=True);make.add_argument('--experiment',required=True);make.add_argument('--source-revision',required=True);make.add_argument('--machine-type',default='c3-standard-4');make.add_argument('--pilot-pairs',type=int,choices=range(1,101));make.add_argument('--output',required=True);make.add_argument('--cloud-config');make.add_argument('--image')
    for name in ('run','resume','status','verify','retrieve','cancel','report','recover','cloud-worker','submit','cloud-status','cloud-cancel','wait-cloud'):
        p=subs.add_parser(name);p.add_argument('--manifest',required=True);p.add_argument('--store',required=True)
        if name in ('run','resume'):p.add_argument('--workers',type=int,default=1);p.add_argument('--thread',required=True)
        if name=='cloud-worker':p.add_argument('--index',type=int,default=int(os.environ.get('BATCH_TASK_INDEX','0')))
        if name=='retrieve':p.add_argument('--output',required=True)
        if name=='recover':p.add_argument('--job',required=True);p.add_argument('--workers-stopped',action='store_true',required=True)
        if name=='report':p.add_argument('--hourly-rate',type=float,required=True)
        if name in ('submit','cloud-status','cloud-cancel','wait-cloud'):p.add_argument('--config',required=True);p.add_argument('--job-name',required=True)
        if name=='wait-cloud':p.add_argument('--subscription',required=True);p.add_argument('--thread',required=True)
        if name=='submit':p.add_argument('--image',required=True);p.add_argument('--spending-approved',action='store_true',required=True)
    args=parser.parse_args()
    if args.command=='prepare':value=make_manifest(args)
    elif args.command in ('run','resume'):value=notify(args,lambda:local_run(args))
    else:
        manifest=read_manifest(args.manifest);validate_manifest(manifest);store=store_at(args.store)
        if args.command=='cloud-worker':
            publish_manifest(manifest,store)
            if not 0<=args.index<len(manifest['pipelines']):raise ValueError('invalid task index')
            pipeline(manifest,store,manifest['pipelines'][args.index]);value={'complete':True,'index':args.index}
        elif args.command=='status':value=status(expanded(manifest,store),store)
        elif args.command=='verify':
            value=[verify_result(store,j)['job']['id'] for j in all_jobs(manifest,store)]
        elif args.command=='cancel':
            store.put(manifest['experiment']+'/cancel.json',encode({'at':time.time(),'policy':'finish active job, stop before next job'}));value={'cancel_requested':True}
        elif args.command=='recover':
            job=next(j for j in all_jobs(manifest,store) if j['id']==args.job)
            key=prefix(job)+'/claim.json';claim=store.get(key)
            store.put(prefix(job)+'/recovery/'+str(time.time_ns())+'.json',encode({'claim':decode(claim),'operator_assertion':'all owning workers stopped','at':time.time()}))
            store.delete(key,claim);value={'claim_reconciled':args.job}
        elif args.command=='retrieve':
            destination=LocalStore(args.output)
            for key in store.list(manifest['experiment']+'/'):
                data=store.get(key)
                if not destination.put(key,data) and destination.get(key)!=data:raise ValueError('destination conflict '+key)
            value=status(expanded(manifest,destination),destination)
        elif args.command=='report':
            receipts=[decode(store.get(k)) for k in store.list(manifest['experiment']+'/jobs/') if k.endswith('/receipt.json') and '/artifacts/' not in k]
            wall=sum(r['wall_seconds'] for r in receipts);value={'attempts':len(receipts),'failed_attempts':sum(r['status']!='complete' for r in receipts),'active_worker_seconds':wall,'child_cpu_seconds':sum(r['child_cpu_seconds'] for r in receipts),'estimated_active_compute_cost':wall/3600*args.hourly_rate,'hourly_rate':args.hourly_rate,'limitations':'Estimate excludes VM startup/idle, storage, logs, network and unfinished attempts; reconcile Cloud Billing before calling it actual cost.'}
        else:
            from . import gcp
            config=decode(Path(args.config).read_bytes())
            if args.command=='submit':
                config=gcp.validate_config(config)
                expected={'config':config,'image':args.image}
                if any(j['environment'].get('cloud') != expected for j in manifest['jobs']): raise ValueError('cloud allocation/image differs from frozen jobs')
                if args.store.rstrip('/') != config['store_uri'].rstrip('/'): raise ValueError('submission and worker stores differ')
                key=publish_manifest(manifest,store)
                if not args.store.startswith('gs://'):raise ValueError('cloud submission requires GCS store')
                body=gcp.batch_config(args.store.rstrip('/')+'/'+key,args.image,config,len(manifest['pipelines']))
                value=gcp.submit_job(args.job_name,args.store.rstrip('/')+'/'+key,args.image,config,len(manifest['pipelines']))
                store.put(manifest['experiment']+'/submissions/'+args.job_name+'.json',encode({'batch_response':value,'configuration':body,'at':time.time()}))
            elif args.command=='wait-cloud':
                def wait():
                    try:
                        result=gcp.wait_for_completion(args.job_name,config,args.subscription)
                    except gcp.BatchJobFailed as exc:
                        store.put(manifest['experiment']+'/completions/'+args.job_name+'.json',encode(exc.result));raise
                    store.put(manifest['experiment']+'/completions/'+args.job_name+'.json',encode(result))
                    return result
                value=notify(args,wait)
            elif args.command=='cloud-status':value=gcp.status_job(args.job_name,config['project'],config['region'])
            else:value=gcp.cancel_job(args.job_name,config['project'],config['region'])
    print(json.dumps(value,indent=2))
if __name__=='__main__':main()
