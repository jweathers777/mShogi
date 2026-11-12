#!/usr/bin/env python3
"""Package and push corrected image; stage manifests without activating a batch."""
import argparse, hashlib, importlib.util, json, subprocess, sys, time
from pathlib import Path
HERE=Path(__file__).resolve().parent
REPO=HERE.parents[5]
sys.path.insert(0,str(REPO))
from tools.experiments.package import package
from tools.experiments.core import decode, file_hash


def main():
    p=argparse.ArgumentParser();p.add_argument('--source-revision',required=True);p.add_argument('--workspace',required=True);a=p.parse_args()
    work=Path(a.workspace);work.mkdir(exist_ok=False)
    state_path=HERE/'image-build-state.json'
    if state_path.exists(): raise FileExistsError('Reconcile and preserve previous image build before retry')
    state={'status':'running','started_epoch':time.time(),'worker_source':a.source_revision,'commands':[]}
    def save(): state_path.write_text(json.dumps(state,indent=2)+'\n')
    def run(cmd, timeout=1800):
        state['commands'].append(cmd);save()
        with (work/'image.log').open('ab') as log: subprocess.run(cmd,cwd=REPO,stdout=log,stderr=subprocess.STDOUT,timeout=timeout,check=True)
    save()
    try:
        native=decode((HERE/'linux-build-state.json').read_bytes());assert native['status']=='success'
        build=decode((HERE/'linux-build.json').read_bytes());assert file_hash(HERE/'linux-build.json')==native['build_sha256']
        loader=importlib.util.spec_from_file_location('policy_cohort',HERE/'prepare_corrected_cohort.py');cohort=importlib.util.module_from_spec(loader);loader.loader.exec_module(cohort)
        old=decode((HERE.parent/'batch-03-manifest.json').read_bytes());old_image=old['jobs'][0]['environment']['cloud']['image']
        staging=work/'package-manifests'
        cohort.prepare(REPO,HERE/'linux-build.json',HERE/'amendment.json',old_image,a.source_revision,staging,True)
        manifest=decode((staging/'batch-03-manifest.json').read_bytes())
        for name,expected in manifest['jobs'][0]['source_files'].items():
            raw=subprocess.check_output(['git','show',a.source_revision+':'+name],cwd=REPO)
            if hashlib.sha256(raw).hexdigest()!=expected: raise ValueError('Worker commit does not bind source: '+name)
        for name in ('tools/experiments/Dockerfile','tools/experiments/requirements.txt'):
            raw=subprocess.check_output(['git','show',a.source_revision+':'+name],cwd=REPO)
            if hashlib.sha256(raw).hexdigest()!=file_hash(REPO/name): raise ValueError('Uncommitted packaging source: '+name)
        context=work/'context';receipt=package(manifest,REPO,Path(native['binaries']),context)
        (HERE/'package-receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
        registry=old_image.split('@')[0];tag=registry+':msh161-policy-'+a.source_revision[:12]
        run(['/usr/local/bin/docker','build','--platform','linux/amd64','--build-arg','PYTHON_BASE='+build['runtime_base'],'-f',str(context/'tools/experiments/Dockerfile'),'-t',tag,str(context)])
        run(['/usr/local/bin/docker','push',tag],900)
        description=json.loads(subprocess.check_output(['gcloud','artifacts','docker','images','describe',tag,'--format=json'],text=True,timeout=120))
        (HERE/'image-registry-description.json').write_text(json.dumps(description,indent=2)+'\n')
        digest=description['image_summary']['digest'];assert digest.startswith('sha256:') and len(digest)==71
        image=registry+'@'+digest
        final=HERE/'staged-manifests'
        receipt=cohort.prepare(REPO,HERE/'linux-build.json',HERE/'amendment.json',image,a.source_revision,final,False)
        state.update(status='success',image=image,tag=tag,context=str(context),staging=str(final),cohort_sha256=file_hash(final/'cohort-transition.json'))
    except BaseException as exc:
        state.update(status='failure',error=repr(exc),reconciliation_required=True,
                     note='Do not retry automatically. Inspect Docker build/registry state; a daemon build can outlive its CLI timeout.')
        raise
    finally:
        state.update(finished_epoch=time.time(),log=str(work/'image.log'));save()
if __name__=='__main__':main()
