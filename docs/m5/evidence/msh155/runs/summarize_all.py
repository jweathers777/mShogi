import json,pathlib,statistics
base=pathlib.Path(__file__).parent
allrows=[];resources={}
for folder in [base,base/'intermediate',base/'followup30']:
 if not (folder/'runs.jsonl').exists():continue
 m=json.loads((folder/'manifest.json').read_text());runs=[json.loads(l) for l in (folder/'runs.jsonl').read_text().splitlines()]
 resources[folder.name]={'attempts':len(runs),'active':sum(x['charged_active_seconds'] for x in runs),'cpu':sum(x['cpu_seconds'] for x in runs),'wall':sum(x['process_wall_seconds'] for x in runs)}
 for run in runs:
  cell=m['cells'][run['index']];p=cell['root'];r=json.load(open(run['result_path']));n=r.get('result');move=n['move'] if n else r['bestmove'];entry={'root':p['id'],'fresh':p['fresh'],'category':p['primary_category'],'engine':run['engine'],'seconds':run['seconds'],'move':move,'active_seconds':run['charged_active_seconds'],'evidence':run['result_path'],'run_scope':folder.name}
  if n:
   last=next(i for i in reversed(n['iterations']) if i['completed']);entry.update(depth=n['completed_depth'],score=n['score'],nodes=n['nodes'],stats=n['stats_parsed'],pv=n['pv'],incomplete_iteration_seconds=max(0,n['elapsed_ms']/1000-last['elapsed_us']/1e6),incomplete_iteration_share=max(0,1-last['elapsed_us']/(n['elapsed_ms']*1000)),root_restored=n['root_unchanged'],pv_native_admitted=n['pv_native_admitted'],legal_map_verified=n['oracle_legal_set_verified'])
  else:
   ev=r.get('last_nonbound_scored_event_matching_bestmove');entry.update(depth=ev['info'].get('depth') if ev else None,score=ev['info'].get('score') if ev else None,nodes=ev['info'].get('nodes') if ev else None,pv=ev['info'].get('pv') if ev else None,depth_semantics='last bestmove-aligned nonbound info, not proof of complete iteration')
  allrows.append(entry)
allrows.sort(key=lambda x:(x['root'],x['engine'],x['seconds']))
(base/'all-summary.json').write_text(json.dumps({'rows':allrows,'resources':resources},indent=2)+'\n')
for rid in sorted({x['root'] for x in allrows}):
 print(rid)
 for engine in ['mshogi','goldfish','oxidation']:
  print(engine,[(x['seconds'],x['move'],x['depth']) for x in allrows if x['root']==rid and x['engine']==engine])
print(resources)
