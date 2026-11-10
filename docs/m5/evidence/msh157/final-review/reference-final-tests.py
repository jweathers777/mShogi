import importlib.util,pathlib,tempfile,json,contextlib,io
spec=importlib.util.spec_from_file_location('ref','/tmp/msh157-reference-final-driver.py');r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
root=pathlib.Path(tempfile.mkdtemp(prefix='msh157-reference-final-tests-'))
panel=pathlib.Path('/tmp/msh156-data/development.json')
c=r.read('/tmp/msh157-reference-candidates.json');c['state']='FROZEN_FINAL_REFERENCE_PAIRS';c['panel_sha256']=r.digest(panel)
for q in c['queries']:q['role']='predecessor' if q['role']=='candidate' else 'candidate'
candidates=root/'candidates.json';r.write(candidates,c)
def fixture(name,body):
 base=root/name;base.mkdir();r.REGISTRY=base/'registry.json';r.write(r.REGISTRY,{'schema':1,'budget_seconds':3300,'directories':{},'external':{'reference':{'reserved_seconds':160}}})
 fake=base/'mock-supervised-worker.py';fake.write_text(body);r.__file__=str(fake);out=base/'run'
 with contextlib.redirect_stdout(io.StringIO()):r.prepare(str(candidates),str(out),str(panel))
 return out
body="import json,pathlib,sys\np=pathlib.Path(sys.argv[2]);q=json.loads(p.read_text());out={'root_id':q['root']['id'],'query':q['query'],'binary_sha256':'"+r.digest(r.BINARY)+"','bestmove':q['query']['root_move'],'active_search_seconds':0.001};p.with_suffix('.result.json').write_text(json.dumps(out))\n"
out=fixture('success',body)
with contextlib.redirect_stdout(io.StringIO()):r.nextq(str(out),'coordinator-slot-released')
row=r.ledger(out)[0];assert row['status']==0 and row['start_utc']<=row['end_utc'];state,used=r.budget.registry_state(r.REGISTRY);assert used==160 and state['external']['reference']['actual_charge_seconds']==.001
for name,body in [('failure','import sys\nsys.exit(7)\n'),('malformed',"import pathlib,sys\npathlib.Path(sys.argv[2]).with_suffix('.result.json').write_text('broken')\n")]:
 out=fixture(name,body)
 try:
  with contextlib.redirect_stdout(io.StringIO()):r.nextq(str(out),'coordinator-slot-released')
 except SystemExit:pass
 else:raise AssertionError('expected failure')
 row=r.ledger(out)[0];assert row['status']!=0 and row['charged_active_seconds']==row['process_wall_seconds'] and row['charged_active_seconds']>0
 r.budget.registry_state(r.REGISTRY)
out=fixture('orphan','raise Exception()\n');state=r.read(r.REGISTRY);pending=out/'pending.json';r.write(pending,{'pending_reference_attempt':0});state['external']['reference'].update({'receipt_path':str(pending),'receipt_sha256':r.digest(pending)});r.budget.save_registry(r.REGISTRY,state)
try:r.budget.registry_state(r.REGISTRY)
except KeyError:pass
else:raise AssertionError('pending marker should fail closed')
original=r.read(candidates)
# Minimal two-query program accepted; malformed pair/count/hash rejected at prepare.
c={**original,'queries':original['queries'][:2],'nominal_active_seconds':6};r.write(candidates,c);out=fixture('two-query','raise Exception()\n');assert len(r.read(out/'manifest.json')['queries'])==2
for name,change in [('odd-count',{'queries':original['queries'][:3],'nominal_active_seconds':9}),('excess-count',{'queries':original['queries']+original['queries'][:2],'nominal_active_seconds':54}),('wrong-panel-hash',{'panel_sha256':'bad'}),('wrong-roles',{'queries':[{**q,'role':'candidate'} for q in original['queries']]})]:
 r.write(candidates,{**original,**change})
 try:fixture(name,'raise Exception()\n')
 except AssertionError:pass
 else:raise AssertionError('invalid final protocol accepted: '+name)
print(json.dumps({'passed':['success UTC and immutable aggregate receipt','reference reservation not double counted','worker failure charged wall','malformed result charged wall','pending marker blocks global registry','two and sixteen queries accepted','odd/excess query counts rejected','wrong panel hash and version roles rejected'],'engine_queries':0,'artifacts':str(root)},indent=2))
