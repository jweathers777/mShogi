#!/usr/bin/env python3
"""One Pub/Sub wait, then existing verified retrieval. No engines, submits or queue calls."""
import argparse,subprocess,sys,time
from pathlib import Path
HERE=Path(__file__).resolve().parent;sys.path.insert(0,str(HERE.parents[4]))
from tools.experiments import core,gcp

def read(p):return core.decode(Path(p).read_bytes())
def put_same(store,key,value):
    raw=core.encode(value)
    if not store.put(key,raw) and store.get(key)!=raw:raise ValueError('Immutable completion conflict: '+key)
def bind(result,submitted,config,job_name):
    job=result['job']
    if job.get('uid')!=submitted['uid'] or not gcp._matches_job_resource(job.get('name'),config['project'],config['region'],job_name):raise ValueError('Terminal job identity mismatch')
def run(batch,subscription):
    if not 1<=batch<=10:raise ValueError('Batch must be1..10')
    stem=f'batch-{batch:02d}';manifest_path=HERE/(stem+'-manifest.json');submission_path=HERE/(stem+'-submission.json');config_path=HERE/'cloud-config.json';retriever=HERE/'retrieve_batch.py'
    manifest=read(manifest_path);core.validate_manifest(manifest);submitted=read(submission_path);config=gcp.validate_config(read(config_path));job_name=submitted['name'].rsplit('/',1)[-1]
    if not submitted.get('uid') or not gcp._matches_job_resource(submitted['name'],config['project'],config['region'],job_name):raise ValueError('Invalid saved submission identity')
    cloud=manifest['jobs'][0]['environment']['cloud']
    if cloud['config']!=config or any(j['environment']['cloud']!=cloud for j in manifest['jobs']):raise ValueError('Manifest/cloud configuration mismatch')
    manifest_key=manifest['experiment']+'/manifest.json';uri=config['store_uri'].rstrip('/')+'/'+manifest_key
    expected=gcp.batch_config(uri,cloud['image'],config,len(manifest['pipelines']))
    groups=submitted['taskGroups']
    if len(groups)!=1 or groups[0]['taskCount']!=expected['taskGroups'][0]['taskCount'] or groups[0]['taskSpec']['runnables']!=expected['taskGroups'][0]['taskSpec']['runnables']:raise ValueError('Saved submission does not execute this manifest/image')
    pins={str(p):core.file_hash(p) for p in [manifest_path,submission_path,config_path,retriever]}
    statepath=HERE/(stem+'-wait-retrieve.json');state={'status':'starting','job_name':job_name,'job_uid':submitted['uid'],'experiment':manifest['experiment'],'started_epoch':time.time(),'file_hashes':pins,'no_engines':True}
    with statepath.open('xb') as f:f.write(core.encode(state))
    def save():
        tmp=statepath.with_suffix('.tmp');tmp.write_bytes(core.encode(state));tmp.replace(statepath)
    store=None
    def terminal(result):
        bind(result,submitted,config,job_name)
        path=HERE/(stem+'-terminal.json')
        raw=core.encode(result)
        if path.exists():
            if path.read_bytes()!=raw:raise ValueError('Local terminal receipt conflict')
        else:
            with path.open('xb') as f:f.write(raw)
        put_same(store,manifest['experiment']+'/completions/'+job_name+'.json',result)
        state['terminal_sha256']=core.digest(raw);state['terminal_epoch']=time.time();save()
    try:
        store=gcp.GCSStore(config['store_uri']);remote=store.get(manifest_key)
        if core.decode(remote)!=manifest:raise ValueError('Remote immutable manifest differs')
        state.update(status='waiting_for_terminal',remote_manifest_sha256=core.digest(remote));save()
        result=gcp.wait_for_completion(job_name,config,subscription);terminal(result)
        if result['job']['status']['state']!='SUCCEEDED':raise ValueError('Terminal job not successful')
        for path,h in pins.items():
            if core.file_hash(path)!=h:raise ValueError('Pinned local input changed before retrieval')
        if core.digest(store.get(manifest_key))!=state['remote_manifest_sha256']:raise ValueError('Remote manifest changed during wait')
        state.update(status='retrieving_and_verifying',retrieval_started_epoch=time.time());save()
        command=[sys.executable,str(retriever),str(batch)];state['retrieval_command']=command;save()
        with (HERE/(stem+'-retrieval.stdout.log')).open('xb') as out,(HERE/(stem+'-retrieval.stderr.log')).open('xb') as err:
            code=subprocess.run(command,stdout=out,stderr=err).returncode
        state.update(retrieval_returncode=code,retrieval_finished_epoch=time.time());save()
        if code:raise RuntimeError('Retrieval/verification failed; preserve evidence, no automatic retry')
        receipt_path=HERE/(stem+'-review')/'retrieval-state.json';receipt=read(receipt_path)
        if receipt['status']!='retrieved_verified_needs_independent_review' or receipt['experiment']!=manifest['experiment']:raise ValueError('Retrieval receipt incomplete or wrong experiment')
        state.update(status='retrieved_verified_needs_independent_review',retrieval_receipt_sha256=core.file_hash(receipt_path),verified_jobs=receipt['verified_jobs'],ended_epoch=time.time());save()
        put_same(store,manifest['experiment']+'/completions/'+job_name+'-retrieved.json',state)
        return state
    except BaseException as exc:
        if isinstance(exc,gcp.BatchJobFailed):
            try:terminal(exc.result)
            except BaseException as save_error:state['terminal_preservation_error']=repr(save_error);state['raw_failure_terminal']=exc.result
        state.update(status='failed_preserved_for_review',error=repr(exc),ended_epoch=time.time());save();raise

def main():
    p=argparse.ArgumentParser();p.add_argument('batch',type=int);p.add_argument('--subscription',required=True);a=p.parse_args();print(core.encode(run(a.batch,a.subscription)).decode())
if __name__=='__main__':main()
