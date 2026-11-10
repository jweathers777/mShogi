import json,re,statistics
from pathlib import Path
root=Path(__file__).parent/'architecture'
if not root.exists(): root=Path('/tmp/msh-adversarial-architecture-inputs')
xs=[json.loads(l) for l in (root/'panel-1s.out.jsonl').read_text().splitlines()]
ss=[{k:int(v) for k,v in re.findall(r'(\w+): (\d+)',x['stats'])} for x in xs]
out={'scope':'12 deliberately selected historical roots replayed on d53f344; descriptive only','metrics':{},'rows':[]}
for label,n,d in [('main_first_move_share_of_cutoffs','main_first_move_cutoffs','main_beta_cutoffs'),('q_first_move_share_of_cutoffs','q_first_move_cutoffs','q_beta_cutoffs'),('main_tt_cutoffs_per_probe','tt_cutoffs','tt_probes'),('context_hits_per_position_hit','tt_context_hits','tt_position_hits')]:
 r=[s[n]/s[d] for s in ss if s[d]];out['metrics'][label]={'min':min(r),'median':statistics.median(r),'max':max(r)}
for x,s in zip(xs,ss):
 last=next(i for i in reversed(x['iterations']) if i['completed'])
 out['rows'].append({'id':x['id'],'move':x['move'],'score':x['score'],'depth':x['completed_depth'],'incomplete_iteration_wall_share':1-last['elapsed_us']/(x['elapsed_ms']*1000),'defence_reply_probes':s['defence_reply_probes'],'see_probes':s['see_probes']})
print(json.dumps(out,indent=2))
