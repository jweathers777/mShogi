"""Derive descriptive telemetry/positions from preserved native reports, not rules/results."""
import gzip,json,statistics
from pathlib import Path
import assessment as a


def parse_info(words):
 d={}
 for key in ['depth','seldepth','nodes','time','nps']:
  if key in words:
   try:d[key]=int(words[words.index(key)+1])
   except (ValueError,IndexError):pass
 if 'score' in words:
  i=words.index('score')
  try:d.update(score_kind=words[i+1],score=int(words[i+2]))
  except (ValueError,IndexError):pass
 if 'pv' in words:d['pv']=words[words.index('pv')+1:]
 return d


def inspect():
 ledger=a.read(a.HERE/'verification.json');games=[]
 for receipt in ledger['runs']:
  if receipt['phase']=='smoke' or not receipt['complete']:continue
  outer=json.loads(gzip.decompress((a.HERE/receipt['folder']/'report.json.gz').read_bytes()))
  report=json.loads(outer['report_json'])
  for game in report['games']:
   active={};turns=[];history=list(report['config']['openings'][0]['moves'])
   for event in game['transcript']['entries']:
    words=event['text'].split();engine=event['engine']
    if event['direction']=='command' and words and words[0]=='go':
     active[engine]={'engine':engine,'version':event['game_version'],'start_ns':event['at_ns']}
    elif event['direction']=='response' and words and words[0]=='info' and engine in active and 'depth' in words:
     parsed=parse_info(words)
     active[engine]['last_work_report']=parsed
     if 'score_kind' in parsed:
      active[engine]['latest']=parsed
      active[engine]['last_scored_report']=parsed
    elif event['direction']=='response' and words and words[0]=='bestmove' and engine in active:
     item=active.pop(engine);item.update(move=words[1],wall_ms=(event['at_ns']-item['start_ns'])/1e6,history=list(history));history.append(words[1]);turns.append(item)
   games.append({'pair':receipt['folder'],'opponent':receipt['opponent'],'phase':receipt['phase'],
                 'pairing':game['pairing'],'engines_by_color':game['engines_by_color'],'conclusion':game['conclusion'],'turns':turns})
 out={'games':games,'descriptive_metrics':{}}
 for opponent in {g['opponent'] for g in games}:
  out['descriptive_metrics'][opponent]={}
  for player in [0,1]:
   ts=[t for g in games if g['opponent']==opponent for t in g['turns'] if t['engine']==player]
   cp=[t for t in ts if t.get('latest',{}).get('score_kind')=='cp'];metrics={'turns':len(ts),'cp_turns':len(cp)}
   for key in ['depth','seldepth']:
    values=[t['last_scored_report'][key] for t in cp if key in t.get('last_scored_report',{})]
    metrics[key]={'min':min(values),'median':statistics.median(values),'max':max(values)} if values else None
   for key in ['nodes','time','nps']:
    values=[t['last_work_report'][key] for t in cp if key in t.get('last_work_report',{})]
    metrics['last_reported_'+key]={'min':min(values),'median':statistics.median(values),'max':max(values)} if values else None
   metrics['depth_meaning']='Last scored nominal report; reference engines may emit root updates before a whole iteration completes. Final work report is separate.'
   values=[t['wall_ms'] for t in ts];metrics['wall_ms']={'min':min(values),'median':statistics.median(values),'max':max(values)} if values else None
   out['descriptive_metrics'][opponent][str(player)]=metrics
 return out


if __name__=='__main__':
 with a.lock():
  result=inspect();(a.HERE/'observations.json.gz').write_bytes(gzip.compress(json.dumps(result).encode(),mtime=0))
  print(json.dumps(result['descriptive_metrics'],indent=2))
