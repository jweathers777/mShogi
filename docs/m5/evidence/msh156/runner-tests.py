from pathlib import Path
import tempfile,subprocess,json,fcntl
PYTHON='/tmp/msh139/venv/bin/python';DRIVER='/tmp/msh156-runner.py'
root=Path(tempfile.mkdtemp(prefix='msh156-runner-tests-'));panel=root/'panel.json';panel.write_text(json.dumps([{'id':'synthetic-standard-start-smoke','fen':'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1'}]))
raw=json.loads(Path('/tmp/msh155/smoke/cell-000-mshogi.native.jsonl').read_text())
def mock(name,result):
 p=root/(name+'-bin');p.write_text('#!/usr/bin/env python3\nprint('+repr(json.dumps(result))+')\n');p.chmod(0o755);return p
receipt=mock('receipt',raw)
def call(*args,ok=True):
 r=subprocess.run([PYTHON,DRIVER,*args],capture_output=True,text=True)
 assert (r.returncode==0)==ok,(args,r.stdout,r.stderr)
 return r
def prepare(name,binary=receipt,registry=None,budget=3300):
 out=root/name;registry=registry or root/(name+'-registry.json')
 call('prepare','--out',str(out),'--panel',str(panel),'--binary',str(binary),'--source-revision','mock-no-engine','--seconds','.1','--budget','20','--registry',str(registry),'--global-budget',str(budget));return out
out=prepare('success');call('next','--out',str(out));row=json.loads((out/'runs.jsonl').read_text());assert row['status']==0 and row['result']['result']['root_unchanged'] and row['start_utc']<=row['end_utc'];assert row['charged_active_seconds']==.10020183300000001
before=(out/'runs.jsonl').read_bytes();call('next','--out',str(out));assert (out/'runs.jsonl').read_bytes()==before
out=prepare('changed');panel.write_text(panel.read_text()+'\n');call('next','--out',str(out),ok=False);assert not (out/'runs.jsonl').exists()
registry=root/'orphan-registry.json';out=prepare('orphan',registry=registry);other=prepare('orphan-other',registry=registry);(out/'cell-0000.started.json').write_text('{"index":0}');call('next','--out',str(other),ok=False);assert not (other/'runs.jsonl').exists()
fail=root/'failure-bin';fail.write_text('#!/bin/sh\nexit 7\n');fail.chmod(0o755);out=prepare('failure',fail);call('next','--out',str(out),ok=False);row=json.loads((out/'runs.jsonl').read_text());assert row['status']!=0 and row['charged_active_seconds']==row['process_wall_seconds'] and row['charged_active_seconds']>0
before=(out/'runs.jsonl').read_bytes();call('next','--out',str(out));assert (out/'runs.jsonl').read_bytes()==before
bad=root/'malformed-bin';bad.write_text('#!/bin/sh\nprintf "not JSON"\n');bad.chmod(0o755);out=prepare('malformed',bad);call('next','--out',str(out),ok=False);assert json.loads((out/'runs.jsonl').read_text())['status']!=0
for name,change in [('wrong-id',{'id':'wrong'}),('wrong-time',{'time_ms':5000}),('wrong-diagnostic',{'diagnostics':raw['diagnostics'].replace('disable_lmr: false','disable_lmr: true')})]:
 out=prepare(name,mock(name,{**raw,**change}));call('next','--out',str(out),ok=False);row=json.loads((out/'runs.jsonl').read_text());assert row['status']!=0 and row['charged_active_seconds']==row['process_wall_seconds']
registry=root/'global-registry.json';a=prepare('global-a',registry=registry,budget=15.2);b=prepare('global-b',registry=registry,budget=15.2);call('next','--out',str(a));call('next','--out',str(b),ok=False);assert not (b/'runs.jsonl').exists()
registry=root/'lock-registry.json';out=prepare('lock',registry=registry)
with open(str(registry)+'.lock','a') as lock:
 fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB);call('next','--out',str(out),ok=False)
assert not (out/'runs.jsonl').exists()
registry=root/'manifest-registry.json';out=prepare('manifest',registry=registry);other=prepare('manifest-other',registry=registry);p=out/'manifest.json';p.write_text(p.read_text()+'\n');call('next','--out',str(other),ok=False)
registry=root/'external-registry.json';out=prepare('external',registry=registry,budget=20)
call('reserve','--registry',str(registry),'--id','oracle','--seconds','5')
call('reserve','--registry',str(registry),'--id','oracle','--seconds','5',ok=False)
call('reserve','--registry',str(registry),'--id','negative','--seconds','-1',ok=False)
call('next','--out',str(out),ok=False);assert not (out/'runs.jsonl').exists()
charge=root/'external-receipt.json';charge.write_text(json.dumps({'charged_active_seconds':1,'process_wall_seconds':1.1,'cpu_seconds':.9,'status':0,'start_utc':'2026-11-10T00:00:00+00:00','end_utc':'2026-11-10T00:00:02+00:00'}))
call('charge','--registry',str(registry),'--id','oracle','--receipt',str(charge));call('charge','--registry',str(registry),'--id','oracle','--receipt',str(charge),ok=False)
call('next','--out',str(out),ok=False);assert not (out/'runs.jsonl').exists() # reservation not refunded
call('reserve','--registry',str(registry),'--id','duplicate','--seconds','1');call('charge','--registry',str(registry),'--id','duplicate','--receipt',str(charge),ok=False)
badcharge=root/'negative-charge.json';v=json.loads(charge.read_text());v['charged_active_seconds']=-1;badcharge.write_text(json.dumps(v));call('charge','--registry',str(registry),'--id','duplicate','--receipt',str(badcharge),ok=False)
print(json.dumps({'passed':['retained receipt replay and native/PV validation','UTC receipts and active charge','completed cell never rerun','changed panel refused','cross-folder orphan refused','failed worker preserved and wall charged','failed attempt never rerun','malformed native output preserved and wall charged','wrong ID rejected','wrong time rejected','wrong diagnostics rejected','global budget aggregates both directories','global lock excludes concurrent runs','any registered manifest mutation blocks all runs','external reservation blocks native spend','duplicate/negative reservation rejected','external actual receipt retained without refund','duplicate/negative receipt rejected'],'engine_searches':0,'artifact_directory':str(root)},indent=2))
