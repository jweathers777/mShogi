"""Offline final verifier; run only after coordinator confirms native campaign stopped."""
from pathlib import Path
import json,hashlib,datetime,importlib.util,collections,chess
R=Path('/tmp/msh157-runs');O=Path('/tmp/msh157-final-review');O.mkdir(exist_ok=True)
def read(p):return json.loads(Path(p).read_text())
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def ledger(p):return [json.loads(s)for s in (p/'runs.jsonl').read_text().splitlines()]
def write(n,x):(O/n).write_text(json.dumps(x,indent=2)+'\n')
assert sha(R/'grade.py')=='02c3cc20fb8292d353b931711e635f86a340122b3307aba037a47c2e876b2e9e'
spec=importlib.util.spec_from_file_location('g',R/'grade.py');g=importlib.util.module_from_spec(spec);spec.loader.exec_module(g)
meta=read('/tmp/msh156-data/metadata.json');parts={p:read('/tmp/msh156-data/'+p+'.json')for p in ['development','heldout']}
for p in parts:assert sha('/tmp/msh156-data/'+p+'.json')==meta[p]['sha256']
checks=collections.Counter();intervals=[];allrows={};manifests={}
for folder,n,seconds,part in [('baseline-development',160,5,'development'),('candidate-development',160,5,'development'),('heldout-predecessor',80,5,'heldout'),('heldout-candidate',80,5,'heldout'),('fast-predecessor',40,1,'fast'),('fast-candidate',40,1,'fast')]:
 d=R/folder;m=read(d/'manifest.json');rows=ledger(d);assert len(rows)==n and [r['index']for r in rows]==list(range(n));assert sha(m['panel_path'])==m['panel_sha256'];assert sha(m['binary_path'])==m['binary_sha256'];assert sha(R/'runner.py')==m['driver_sha256']
 expected=read(m['panel_path']);assert expected==(parts[part]if part in parts else read(R/'fast-panel.json'));assert len(m['cells'])==n
 for cell,row in zip(m['cells'],rows):
  assert cell['root']==expected[row['index']];assert row['root_id']==cell['root']['id'];assert cell['seconds']==row['seconds']==seconds;assert cell['table_entries']==4096;assert not any(cell['diagnostics'].values()) and row['diagnostics']==cell['diagnostics'];assert row['status']==0
  for k in ['panel_sha256','driver_sha256','binary_sha256']:assert row[k]==m[k]
  for file,h in row['artifact_sha256'].items():assert sha(d/file)==h;checks['artifact_hashes']+=1
  root=cell['root'];b=chess.Board(root['fen'])
  for u in root['history']:b.push_uci(u)
  native=row['result']['result'];assert b.fen(en_passant='fen')==root['expected_root_fen']==native['fen'];assert native['root_unchanged']and native['pv_native_admitted'];assert native['node_limit']is None and native['depth_limit']==128 and native['table_entries']==4096 and native['time_ms']==seconds*1000
  if native['move']:assert chess.Move.from_uci(native['move'])in b.legal_moves
  for action in native['pv']:
   if action['kind']=='move':b.push_uci(action['uci']);checks['pv_plies']+=1
  assert abs(row['charged_active_seconds']-native['elapsed_ms']/1000)<1e-9
  start=datetime.datetime.fromisoformat(row['start_utc']);end=datetime.datetime.fromisoformat(row['end_utc']);assert start<=end;intervals.append((start,end,folder,row['index']));checks['native_results']+=1
 manifests[folder]=m;allrows[folder]={r['root_id']:r for r in rows}
intervals.sort()
for a,b in zip(intervals,intervals[1:]):assert a[1]<=b[0],('overlap',a,b)
for label in ['heldout','fast']:
 for i in range(len(manifests[label+'-predecessor']['cells'])):
  matches=sorted([r for r in intervals if r[2].startswith(label+'-')and r[3]==i]);assert [x[2]for x in matches]==([label+'-predecessor',label+'-candidate']if i%2==0 else[label+'-candidate',label+'-predecessor'])
