"""Cloud recovery check. Uses one real confirmation, not a game or study result."""
import hashlib,json,os
from pathlib import Path
from tools.experiments import core,cli
from tools.experiments.gcp import GCSStore
store=GCSStore('gs://mshogi-experiments-216675633801/msh161')
manifest=core.decode(store.get('msh162-cloud-pilot-one-v1/manifest.json'))
screen=next(j for j in manifest['jobs'] if j['kind']=='reference_screening')
completed=core.verify_result(store,screen)
original=cli.derived_confirmations(screen,completed,store)[0]
core.verify_result(store,original)
probe=core.seal(dict(original,validation_probe='msh162-cloud-failed-job-recovery-v1',probe_script_sha256=os.environ['MSH162_PROBE_SHA256']))
base=core.prefix(probe)
assert not store.exists(base+'/success.json'), 'Probe already completed; inspect its receipt instead'
def fail(job,out,*args):
    (out/'injected-failure.txt').write_text('Intentional validation failure before any engine query.\n')
    raise RuntimeError('intentional cloud recovery validation failure')
try:
    core.run_job(probe,store,cli.REPO,fail)
except RuntimeError as exc:
    assert str(exc)=='intentional cloud recovery validation failure'
else:
    raise AssertionError('Expected injected failure')
assert not store.exists(base+'/success.json')
assert not store.exists(base+'/claim.json')
result=core.run_job(probe,store,cli.REPO)
before=store.get(base+'/success.json')
def forbidden(*args):raise AssertionError('Completed job executed twice')
assert core.run_job(probe,store,cli.REPO,forbidden)==result
assert store.get(base+'/success.json')==before
receipts=[core.decode(store.get(k)) for k in store.list(base+'/attempts/') if k.endswith('/receipt.json') and '/artifacts/' not in k]
assert len(receipts)==2 and sorted(r['status'] for r in receipts)==['complete','failed']
record={'probe_job':probe,'original_job':original['id'],'outcome':'passed','attempts':len(receipts),'statuses':[r['status'] for r in receipts],'success_sha256':hashlib.sha256(before).hexdigest(),'duplicate_execution':False,'scope':'Injected adapter failure on real C3/GCS; retry executes one genuine frozen5M confirmation. This is infrastructure validation, excluded from scientific totals. Not abrupt VM termination.'}
assert store.put('msh162-cloud-pilot-one-v1/validation/recovery-v1.json',core.encode(record))
print(json.dumps(record,indent=2))
