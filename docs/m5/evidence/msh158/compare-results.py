#!/usr/bin/env python3
"""Read-only frozen-version comparisons. Writes summaries/planned reference pairs; never runs engines."""
import argparse,collections,datetime,hashlib,importlib.util,json,pathlib,re
import chess
P=pathlib.Path
BASE=P('/tmp/msh158');REPO=P('/Users/johnweathers/.codex/worktrees/chess-adversarial-assessment/mShogi')
def read(p):return json.loads(P(p).read_text())
def sha(p):return hashlib.sha256(P(p).read_bytes()).hexdigest()
freeze=read(REPO/'docs/m5/evidence/msh158/candidate-freeze.json')
gradepath=BASE/'runs/grade.py';assert sha(gradepath)==freeze['grader_sha256']
spec=importlib.util.spec_from_file_location('frozen_grade',gradepath);grader=importlib.util.module_from_spec(spec);spec.loader.exec_module(grader)
exposure=read(REPO/'docs/m5/evidence/msh157/final-review/freshness-review.json');excluded=set(exposure['affected_family'])
def board(root):
 b=chess.Board(root.get('fen',chess.STARTING_FEN))
 for u in root.get('history',[]):b.push_uci(u)
 assert b.fen(en_passant='fen')==root['expected_root_fen'];return b

def rows(directory):
 p=P(directory)/'runs.jsonl'
 if not p.exists():return {},False
 raw=p.read_bytes();complete_tail=raw.endswith(b'\n');parts=raw.splitlines()
 if not complete_tail:parts=parts[:-1]
 result={}
 for line in parts:
  r=json.loads(line);assert r['root_id'] not in result
  result[r['root_id']]=r
 return result,complete_tail

def native(row):
 if row['status']!=0:return None
 if row.get('result') is not None:return row['result']['result']
 p=P(row['result_path']);obj=read(p);return obj.get('result',obj)

def verify(root,row,want_hash,seconds):
 assert row['binary_sha256']==want_hash and row['seconds']==seconds
 assert not any(row['diagnostics'].values())
 n=native(row)
 if n is None:return {'status':'failed','move':None,'decision':None,'reason':row['status']}
 b=board(root);assert n['id']==root['id'] and n['fen']==b.fen(en_passant='fen')
 assert n['root_unchanged'] and n['pv_native_admitted'] and n['node_limit'] is None
 assert n['time_ms']==round(seconds*1000) and n['table_entries']==4096 and n['mode']=='Conventional'
 move=n['move'];san=None
 if move is not None:
  action=chess.Move.from_uci(move);assert action in b.legal_moves;san=b.san(action)
 pv=[]
 for choice in n['pv']:
  if choice['kind']=='claim':
   pv.append({'claim':choice});break
  assert choice['kind']=='move';a=chess.Move.from_uci(choice['uci']);assert a in b.legal_moves
  pv.append({'uci':choice['uci'],'san':b.san(a)});b.push(a)
 assert b.fen(en_passant='fen')==n['pv_final_fen']
 return {'status':'success','move':move,'san':san,'decision':n['decision'],'score':n['score'],'completed_depth':n['completed_depth'],'nodes':n['nodes'],'elapsed_ms':n['elapsed_ms'],'pv':pv,'root_unchanged':True,'pv_native_admitted':True,'pv_python_legal':True,'native_pv_outcome':n['pv_native_outcome'],'search_status':n['status'],'iteration_summary':[{'depth':i['depth'],'completed':i['completed'],'elapsed_us':i['elapsed_us']} for i in n['iterations']]}

def aggregate(rs):
 c=collections.Counter((r['tier'],r['outcome_change']) for r in rs)
 return [{'tier':a,'outcome':b,'count':n} for (a,b),n in sorted(c.items())]

def compare(part,panelpath,old,new,seconds,panel_hash):
 assert sha(panelpath)==panel_hash;panel=read(panelpath);oldrows,oldtail=rows(old);newrows,newtail=rows(new)
 ids={r['id'] for r in panel};assert set(oldrows)<=ids and set(newrows)<=ids
 output=[]
 for root in panel:
  key=root['id']
  if key not in oldrows or key not in newrows:continue
  before=verify(root,oldrows[key],freeze['predecessor_binary_sha256'],seconds)
  after=verify(root,newrows[key],freeze['candidate_binary_sha256'],seconds)
  comp=grader.compare(root,before['move'],after['move'])
  if before['status']!='success' or after['status']!='success':comp['outcome_change']='unresolved-run-failure'
  output.append({**comp,'partition':part,'source_kind':root['source_kind'],'stratum':root['stratum'],'before_observation':before,'after_observation':after,'prior_exposed_family':key in excluded})
 complete=len(output)==len(panel) and oldtail and newtail
 summary={'state':'complete' if complete else 'partial','planned':len(panel),'compared':len(output),'missing_before':sorted(ids-set(oldrows)),'missing_after':sorted(ids-set(newrows)),'summary_by_authority':aggregate(output),'changed':[r for r in output if r['decision_change']!='unchanged'],'exact_regressions':[r['id'] for r in output if r['outcome_change']=='exact-regressed'],'invalid_exact_comparisons':[r['id'] for r in output if r['outcome_change']=='invalid-exact-comparison'],'unresolved_ids':[r['id'] for r in output if r['outcome_change'].startswith('unresolved')],'family_exclusion_sensitivity':{'family':exposure['family_key'],'excluded_ids':sorted(excluded),'compared':sum(not r['prior_exposed_family'] for r in output),'summary_by_authority':aggregate([r for r in output if not r['prior_exposed_family']])},'rows':output,'provenance':{'panel_sha256':sha(panelpath),'predecessor_directory':str(old),'candidate_directory':str(new),'predecessor_ledger_sha256':sha(P(old)/'runs.jsonl') if (P(old)/'runs.jsonl').exists() else None,'candidate_ledger_sha256':sha(P(new)/'runs.jsonl') if (P(new)/'runs.jsonl').exists() else None}}
 return summary,panel

