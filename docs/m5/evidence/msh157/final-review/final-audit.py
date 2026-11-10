"""Independent original-artifact audit; no native queries. Run after searches stop."""
from pathlib import Path
import json,hashlib,re,datetime,collections,math,chess
R=Path('/tmp/msh157-runs');O=Path('/tmp/msh157-final-review');O.mkdir(exist_ok=True)
def read(p):return json.loads(Path(p).read_text())
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def ledger(d):return [json.loads(s)for s in (Path(d)/'runs.jsonl').read_text().splitlines()] if (Path(d)/'runs.jsonl').exists() else []
checks=collections.Counter();intervals=[];totals={};failures=[]
def hashes(d,row):
 for name,h in row['artifact_sha256'].items():assert sha(Path(d)/name)==h,(d,name);checks['artifact_hashes']+=1
 for k in ['charged_active_seconds','process_wall_seconds','cpu_seconds']:assert math.isfinite(row[k])and row[k]>=0
 intervals.append((datetime.datetime.fromisoformat(row['start_utc']),datetime.datetime.fromisoformat(row['end_utc']),str(d),row.get('index',row.get('query_index'))));assert intervals[-1][0]<=intervals[-1][1]
def addtotal(kind,row):
 t=totals.setdefault(kind,{'attempts':0,'charged_active_seconds':0,'cpu_seconds':0,'process_wall_seconds':0});t['attempts']+=1
 for k in ['charged_active_seconds','cpu_seconds','process_wall_seconds']:t[k]+=row[k]
def nativecheck(root,n,depth,ms,table,diag):
 b=chess.Board(root.get('fen',chess.STARTING_FEN))
 for u in root.get('history',[]):b.push_uci(u)
 assert n['id']==root['id']and n['fen']==b.fen(en_passant='fen');assert n['root_unchanged']and n['pv_native_admitted']and n['node_limit']is None
 assert n['mode']=='Conventional'and n['depth_limit']==depth and n['time_ms']==ms and n['table_entries']==table
 assert dict(re.findall(r'(\w+): (true|false)',n['diagnostics']))=={k:str(v).lower()for k,v in diag.items()}
 if n['move']:assert chess.Move.from_uci(n['move'])in b.legal_moves
 for a in n['pv']:
  if a['kind']=='move':b.push_uci(a['uci']);checks['pv_plies']+=1
 checks['native_results']+=1
build=read('docs/m5/evidence/msh157/native-build.json')
for name,h in build['source_hashes'].items():assert sha(name)==h,name;checks['source_hashes']+=1
assert sha(build['binary_local_path'])==build['binary_sha256']
registry=read(R/'registry.json');nativeactual=0
for dirname,mhash in registry['directories'].items():
 d=Path(dirname);assert sha(d/'manifest.json')==mhash;m=read(d/'manifest.json');assert sha(m['panel_path'])==m['panel_sha256'];assert sha(m['binary_path'])==m['binary_sha256'];assert sha(m['driver_path'])==m['driver_sha256'];rows=ledger(d);assert len(rows)<=len(m['cells']);assert [r['index']for r in rows]==list(range(len(rows)))
 for row,cell in zip(rows,m['cells']):
  hashes(d,row);addtotal('native',row)
  for k in ['panel_sha256','binary_sha256','driver_sha256']:assert row[k]==m[k]
  assert row['root_id']==cell['root']['id'] and row['seconds']==cell['seconds'] and row['diagnostics']==cell['diagnostics']
  if row['status']!=0:failures.append({'directory':dirname,'index':row['index'],'status':row['status'],'charge':row['charged_active_seconds']});continue
  n=row['result']['result'];nativecheck(cell['root'],n,cell.get('depth',128),round(cell['seconds']*1000),cell['table_entries'],cell['diagnostics']);assert row['charged_active_seconds']==n['elapsed_ms']/1000
