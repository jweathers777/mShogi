"""Pinned-image explicit confirmation dispatcher; submitted as reviewed Python bytes."""
import argparse, os
from tools.experiments import core, cli
from tools.experiments.gcp import GCSStore

def main():
    p=argparse.ArgumentParser();p.add_argument('--store',required=True);p.add_argument('--plan-key',required=True);p.add_argument('--plan-sha256',required=True)
    a=p.parse_args();store=GCSStore(a.store);raw=store.get(a.plan_key)
    if core.digest(raw)!=a.plan_sha256:raise ValueError('Plan hash mismatch')
    plan=core.decode(raw)
    if plan['schema']!='msh161-adjudication-v1':raise ValueError('Unknown plan schema')
    if a.store!=plan['store_uri']:raise ValueError('Wrong store')
    if core.digest(store.get(plan['original_manifest']['key']))!=plan['original_manifest']['sha256']:raise ValueError('Original manifest changed')
    index=int(os.environ['BATCH_TASK_INDEX']);bins=plan['bins'];jobs={j['id']:j for j in plan['jobs']}
    if len(jobs)!=len(plan['jobs']) or sorted(x for b in bins for x in b)!=sorted(jobs):raise ValueError('Invalid bin coverage')
    if not 0<=index<len(bins):raise ValueError('Invalid task index')
    refs={r['key']:r for key in bins[index] for r in jobs[key]['adjudication']['consumed_successes']}
    for ref in refs.values():
        if core.digest(store.get(ref['key']))!=ref['sha256']:raise ValueError('Consumed result changed')
    cli.install_signal_handlers()
    completed=[]
    for key in bins[index]:
        j=jobs[key];core.validate_job(j)
        if j['kind']!='confirmation' or j['nodes']!=20000000 or j['experiment']!=plan['experiment']:raise ValueError('Only planned20M confirmations allowed')
        if j['adjudication']['plan_id']!=plan['plan_id']:raise ValueError('Wrong scientific plan')
        r=core.run_job(j,store,cli.REPO)
        completed.append({'job':key,'success_sha256':core.digest(store.get(core.prefix(j)+'/success.json'))})
    key=plan['plan_prefix']+'/task-'+str(index)+'.json';raw=core.encode({'plan_sha256':a.plan_sha256,'index':index,'completed':completed})
    if not store.put(key,raw) and store.get(key)!=raw:raise ValueError('Task receipt conflict')

if __name__=='__main__':main()
