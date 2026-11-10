#!/usr/bin/env python3
"""Read-only artifact/history audit; writes only /tmp/msh159/resource-receipt.json.
Run after every campaign manifest is complete, using the campaign Python env.
"""
import pathlib,json,hashlib,importlib.util,datetime,math,re
P=pathlib.Path; BASE=P('/tmp/msh159')
def read(p): return json.loads(P(p).read_text())
def sha(p): return hashlib.sha256(P(p).read_bytes()).hexdigest()
def stamp(s): return datetime.datetime.fromisoformat(s)
spec=importlib.util.spec_from_file_location('budget',BASE/'runs/runner.py');budget=importlib.util.module_from_spec(spec);spec.loader.exec_module(budget)
artifacts=0;attempts=[];references=[];folders=[];traces=[]
def hashes(d,r):
 global artifacts
 for name,h in r['artifact_sha256'].items():
  assert P(name).name==name,('unsafe artifact path',name)
  assert sha(d/name)==h,(d,name,'hash');artifacts+=1
 assert stamp(r['start_utc'])<=stamp(r['end_utc'])
 for k in ('charged_active_seconds','cpu_seconds','process_wall_seconds'):
  assert math.isfinite(r[k]) and r[k]>=0

def replay(root,pv):
 b=budget.board_for(root)
 for move in pv:
  if isinstance(move,dict):
   if move['kind']!='move':continue
   move=move['uci']
  b.push_uci(move)
 return b

with budget.registry_lock(BASE/'runs/registry.json'):
 state,effective=budget.registry_state(BASE/'runs/registry.json')
 for folder,h in state['directories'].items():
  d=P(folder);m=read(d/'manifest.json');rows=budget.ledger(d)
  assert sha(d/'manifest.json')==h
  assert len(rows)==len(m['cells']) or (rows and rows[-1]['status']!=0 and not d.name.startswith('candidate-')),(folder,'incomplete')
  for key in ('panel','binary','driver'):assert sha(m[key+'_path'])==m[key+'_sha256'],(folder,key)
  for r,c in zip(rows,m['cells']):
   assert r['root_id']==c['root']['id'] and r['index']==c['index']
   hashes(d,r);assert read(d/f"cell-{r['index']:04d}.json")==c
   attempts.append(r)
   if r['status']!=0:continue
   result=r.get('result') or read(r['result_path']);native=result['result'];root=c['root']
   assert native['id']==root['id'] and native['root_unchanged'] and native['pv_native_admitted']
   inp=read(d/f"cell-{r['index']:04d}.native-input.json");inp=inp[0] if isinstance(inp,list) else inp
   assert inp['history']==root['history'] and inp['fen']==root['fen']
   b=budget.board_for(root);assert native['fen']==b.fen(en_passant='fen')
   assert native['time_ms']==round(c['seconds']*1000) and native['table_entries']==c['table_entries']
   assert native['depth_limit']==inp['depth'] and native['node_limit'] is None
   assert dict(re.findall(r'(\w+): (true|false)',native['diagnostics']))=={k:str(v).lower() for k,v in c['diagnostics'].items()}
   end=replay(root,native['pv']);assert end.fen(en_passant='fen')==result['pv_validation']['final_fen']
   if native['decision']['kind']=='move': b.push_uci(native['decision']['uci'])
   if 'trace' in native:
    traces.append({'directory':d.name,'index':r['index'],'events':len(native['trace']['events']),'dropped':native['trace']['dropped']})
  folders.append({'directory':d.name,'attempts':len(rows),'planned_cells':len(m['cells']),'failures':sum(r['status']!=0 for r in rows),'active_seconds':sum(r['charged_active_seconds'] for r in rows),'manifest_sha256':h})
 for folder,h in state['external'].get('reference',{}).get('programs',{}).items():
  d=P(folder);m=read(d/'manifest.json');assert sha(d/'manifest.json')==h
  for key in ('panel','binary','candidates'):assert sha(m[key+'_path'])==m[key+'_sha256']
  assert sha(BASE/'reference-driver.py')==m['driver_sha256']
  rows=budget.ledger(d);assert [r['query_index'] for r in rows]==list(range(len(rows)))
  assert len(rows)==len(m['queries']) or (rows and rows[-1]['status']!=0)
  for r,q in zip(rows,m['queries']):
   hashes(d,r);references.append(r)
   assert r['root_id']==q['root_id'] and r['root_move']==q['root_move']
   if r['status']!=0:continue
   result=read(r['result_path']);root=m['roots'][q['root_id']];b=budget.board_for(root)
   assert result['root_fen']==b.fen(en_passant='fen') and result['full_history_plies']==len(root['history'])
   assert result['query']==q and result['bestmove']==q['root_move']
   for e in result['events']:
    if e.get('pv'):
     end=replay(root,e['pv']);assert e['pv'][0]==q['root_move']
     assert e['pv_validation']['legal'] and end.fen(en_passant='fen')==e['pv_validation']['final_fen']
 intervals=sorted((stamp(r['start_utc']),stamp(r['end_utc'])) for r in attempts+references)
 assert all(a[1]<=b[0] for a,b in zip(intervals,intervals[1:])), 'native/reference overlap'
 native=sum(r['charged_active_seconds'] for r in attempts);reference=sum(r['charged_active_seconds'] for r in references)
 assert abs(reference-state['external']['reference']['actual_charge_seconds'])<1e-6
 reserved=state['external']['correctness-and-static']['reserved_seconds']
 report={'audited_utc':budget.utc(),'native_attempts':len(attempts),'failed_native_attempts':sum(r['status']!=0 for r in attempts),'reference_attempts':len(references),'failed_reference_attempts':sum(r['status']!=0 for r in references),'verified_native_and_reference_artifacts':artifacts,'native_active_seconds':native,'reference_active_seconds':reference,'measured_timed_engine_seconds':native+reference,'correctness_and_static_reserved_upper_bound_seconds':reserved,'conservative_total_including_correctness_and_static':native+reference+reserved,'reference_reservation_seconds':state['external']['reference']['reserved_seconds'],'effective_registry_charge_with_unused_reservations':effective,'global_cap_seconds':state['budget_seconds'],'within_cap':effective<=state['budget_seconds'],'native_worker_cpu_seconds':sum(r['cpu_seconds'] for r in attempts),'native_process_wall_seconds':sum(r['process_wall_seconds'] for r in attempts),'folders':folders,'diagnostic_trace_limits':traces,'checks':['frozen manifest/panel/driver/binary/artifact hashes','sequential complete ledgers or explicitly failed diagnostic stop; no orphan attempts','exact histories and runtime input FENs','legal native decisions/PVs and reference PVs replayed from full histories','native root restoration and admission assertions','no native/reference overlap'],'limitations':'Correctness-and-static120s is a conservative reservation, not measured CPU. Reference180s remains charged even where actual usage is lower. Failed trials and incomplete diagnostic trace iterations are preserved; PVs and static scores do not prove outcomes. Predecessor reused measurements are outside new resource consumption. No matches.'}
 assert report['within_cap']
 (BASE/'resource-receipt.json').write_text(json.dumps(report,indent=2)+'\n')
 print(json.dumps({k:v for k,v in report.items() if k not in ('folders','diagnostic_trace_limits')},indent=2))
