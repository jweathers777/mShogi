"""Extract descriptive actual UCI telemetry; no chess rules or strength inference."""
import gzip,json,statistics,hashlib
from pathlib import Path
base=Path(__file__).resolve().parent
all_games=[]
def info(tokens):
 d={}
 for key in ['depth','seldepth','nodes','time','nps']:
  if key in tokens:
   try:d[key]=int(tokens[tokens.index(key)+1])
   except (ValueError,IndexError):pass
 if 'score' in tokens:
  i=tokens.index('score')
  try:d['score_kind']=tokens[i+1];d['score']=int(tokens[i+2])
  except (ValueError,IndexError):pass
 if 'pv' in tokens:d['pv']=tokens[tokens.index('pv')+1:]
 return d
for path in sorted(base.glob('pair-*/report.json.gz')):
 outer=json.loads(gzip.decompress(path.read_bytes()));r=json.loads(outer['report_json'])
 for index,g in enumerate(r['games']):
  active={};requests=[]
  for e in g['transcript']['entries']:
   t=e['text'].split();eng=e['engine']
   if e['direction']=='command' and t and t[0]=='go':active[eng]={'engine':eng,'version':e['game_version'],'go_at_ns':e['at_ns'],'iterations':[]}
   elif e['direction']=='response' and t and t[0]=='info' and 'depth' in t and eng in active:
    latest=info(t);active[eng]['latest']=latest;active[eng]['iterations'].append(latest)
   elif e['direction']=='response' and t and t[0]=='bestmove' and eng in active:
    q=active.pop(eng);q['bestmove']=t[1];q['wall_ms']=(e['at_ns']-q['go_at_ns'])/1e6;requests.append(q)
  m=json.loads(g['match_record_json']);initial=json.loads(m['initial_record_json'])
  prefixes=list(r['config']['openings'][0]['moves']);moves=[]
  for event in m['events']:
   command=event['request']['command']
   if isinstance(command,dict) and 'move' in command:moves.append({'version':event['request']['version'],'side':event['request']['actor'],'action':command['move']})
  row={'opening':r['config']['openings'][0]['id'],'pairing':index,'mshogi_side':'white' if g['engines_by_color'][0]==0 else 'black','stop':g['stop'],'conclusion':g['conclusion'],'initial_fen':initial['initial_fen'],'prefix':prefixes,'moves':moves,'requests':requests,'source_report':path.name,'source_pair':path.parent.name,'source_report_sha256':hashlib.sha256(gzip.decompress(path.read_bytes())).hexdigest(),'metrics':{}}
  for eng in [0,1]:
   qs=[q for q in requests if q['engine']==eng];cp=[q for q in qs if q.get('latest',{}).get('score_kind')=='cp']
   metrics={'requests':len(qs),'cp_scored_requests':len(cp)}
   for key in ['depth','seldepth','nodes','time','nps']:
    vals=[q['latest'][key] for q in qs if key in q.get('latest',{})];cvals=[q['latest'][key] for q in cp if key in q.get('latest',{})]
    metrics[key]={'all_min':min(vals),'all_median':statistics.median(vals),'all_max':max(vals),'cp_min':min(cvals) if cvals else None,'cp_median':statistics.median(cvals) if cvals else None,'cp_max':max(cvals) if cvals else None} if vals else None
   wall=[q['wall_ms'] for q in qs];metrics['wall_ms']={'min':min(wall),'median':statistics.median(wall),'max':max(wall)} if wall else None
   row['metrics'][str(eng)]=metrics
  all_games.append(row)
aggregate={}
for engine in [0,1]:
 qs=[q for g in all_games for q in g['requests'] if q['engine']==engine];cp=[q for q in qs if q.get('latest',{}).get('score_kind')=='cp']
 aggregate[str(engine)]={'requests':len(qs),'cp_scored_requests':len(cp)}
 for key in ['depth','seldepth','nodes','time']:
  values=[q['latest'][key] for q in cp if key in q.get('latest',{})]
  aggregate[str(engine)][key]={'min':min(values),'median':statistics.median(values),'max':max(values)} if values else None
 wall=[q['wall_ms'] for q in qs]
 aggregate[str(engine)]['wall_ms_all']={'min':min(wall),'median':statistics.median(wall),'max':max(wall)} if wall else None
out={'aggregate_cp_scored_turns':aggregate,'scope':'Descriptive actual protocol observations, not significance/CI/Elo; nominal depths are engine-specific, cp/mate scores are not cross-engine calibrated. Source reports/native replay own all rules/outcomes.','games':all_games}
(base/'observations.json.gz').write_bytes(gzip.compress((json.dumps(out,indent=2)+'\n').encode(),mtime=0))
for g in all_games:
 print(g['opening'],g['pairing'],g['mshogi_side'],len(g['moves']),g['conclusion'])
 for eng in ['0','1']:print(' engine',eng,json.dumps(g['metrics'][eng]))
 print('mShogi cp/mate timeline:')
 print(' '.join(f"v{q['version']}:{q['bestmove']}:{q.get('latest',{}).get('score_kind')}{q.get('latest',{}).get('score')}" for q in g['requests'] if q['engine']==0))