def select(reports,panels,out):
 assert all(reports[k]['state']=='complete' for k in ['development','heldout']),'selection requires complete primary partitions'
 selected=[];queries=[];roots=[];reused=[]
 oldref=P('/tmp/msh157-reference-initial/manifest.json');om=read(oldref)
 for part in ['development','heldout']:
  lookup={x['id']:x for x in panels[part]};eligible=[]
  for row in reports[part]['changed']:
   if row['tier'].startswith('exact-') or row['before_observation']['status']!='success' or row['after_observation']['status']!='success':continue
   b=row['before_observation']['move'];a=row['after_observation']['move']
   if b is None or a is None or b==a:continue
   root=lookup[row['id']];pref=set(root['expectation']['preferred_moves']);priority=0 if b in pref and a not in pref else 1 if b not in pref and a in pref else 2
   eligible.append((priority,row['id'],root,b,a))
  for priority,key,root,b,a in sorted(eligible,key=lambda x:(x[0],x[1]))[:8]:
   selected.append({'partition':part,'id':key,'priority':['lost published preference','gained published preference','other soft change'][priority],'before':b,'after':a});roots.append(root)
   if key=='lichess-puzzle-28pJK' and {b,a}=={'f5f4','f5c8'}:
    oldroot=om['roots'][key]
    assert all(oldroot.get(k)==root.get(k) for k in ['fen','history','expected_root_fen'])
    qs=[q for q in om['queries'] if q['root_id']==key and q['root_move'] in [b,a]]
    assert len(qs)==2 and all(q['seconds']==3 for q in qs)
    reused.append({'id':key,'manifest':str(oldref),'manifest_sha256':sha(oldref),'queries':qs,'reason':'same full-history root and both actual moves under recorded equal3s reference conditions; inspect retained bound/PV receipts for final opinion'})
   else:
    for role,move in [('predecessor',b),('candidate',a)]:queries.append({'query_index':len(queries),'root_id':key,'root_move':move,'seconds':3,'role':role})
 assert len(queries)*3<=96
 panel_bytes=(json.dumps(roots,indent=2)+'\n').encode();panel_sha=hashlib.sha256(panel_bytes).hexdigest()
 files={'reference-selection.json':{'panel_sha256':panel_sha,'local_only_contains_source_rows':True,'state':'FROZEN_FINAL_REFERENCE_PAIRS','selected':selected,'reused':reused,'queries':queries,'nominal_active_seconds':len(queries)*3,'rule':freeze['judgment'],'unselected_changed_soft_ids':[r['id'] for part in ['development','heldout'] for r in reports[part]['changed'] if not r['tier'].startswith('exact-') and r['id'] not in {x['id'] for x in selected}]},'reference-panel.json':roots}
 for name,obj in files.items():
  path=out/name;assert not path.exists(),'selection already exists; preserve first freeze'
  path.write_text(json.dumps(obj,indent=2)+'\n')
 assert sha(out/'reference-panel.json')==panel_sha

def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--out',type=P,default=BASE/'comparison');p.add_argument('--select',action='store_true');a=p.parse_args();a.out.mkdir(exist_ok=True)
 configs=[('development',P('/tmp/msh156-data/development.json'),P('/tmp/msh157-runs/candidate-development'),BASE/'runs/candidate-development',5,freeze['development_sha256']),('fast',P('/tmp/msh157-runs/fast-panel.json'),P('/tmp/msh157-runs/fast-candidate'),BASE/'runs/candidate-fast',1,freeze['fast_panel_sha256']),('heldout',P('/tmp/msh156-data/heldout.json'),P('/tmp/msh157-runs/heldout-candidate'),BASE/'runs/candidate-heldout',5,freeze['heldout_sha256'])]
 reports={};panels={}
 for part,*args in configs:
  reports[part],panels[part]=compare(part,*args)
  (a.out/(part+'.json')).write_text(json.dumps(reports[part],indent=2)+'\n')
 total=[r for part in ['development','heldout'] for r in reports[part]['rows']]
 summary={'created_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'state':'complete' if all(r['state']=='complete' for r in reports.values()) else 'partial','partitions':{k:{f:v for f,v in r.items() if f not in ['rows','changed']}|{'changed_count':len(r['changed']),'changes':[{'id':x['id'],'before':x['before_observation']['move'],'after':x['after_observation']['move'],'outcome':x['outcome_change']} for x in r['changed']]}for k,r in reports.items()},'primary_summary':aggregate(total),'primary_without_exposed_family':aggregate([r for r in total if not r['prior_exposed_family']]),'limitations':'All changes retained. Authority tiers remain separate; unresolved alternative is not failure. Depth/work/score changes do not establish decision improvement. Source/candidate frozen; no retuning after heldout reveal.'}
 (a.out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
 if a.select:select(reports,panels,a.out)
 print(json.dumps({'state':summary['state'],'partitions':{k:{'compared':v['compared'],'planned':v['planned'],'changes':len(v['changed'])}for k,v in reports.items()},'engines_executed':0},indent=2))
if __name__=='__main__':main()
