#!/usr/bin/env python3
"""Pure rubric grading, never a chess engine or a proof generator.
Unlisted soft alternatives remain unresolved. Scores/PVs do not change move grades.
"""
import argparse, collections, json, pathlib

def grade(root,move):
 e=root['expectation'];tier=e['tier'];out={'tier':tier,'move':move,'judgment':'unresolved','value':None}
 if move is None:return {**out,'reason':'no board move; claim/terminal/failure requires separate outcome review'}
 if tier=='exact-tablebase-wdl':
  values=e['move_wdl'];assert all(v in (-1,0,1) for v in values.values())
  if move not in values:return {**out,'judgment':'invalid-or-incomplete-exact-map','reason':'move absent from asserted complete exact map'}
  value=values[move];assert value<=e['root_wdl'],'inconsistent exact root value'
  return {**out,'judgment':'exact-preserved' if value==e['root_wdl'] else 'exact-worsened','value':value,'root_value':e['root_wdl']}
 if tier in ('exact-mate-proof','exact-mate-in-one'):
  # A missed immediate-mate objective does not establish a lost game-theoretic win.
  mates=e['preferred_moves'] if tier=='exact-mate-in-one' else e['mate_moves']
  legal=e.get('legal_moves',[])
  complete=(e.get('all_legal_roots_enumerated')==len(legal) and bool(legal)) if tier=='exact-mate-in-one' else e.get('complete',False)
  if move in mates:return {**out,'judgment':'exact-goal-met','value':1,'goal':'immediate mate' if tier=='exact-mate-in-one' else 'stated bounded mate'}
  if complete and move in legal:return {**out,'judgment':'exact-goal-missed','value':0,'reason':'missed stated mate objective, not proof of losing a win'}
  if complete:return {**out,'judgment':'invalid-or-incomplete-exact-map','reason':'move absent from exhaustive legal root set'}
  return {**out,'reason':'not in proved mating set; completeness/legal coverage not established'}
 if tier=='published-strategic-preference':
  if move in e['published_points']:return {**out,'judgment':'published-points','value':e['published_points'][move]}
  return {**out,'reason':'unlisted STS alternative is not an automatic zero'}
 if tier in ('published-puzzle-preference','finite-reference-opinion'):
  if move in e['preferred_moves']:return {**out,'judgment':'published-preferred' if tier.startswith('published') else 'reference-preferred'}
  return {**out,'reason':'unlisted alternative needs independent continuation adjudication'}
 raise ValueError('unsupported frozen rubric tier: '+tier)

def compare(root,before,after):
 a=grade(root,before);b=grade(root,after);out={'id':root['id'],'tier':a['tier'],'before':a,'after':b,'decision_change':'unchanged' if before==after else 'changed'}
 if before is None or after is None:return {**out,'decision_change':'unresolved','outcome_change':'unresolved'}
 if a['judgment']=='invalid-or-incomplete-exact-map' or b['judgment']=='invalid-or-incomplete-exact-map':return {**out,'outcome_change':'invalid-exact-comparison'}
 if before==after:return {**out,'outcome_change':'unchanged-decision'}
 if a['value'] is not None and b['value'] is not None:
  direction='improved' if b['value']>a['value'] else 'regressed' if b['value']<a['value'] else 'retained'
  authority='exact' if a['tier'].startswith('exact-') else 'published-preference'
  return {**out,'outcome_change':authority+'-'+direction}
 if a['judgment']==b['judgment'] and a['judgment'] in ('published-preferred','reference-preferred'):
  return {**out,'outcome_change':'changed-within-published-preferences'}
 return {**out,'outcome_change':'unresolved'}

def load_moves(path):
 rows=[json.loads(line) for line in pathlib.Path(path).read_text().splitlines()];result={}
 for row in rows:
  key=row['root_id'];assert key not in result,'duplicate root attempt; use a single frozen allowance/configuration'
  if row['status']!=0:result[key]={'move':None,'run_status':'failed'};continue
  native=row['result']['result'];assert native['id']==key
  result[key]={'move':native['move'],'run_status':'success'}
 return result

def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--panel',required=True);p.add_argument('--before',required=True);p.add_argument('--after');p.add_argument('--out',required=True);a=p.parse_args()
 panel=json.loads(pathlib.Path(a.panel).read_text());roots=panel if isinstance(panel,list) else panel['roots'];before=load_moves(a.before);after=load_moves(a.after) if a.after else None;ids={r['id'] for r in roots}
 assert len(ids)==len(roots) and set(before)==ids,'panel/run roots must match exactly'
 if after is not None:assert set(after)==ids
 rows=[]
 for root in roots:
  key=root['id']
  if after is None:row={'id':key,**grade(root,before[key]['move']),'run_status':before[key]['run_status']}
  else:
   row=compare(root,before[key]['move'],after[key]['move'])
   if before[key]['run_status']!='success' or after[key]['run_status']!='success':row['outcome_change']='unresolved-run-failure'
  rows.append({**row,'source_kind':root.get('source_kind'),'stratum':root.get('stratum')})
 summary=collections.Counter((r['tier'],r.get('outcome_change',r.get('judgment'))) for r in rows)
 pathlib.Path(a.out).write_text(json.dumps({'rows':rows,'summary':[{'tier':k[0],'judgment':k[1],'count':v} for k,v in sorted(summary.items())],'limitations':'Separate authority tiers; published/reference preference is not proved correctness or playing strength.'},indent=2)+'\n')
if __name__=='__main__':main()
