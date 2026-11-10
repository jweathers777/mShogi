import pathlib,json,hashlib,importlib.util,datetime,collections
P=pathlib.Path;BASE=P('/tmp/msh158');sha=lambda p:hashlib.sha256(P(p).read_bytes()).hexdigest();read=lambda p:json.loads(P(p).read_text())
spec=importlib.util.spec_from_file_location('budget',BASE/'runs/runner.py');budget=importlib.util.module_from_spec(spec);spec.loader.exec_module(budget)
with budget.registry_lock(BASE/'runs/registry.json'):
 state,effective=budget.registry_state(BASE/'runs/registry.json');attempts=[];artifacts=0;folders=[]
 for folder,h in state['directories'].items():
  d=P(folder);m=read(d/'manifest.json');rows=budget.ledger(d)
  assert len(rows)==len(m['cells']),(folder,'incomplete')
  assert sha(m['panel_path'])==m['panel_sha256'] and sha(m['binary_path'])==m['binary_sha256'] and sha(m['driver_path'])==m['driver_sha256']
  for r,c in zip(rows,m['cells']):
   assert r['status']==0 and r['root_id']==c['root']['id']
   for name,hash_ in r['artifact_sha256'].items():assert sha(d/name)==hash_;artifacts+=1
   assert read(d/f"cell-{r['index']:04d}.json")==c
   result=r.get('result') or read(r['result_path']);native=result['result'];assert native['root_unchanged'] and native['pv_native_admitted']
   inp=read(d/f"cell-{r['index']:04d}.native-input.json")
   inp=inp[0] if isinstance(inp,list) else inp
   assert inp['history']==c['root']['history'] and inp['fen']==c['root']['fen']
   attempts.append(r)
  folders.append({'directory':d.name,'attempts':len(rows),'active_seconds':sum(r['charged_active_seconds']for r in rows),'manifest_sha256':h})
 reference_rows=[]
 for folder,h in state['external']['reference']['programs'].items():
  d=P(folder);assert sha(d/'manifest.json')==h
  for line in (d/'runs.jsonl').read_text().splitlines():
   r=json.loads(line);assert r['status']==0
   for name,hash_ in r['artifact_sha256'].items():assert sha(d/name)==hash_;artifacts+=1
   reference_rows.append(r)
 intervals=sorted((r['start_utc'],r['end_utc'])for r in attempts+reference_rows)
 assert all(a[1]<=b[0]for a,b in zip(intervals,intervals[1:])), 'native overlap'
 reference=state['external']['reference']['actual_charge_seconds'];native_seconds=sum(r['charged_active_seconds']for r in attempts)
 result={'audited_utc':budget.utc(),'native_attempts':len(attempts),'failed_native_attempts':0,'verified_native_and_reference_artifacts':artifacts,'native_active_seconds':native_seconds,'reference_active_seconds':reference,'measured_timed_engine_seconds':native_seconds+reference,'correctness_test_reserved_upper_bound_seconds':120,'conservative_total_including_tests':native_seconds+reference+120,'effective_registry_charge_with_unused_reservations':effective,'global_cap_seconds':2400,'within_cap':effective<=2400,'native_process_wall_seconds':sum(r['process_wall_seconds']for r in attempts),'native_worker_cpu_seconds':sum(r['cpu_seconds']for r in attempts),'folders':folders,'checks':['manifest/panel/driver/binary/artifact hashes','complete sequential ledgers, no orphan attempts','exact runtime input histories and FENs against manifest','native root restoration and PV admission','no native/reference overlap'],'limitations':'Test reservation120s is conservative, not measured engine-only CPU. Reference120s reservation remains charged despite unused allowance. Reused predecessor measurements excluded from new resource use; full original receipts inMSH157. No matches.'}
 (BASE/'resource-receipt.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({k:v for k,v in result.items()if k!='folders'},indent=2))
