#!/usr/bin/env python3
import argparse, pathlib,json,hashlib,datetime
p=argparse.ArgumentParser();p.add_argument('--snapshot',action='store_true');p.add_argument('--out',default='/tmp/msh155-adjudication-candidates.json');a=p.parse_args()
root=pathlib.Path('/tmp/msh155');manifest=json.loads((root/'manifest.json').read_text());panel=json.loads(pathlib.Path(manifest['panel_path']).read_text());anchors={'historical-22Nf5':'d3f5','historical-24h4':'h3h4','fresh-sts-undermine-004':'b4b5','historical-23Rf6':'d6e4','historical-34Rc5':'e2e3'}
candidates={id:{'canonical_anchor':anchor,'selected_moves':{},'sources':[]} for id,anchor in anchors.items()};seen=set()
scopes=[root,root/'followup30',root/'intermediate']
for cellfile in sorted(p for scope in scopes if scope.exists() for p in scope.glob('cell-*.json')):
 if '.result.' in cellfile.name or '.native-' in cellfile.name:continue
 cell=json.loads(cellfile.read_text())
 if not all(k in cell for k in ['index','root','engine','seconds']):continue
 resultfile=cellfile.with_name(cellfile.stem+'.result.json')
 if not resultfile.exists():continue
 result=json.loads(resultfile.read_text())
 if cellfile.parent==root:seen.add(cell['index'])
 id=cell['root']['id']
 if id not in candidates:continue
 move=result.get('bestmove') if cell['engine']!='mshogi' else result['result']['move']
 if move:candidates[id]['selected_moves'].setdefault(move,[]).append({'engine':cell['engine'],'seconds':cell['seconds'],'cell_index':cell['index'],'scope':str(cellfile.parent.relative_to(root))})
 candidates[id]['sources'].append({'file':str(resultfile),'sha256':hashlib.sha256(resultfile.read_bytes()).hexdigest()})
missing=[c['index'] for c in manifest['cells'] if c['index'] not in seen]
if missing and not a.snapshot:raise SystemExit('Cannot freeze while baseline cells missing:'+str(missing))
queries=[]
for id in sorted(candidates):
 c=candidates[id];moves=set(c['selected_moves'])|{c['canonical_anchor']}
 if id=='historical-22Nf5':moves.add('e3f5')
 if id=='historical-23Rf6':moves.add('f8f6')
 c['candidate_moves']=sorted(moves);queries.append({'root_id':id,'root_move':None,'role':'unrestricted'})
 for move in sorted(moves):queries.append({'root_id':id,'root_move':move,'role':'canonical_anchor' if move==c['canonical_anchor'] else 'observed_or_prior_weakness'})
if len(queries)>28 and not a.snapshot:raise SystemExit('Candidate union exceeds approved28query ceiling; return list for coordinator decision')
seconds=3
for i,q in enumerate(queries):q['query_index']=i;q['seconds']=seconds
out={'schema':1,'state':'DRAFT_NOT_FROZEN' if a.snapshot else 'FROZEN_PENDING_COORDINATOR_APPROVAL_AND_ENGINE_SLOT','created_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'benchmark_panel_sha256':manifest['panel_sha256'],'baseline_missing_indices':missing,'included_scopes':[str(s) for s in scopes],'excluded_scope':'smoke','candidates':candidates,'queries':queries,'nominal_active_seconds':len(queries)*seconds,'hard_aggregate_active_seconds':100,'no_engine_calls_made_by_collector':True}
pathlib.Path(a.out).write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({'state':out['state'],'queries':len(queries),'seconds_each':seconds,'nominal_seconds':len(queries)*seconds,'out':a.out,'sha256':hashlib.sha256(pathlib.Path(a.out).read_bytes()).hexdigest()}))