comparisons=[]
for part,before,after in [('development','baseline-development','candidate-development'),('heldout','heldout-predecessor','heldout-candidate'),('fast','fast-predecessor','fast-candidate')]:
 panel=parts[part]if part in parts else read(R/'fast-panel.json');rows=[]
 for root in panel:
  pre=allrows[before][root['id']]['result']['result'];cur=allrows[after][root['id']]['result']['result'];row={**g.compare(root,pre['move'],cur['move']),'partition':part,'source_kind':root['source_kind'],'stratum':root['stratum']};rows.append(row)
 write(part+'-comparison.json',{'rows':rows,'summary':[{'tier':k[0],'judgment':k[1],'count':v}for k,v in sorted(collections.Counter((x['tier'],x['outcome_change'])for x in rows).items())]});comparisons+=rows
write('verification.json',{'status':'passed','checks':dict(checks),'primary_roots':240,'secondary_fast_roots':40,'utc_intervals_nonoverlapping':True,'paired_order_verified':['heldout','fast'],'development_order':'predecessor batch then candidate batch','manifest_hashes':{p:sha(R/p/'manifest.json')for p in manifests},'ledger_hashes':{p:sha(R/p/'runs.jsonl')for p in manifests},'source_revisions':{p:m['source_revision']for p,m in manifests.items()},'binary_hashes':{p:m['binary_sha256']for p,m in manifests.items()},'engine_queries':0})
# Prior context reuse: conditions and both same-parent chosen moves verified without new query.
initial=Path('/tmp/msh157-reference-initial');im=read(initial/'manifest.json');ir=ledger(initial);prior=collections.defaultdict(dict)
for row in ir:
 if row['status']!=0:continue
 for name,h in row['artifact_sha256'].items():assert sha(initial/name)==h
 r=read(row['result_path']);options=r['configured_options'];event=r.get('last_nonbound_event_matching_bestmove')
 if r['query']['seconds']!=3 or options.get('Threads')!=1 or options.get('Hash')!=16 or options.get('SyzygyPath')!='' or not event:continue
 if event['lowerbound']or event['upperbound']or event['pv'][0]!=r['bestmove']:continue
 prior[row['root_id']][row['root_move']]={'result_path':row['result_path'],'result_sha256':sha(row['result_path'])}
roots={x['id']:x for p in parts.values()for x in p};eligible=[];reused=[];tierorder={'published-puzzle-preference':0,'published-strategic-preference':1,'finite-reference-opinion':2}
for row in comparisons:
 if row['partition']=='fast' or row['tier']not in tierorder or row['decision_change']!='changed':continue
 u,v=row['before']['move'],row['after']['move']
 if not u or not v:continue
 root=roots[row['id']];domain=root['source_kind']+':'+('strategic'if root['source_kind']=='sts'else root['stratum'].split('/')[0]);item={'id':row['id'],'partition':row['partition'],'tier':row['tier'],'domain':domain,'moves':sorted([u,v]),'raw_outcome_change':row['outcome_change']}
 old=im['roots'].get(row['id'])
 if old and all(old[k]==root[k]for k in ['fen','history','expected_root_fen'])and u in prior[row['id']]and v in prior[row['id']]:item['evidence']=[prior[row['id']][move]for move in sorted([u,v])];reused.append(item)
 else:eligible.append(item)
by={p:[x for x in eligible if x['partition']==p]for p in ['development','heldout']};quota={p:min(4,len(x))for p,x in by.items()}
while sum(quota.values())<min(8,len(eligible)):
 for p in ['heldout','development']:
  if quota[p]<len(by[p]):quota[p]+=1;break
selected=[];rankings={}
for p,pool in by.items():
 groups=collections.defaultdict(list)
 for x in pool:groups[x['domain']].append(x)
 for v in groups.values():v.sort(key=lambda x:x['id'])
 domains=sorted(groups,key=lambda d:(tierorder[groups[d][0]['tier']],d));ranked=[];roundnum=0
 while len(ranked)<len(pool):
  for d in domains:
   if roundnum<len(groups[d]):ranked.append(groups[d][roundnum])
  roundnum+=1
 rankings[p]=ranked;selected+=ranked[:quota[p]]
write('selection.json',{'state':'FROZEN_FINAL_REFERENCE_SELECTION','plan_sha256':sha('/tmp/msh157-final-selection-plan.md'),'new_pair_limit':8,'quota':quota,'selected':selected,'reused':reused,'eligible_rankings':rankings,'selection_uses_outcome_direction':False,'nominal_new_seconds':len(selected)*6})
write('selected-roots-private.json',[roots[x['id']]for x in selected])
print(json.dumps({'verification':'passed','checks':dict(checks),'new_pairs':len(selected),'reused_pairs':len(reused),'quota':quota,'output':str(O)},indent=2))