refentry=registry['external']['reference']
for dirname,mhash in refentry.get('programs',{}).items():
 d=Path(dirname);assert sha(d/'manifest.json')==mhash;m=read(d/'manifest.json');assert sha(m['binary_path'])==m['binary_sha256'];rows=ledger(d);assert len(rows)<=len(m['queries']);assert [r['query_index']for r in rows]==list(range(len(rows)))
 assert sha(m['panel_path'])==m['panel_sha256'];assert sha(m['candidates_path'])==m['candidates_sha256']
 driver=Path('/tmp/msh157-reference-final-driver.py') if 'final' in d.name else Path('/tmp/msh157-reference-driver.py');assert sha(driver)==m['driver_sha256']
 for row in rows:
  hashes(d,row);addtotal('reference',row)
  for key in ['panel_sha256','binary_sha256','driver_sha256']:assert row[key]==m[key]
  if row['status']!=0:failures.append({'directory':dirname,'index':row['query_index'],'status':row['status'],'charge':row['charged_active_seconds']});continue
  r=read(row['result_path']);root=m['roots'][row['root_id']];b=chess.Board(root.get('fen',chess.STARTING_FEN))
  for u in root['history']:b.push_uci(u)
  assert r['root_fen']==b.fen(en_passant='fen')and r['root_id']==row['root_id'];assert r['bestmove']==row['root_move'];assert r['binary_sha256']==m['binary_sha256'];assert r['active_search_seconds']==row['charged_active_seconds']
  e=r['last_nonbound_event_matching_bestmove'];assert e and not e['lowerbound']and not e['upperbound']and e['pv'][0]==r['bestmove']
  for ev in r['events']:
   p=b.copy()
   for u in ev.get('pv',[]):p.push_uci(u);checks['reference_pv_plies']+=1
   checks['reference_events']+=1
false_diag={k:False for k in ['disable_null','disable_lmr','disable_quiet_omissions','disable_see_pruning','disable_delta_pruning','endgame_lmr']}
for dirname in ['/tmp/msh157-cost-run','/tmp/msh157-cost-retry']:
 d=Path(dirname);m=read(d/'manifest.json')
 for name,h in m['input_sha256'].items():assert sha(d/(name+'.json'))==h
 for version,h in m['binary_sha256'].items():assert sha(m['binary_paths'][version])==h
 for row in ledger(d):
  hashes(d,row);addtotal('cost-profile',row)
  assert row['binary_sha256']==m['binary_sha256'][row['version']] and row['input_sha256']==m['input_sha256'][row['input']]
  if row['status']!=0:failures.append({'directory':dirname,'index':row['index'],'status':row['status'],'charge':row['charged_active_seconds']});continue
  inp=read(d/(row['input']+'.json'));assert len(inp)==len(row['result'])
  for root,n in zip(inp,row['result']):nativecheck(root,n,2 if row['kind']=='fixed' else 128,None if row['kind']=='fixed' else 1000,4096,false_diag)
  assert abs(row['charged_active_seconds']-sum(n['elapsed_ms']/1000 for n in row['result']))<1e-9
intervals.sort()
for a,b in zip(intervals,intervals[1:]):assert a[1]<=b[0],('search overlap',a,b)
actual=sum(t['charged_active_seconds']for t in totals.values());reserved=totals['native']['charged_active_seconds']+sum(max(e['reserved_seconds'],read(e['receipt_path'])['charged_active_seconds'])for e in registry['external'].values());assert actual<=registry['budget_seconds']and reserved<=registry['budget_seconds']
out={'status':'passed','checks':dict(checks),'totals':totals,'actual_engine_active_seconds':actual,'registry_effective_reserved_charge_seconds':reserved,'global_ceiling_seconds':registry['budget_seconds'],'failures':failures,'search_utc_nonoverlap':True,'interval_count':len(intervals),'limitations':['This audits original local artifact bytes; any reference-only STS archive intentionally omits payloads and must not be described as preserving those original bytes.','Compilation/tests/data retrieval/offline proof are separate resources; historical exposure scan CPU is separately recorded and may overlap native timing.']}
(O/'aggregate-audit.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
