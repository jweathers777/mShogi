import json,hashlib,collections,chess
from pathlib import Path
p=Path('/tmp/msh156-data');a=json.loads((p/'development.json').read_text());b=json.loads((p/'heldout.json').read_text());near=json.loads((p/'near-pairs.json').read_text());assert len(near)==1
left=next(x for x in a if x['id']==near[0][2]);right=next(x for x in b if x['id']==near[0][3]);swap=sorted([x for x in a if x['source_kind']=='tablebase'and x['stratum']==right['stratum']and x['expectation']['root_wdl']==0 and x is not left],key=lambda x:x['id'])[0]
a.remove(swap);b.remove(right);a.append(right);b.append(swap)
for x in [left,right]:x['family_key']='near-root-cluster-001'
for part,arr in [('development',a),('heldout',b)]:
 for x in arr:
  x['partition']=part
  if x['expectation']['tier']=='exact-mate-in-one':x['expectation']['legal_moves']=[m.uci() for m in chess.Board(x['expected_root_fen']).legal_moves]
 (p/(part+'.json')).write_text(json.dumps(sorted(arr,key=lambda x:x['id']),indent=2)+'\n')
m=json.loads((p/'metadata.json').read_text())
for part,arr in [('development',a),('heldout',b)]:
 m[part]['sha256']=hashlib.sha256((p/(part+'.json')).read_bytes()).hexdigest();m[part]['history']=dict(collections.Counter(x['history_status']for x in arr))
(p/'metadata.json').write_text(json.dumps(m,indent=2)+'\n')
