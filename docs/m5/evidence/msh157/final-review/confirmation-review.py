import json,pathlib,hashlib,collections,importlib.util
R=pathlib.Path('/tmp/msh157-runs');O=pathlib.Path('/tmp/msh157-final-review');panel=json.loads((R/'quiet-confirmation-panel.json').read_text());selection=json.loads((R/'quiet-confirmation-selection.json').read_text());assert hashlib.sha256((R/'quiet-confirmation-panel.json').read_bytes()).hexdigest()==selection['panel_sha256']
def load(d):return {r['root_id']:r for r in map(json.loads,(R/d/'runs.jsonl').read_text().splitlines())}
spec=importlib.util.spec_from_file_location('g',R/'grade.py');g=importlib.util.module_from_spec(spec);spec.loader.exec_module(g)
rows=[]
for root in panel:
 result={'id':root['id']}
 for side in ['predecessor','candidate']:
  original=load('heldout-'+side)[root['id']];repeat=load('quiet-'+side)[root['id']]; assert repeat['status']==0
  a=original['result']['result'];b=repeat['result']['result'];result[side]={'original_move':a['move'],'repeat_move':b['move'],'same_move':a['move']==b['move'],'original_depth':a['completed_depth'],'repeat_depth':b['completed_depth'],'original_visits':a['nodes'],'repeat_visits':b['nodes']}
 result['repeat_pair']=g.compare(root,result['predecessor']['repeat_move'],result['candidate']['repeat_move']);rows.append(result)
out={'scope':'Timestamp-selected19roots; original80rootprimaryretained. Repeat is sensitivity evidence, not independentextra sample.','rows':rows,'within_version_move_changes':{s:sum(not r[s]['same_move']for r in rows)for s in ['predecessor','candidate']},'repeat_pair_counts':dict(collections.Counter(r['repeat_pair']['outcome_change']for r in rows))}
(O/'quiet-confirmation.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items()if k!='rows'},indent=2))
