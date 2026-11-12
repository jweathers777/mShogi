#!/usr/bin/env python3
"""One owner-authorized short cloud match; completion events, no polling/retry."""
import hashlib, json, sys, time
from pathlib import Path
HERE=Path(__file__).resolve().parent
REPO=HERE.parents[5]
sys.path.insert(0,str(REPO))
from tools.experiments import gcp


def save(path,value): path.write_text(json.dumps(value,indent=2)+'\n')

def main():
    state_path=HERE/'validation-state.json'
    with state_path.open('x') as f: json.dump({'status':'starting','started_epoch':time.time()},f)
    state=json.loads(state_path.read_text());result_dir=HERE/'validation-results';result_dir.mkdir(exist_ok=False)
    config=json.loads((HERE.parent/'cloud-config.json').read_text())
    image_state=json.loads((HERE/'image-build-state.json').read_text());assert image_state['status']=='success'
    job='msh161-policy-validation-v1';prefix='policy-transition-validation-v1'
    store=gcp.GCSStore(config['store_uri']);state.update(job_id=job,store_prefix=prefix,image=image_state['image']);save(state_path,state)
    try:
        if store.list(prefix+'/'): raise RuntimeError('Validation prefix already exists; reconcile without rerun')
        conf=dict(config,parallelism=1,max_retries=0,max_run_seconds=600)
        spec=gcp.batch_config(config['store_uri']+'/'+prefix+'/unused-manifest.json',image_state['image'],conf,1)
        container=spec['taskGroups'][0]['taskSpec']['runnables'][0]['container']
        container['commands']=['docs/m5/evidence/msh161/cloud-primary/policy-transition/validate_policy_match.py',
            '--build','docs/m5/evidence/msh161/cloud-primary/policy-transition/linux-build.json',
            '--output','/tmp/msh161-policy-validation','--store',config['store_uri'],'--prefix',prefix]
        save(HERE/'validation-job-spec.json',spec)
        state['submission_intent_epoch']=time.time();save(state_path,state)
        submission=gcp._run(['submit',job,*gcp._scope(conf['project'],conf['region'],job),'--config',str(HERE/'validation-job-spec.json')])
        save(HERE/'validation-submission.json',submission)
        terminal=gcp.wait_for_completion(job,conf,'projects/mshogi/subscriptions/mshogi-pilot-completion')
        save(HERE/'validation-cloud-completion.json',terminal)
        assert terminal['job']['uid']==submission['uid'] and terminal['job']['name']==submission['name'], 'Submission/terminal job identity changed'
        state['cloud_succeeded']=True
    except BaseException as exc:
        state.update(status='failure',error=repr(exc))
        if isinstance(exc,gcp.BatchJobFailed): save(HERE/'validation-cloud-failure.json',exc.result)
        raise
    finally:
        try:
            keys=store.list(prefix+'/');state['retrieved_objects']=[]
            for key in keys:
                relative=Path(key.removeprefix(prefix+'/'))
                if relative.is_absolute() or '..' in relative.parts: raise ValueError('Unsafe validation artifact path')
                path=result_dir/relative;path.parent.mkdir(parents=True,exist_ok=True);raw=store.get(key);path.write_bytes(raw)
                state['retrieved_objects'].append({'key':key,'sha256':hashlib.sha256(raw).hexdigest()})
            if state.get('cloud_succeeded'):
                manifest=json.loads((result_dir/'artifact-manifest.json').read_text())
                # Validate helper's bounded immutable upload manifest before accepting its receipt.
                names=[r['name'] for r in manifest['artifacts']]
                assert len(names)==len(set(names)) and {'receipt.json','build-input.json','report.json'}<=set(names)
                for row in manifest['artifacts']:
                    relative=Path(row['name'])
                    assert len(relative.parts)==1 and relative.name not in ('.','..')
                    path=result_dir/relative
                    assert hashlib.sha256(path.read_bytes()).hexdigest()==row['sha256']
                receipt=json.loads((result_dir/'receipt.json').read_text())
                assert receipt['smoke_pass'] and receipt['status']=='passed'
                expected_build=hashlib.sha256((HERE/'linux-build.json').read_bytes()).hexdigest()
                build_state=json.loads((HERE/'linux-build-state.json').read_text())
                assert receipt['build_sha256']==expected_build==build_state['build_sha256']
                assert json.loads((result_dir/'build-input.json').read_text())==json.loads((HERE/'linux-build.json').read_text())
                package=json.loads((HERE/'package-receipt.json').read_text())
                helper='docs/m5/evidence/msh161/cloud-primary/policy-transition/validate_policy_match.py'
                assert receipt['helper_sha256']==package['sources'][helper]
                assert receipt['report_sha256']==hashlib.sha256((result_dir/'report.json').read_bytes()).hexdigest()
                state.update(status='success',receipt_sha256=hashlib.sha256((result_dir/'receipt.json').read_bytes()).hexdigest())
        except BaseException as retrieval_error:
            state.update(status='failure',retrieval_error=repr(retrieval_error))
        state['finished_epoch']=time.time();save(state_path,state)
    if state['status']!='success': raise RuntimeError('Cloud validation needs reconciliation; preserve evidence before retry')
if __name__=='__main__':main()
