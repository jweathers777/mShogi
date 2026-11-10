import json,pathlib,hashlib,math,re,collections
import chess
P=pathlib.Path
read=lambda p:json.loads(P(p).read_text())
sha=lambda p:hashlib.sha256(P(p).read_bytes()).hexdigest()
repo=P('/Users/johnweathers/.codex/worktrees/chess-adversarial-assessment/mShogi')
counts=collections.Counter();totals={};problems=[]
def check(cond,msg):
 if not cond:problems.append(msg)
def board(root):
 b=chess.Board(root['fen'])
 for u in root['history']:b.push_uci(u)
 check(b.is_valid(),'invalidroot');check(not root.get('expected_root_fen') or b.fen(en_passant='fen')==root['expected_root_fen'],'rootfen')
 return b
def pvcheck(b,pv):
 x=b.copy()
 for u in pv:
  m=chess.Move.from_uci(u);check(m in x.legal_moves,('pv',u,x.fen()));x.push(m)
 counts['pv_lines']+=1
 return x
for folder in [P('/tmp/msh155'),P('/tmp/msh155/intermediate'),P('/tmp/msh155/followup30'),P('/tmp/msh155/smoke'),P('/tmp/msh155-adjudication')]:
 m=read(folder/'manifest.json');sf=folder.name=='msh155-adjudication';runs=[json.loads(l) for l in (folder/'runs.jsonl').read_text().splitlines()]
 totals[folder.name]={k:sum(r[k] for r in runs) for k in ['charged_active_seconds','cpu_seconds','process_wall_seconds']};totals[folder.name]['attempts']=len(runs)
 check(sha(m['panel_path'])==m['panel_sha256'],('panelhash',str(folder)))
 driver=P('/tmp/msh155-adjudication-driver.py') if sf else folder/'driver.py'
 check(sha(driver)==m['driver_sha256'],('driverhash',str(folder)))
 if sf:check(sha(m['binary_path'])==m['binary_sha256'],'SFbinary')
 else:
  for name,h in m['binary_sha256'].items():check(sha(m['binary_paths'][name])==h,('binary',name))
  for name,h in m['native_provenance']['source_hashes'].items():check(sha(repo/name)==h,('source',name));counts['source_hash_checks']+=1
 agg=0
 for run in runs:
  counts['attempts']+=1;agg+=run['charged_active_seconds'];check(abs(agg-run['aggregate_charged_seconds'])<1e-8,'aggregate')
  for name,h in run['artifact_sha256'].items():check(sha(folder/name)==h,('artifact',str(folder/name)));counts['artifact_hashes']+=1
  index=run['query_index'] if sf else run['index'];stem=folder/(f'query-{index:03d}' if sf else f"cell-{index:03d}-{run['engine']}")
  cell=read(str(stem)+'.json');b=board(cell['root'])
  if run['status']!=0:
   counts['failed']+=1;check(sf and index==0 and run['result_path'] is None,'unexpectedfailure');check(': << go ' not in P(str(stem)+'.raw-uci.log').read_text(),'failureaftergo');continue
  r=read(run['result_path']);check(abs(r['active_search_seconds']-run['charged_active_seconds'])<1e-8,'activecharge')
  check(run['process_wall_seconds']>=run['charged_active_seconds']>=0,'timingorder')
  if r.get('result'):
   counts['native']+=1;n=r['result'];check(n['mode']=='Conventional' and n['depth_limit']==128 and n['node_limit'] is None and n['table_entries']==4096 and n['time_ms']==round(cell['seconds']*1000),'nativeconfig')
   check('true' not in n['diagnostics'] and n['diagnostics'].count('false')==6,'diagnostics')
   check(n['root_unchanged'] and n['pv_native_admitted'] and n['fen']==b.fen(en_passant='fen'),'nativeassertions')
   raw=read(str(stem)+'.native.jsonl');check(all(n[k]==v for k,v in raw.items() if k not in ['iterations']),'nativeraw')
   st={k:int(v) for k,v in re.findall(r'(\w+): (\d+)',n['stats'])};check(st==n['stats_parsed'],'statsparse');check(n['nodes']==st['main_nodes']+st['q_nodes'],'visitcount')
   check(n['completed_depth']==max(i['depth'] for i in n['iterations'] if i['completed']),'completeddepth')
   pv=[q['uci'] for q in n['pv'] if q['kind']=='move'];bb=pvcheck(b,pv);check(not pv or pv[0]==n['move'],'nativefirst');check(bb.fen(en_passant='fen')==n['pv_final_fen'],'nativepvfen')
  else:
   counts['uci_success']+=1;move=r['bestmove'];check(chess.Move.from_uci(move) in b.legal_moves,'bestlegal');raw=P(str(stem)+'.raw-uci.log').read_text();check(raw.count(': << go ')==1 and raw.count(': >> bestmove ')==1,'onequery')
   check(': << ucinewgame' in raw,'coldnewgame');check(': >> bestmove '+move in raw,'rawbest')
   if sf:
    check(r['configured_options']=={'Threads':1,'Hash':16,'SyzygyPath':'','UCI_LimitStrength':False},'SFconfig');events=r['events'];aligned=r['last_nonbound_event_matching_bestmove'];expected=[e for e in events if 'score' in e and e.get('pv') and e.get('multipv',1)==1 and not e.get('lowerbound') and not e.get('upperbound') and e['pv'][0]==move]
    check(not r['bestmove_alignment_issue'],'SFalignmentissue')
   else:
    check(r['configured_options']['Hash']=={'goldfish':16,'oxidation':64}[run['engine']],'refhash')
    events=r['events'];aligned=r['last_nonbound_scored_event_matching_bestmove'];expected=[e for e in events if 'score' in e['info'] and e['info'].get('pv') and e['info'].get('multipv',1)==1 and not e['info'].get('lowerbound') and not e['info'].get('upperbound') and e['info']['pv'][0]==move]
   check(aligned==(expected[-1] if expected else None),'alignedfield')
   disk=[json.loads(l) for l in P(str(stem)+'.events.jsonl').read_text().splitlines()];check(events==disk,'eventstream')
   for e in events:
    info=e if sf else e['info']
    if info.get('pv'):pvcheck(b,info['pv'])
    counts['uci_events']+=1
summary=read('/tmp/msh155/all-summary.json')
for r in summary['rows']:
 source=read(r['evidence']);n=source.get('result');check(r['move']==(n['move'] if n else source['bestmove']),'summarymove')
 check(r['active_seconds']==source['active_search_seconds'],'summaryactive')
 if n:
  last=max(i['elapsed_us'] for i in n['iterations'] if i['completed']);check(abs(r['incomplete_iteration_seconds']-max(0,n['elapsed_ms']/1000-last/1e6))<1e-8,'incomplete');check(r['depth']==n['completed_depth'] and r['nodes']==n['nodes'],'summarynative')
 else:
  e=source['last_nonbound_scored_event_matching_bestmove'];check(r['depth']==e['info'].get('depth'),'summaryreference')
for k,v in summary['resources'].items():
 for a,b in [('active','charged_active_seconds'),('cpu','cpu_seconds'),('wall','process_wall_seconds')]:check(v[a]==totals[k][b],('summaryresources',k,a))
output={'counts':dict(counts),'totals':totals,'grand_totals':{k:sum(t[k] for t in totals.values()) for k in ['charged_active_seconds','cpu_seconds','process_wall_seconds','attempts']},'problems':problems}
P('/tmp/msh155-harness-verification.json').write_text(json.dumps(output,indent=2)+'\n');print(json.dumps(output,indent=2))
