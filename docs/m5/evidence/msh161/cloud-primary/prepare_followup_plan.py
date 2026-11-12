"""Offline followup singletons, reusing completed equal-work played evidence."""
import argparse,copy,json,re
from pathlib import Path
import adjudication_control as c

def prepare(a):
 if not re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9_.-]{0,95}',a.plan_id):raise ValueError('Unsafe plan ID')
 old=c.read(a.prior_plan);c.validate_plan(old);store=c.core.LocalStore(a.cache)
 groups={}
 for j in old['jobs']:groups.setdefault((j['spec']['id'],j['game'],j['ply']),[]).append(j)
 bound={};queries={}
 def inspect(root):
  if root in queries:return queries[root]
  records=[]
  for j in groups[root]:
   result=c.core.verify_result(store,j);key=c.core.prefix(j)+'/success.json'
   ref={'key':key,'sha256':c.core.digest(store.get(key))};bound[key]=ref
   for name,artifact in result['artifacts'].items():
    if not(name.startswith('analysis/g') and '-attempt' in name):continue
    q=c.core.decode(store.get(artifact['key']))
    if q['decision']!=j['adjudication']['decision'] or q['nodes_requested']!=20000000 or q['engine_sha256']!=j['build']['engines']['stockfish19']['sha256'] or q['status']!='complete' or q['raw_truncated'] or q['exit_code']!=0 or not q['process_group_settled']:raise ValueError('Invalid prior query')
    allowed=[j['move']] if j['move'] is not None else [None,j['adjudication']['decision']['played']]
    if q['forced_move'] not in allowed:raise ValueError('Wrong prior forced root')
    if q['options']!=c.analysis.OPTIONS:raise ValueError('Wrong prior reference options')
    if q['forced_move'] and q['assessment']['bestmove']!=q['forced_move']:raise ValueError('Forced bestmove differs')
    if q['assessment']['score'] is not None and not q['pv_independent_legal']:raise ValueError('Prior PV not legal')
    if q['assessment']['score'] is None and 'no_nonbound_score_aligned_with_bestmove' not in q['assessment']['uncertainty']:raise ValueError('Missing score uncertainty')
    records.append({'query':q,'job':j['id'],'success':ref,'artifact':artifact})
  queries[root]=records;return records
 jobs=[];decisions=[];reused=[];selection_refs=[];seen=set()
 for file in a.selection:
  selection_refs.append({'file':Path(file).name,'sha256':c.core.file_hash(file)})
  for row in c.read(file):
   root=(row['pair'],row['game'],row['ply']);prior=inspect(root);template=groups[root][0];decision=template['adjudication']['decision'];played=decision['played']
   if played not in row['moves']:raise ValueError('Played comparison missing')
   played_records=[r for r in prior if r['query']['forced_move']==played]
   if len(played_records)!=1:raise ValueError('Unique prior played singleton required')
   evidence=played_records[0];reuse={k:evidence[k] for k in ['job','success','artifact']};reuse.update(pair=root[0],game=root[1],ply=root[2],played=played)
   supporting=row.get('supporting_root');support=inspect((supporting['pair'],supporting['game'],supporting['ply'])) if supporting else []
   refs={r['key']:r for r in template['adjudication']['consumed_successes']}
   for r in prior+support:refs[r['success']['key']]=r['success']
   new_moves=[]
   for move in row['moves']:
    if move==played:continue
    if move is None or (root,move) in seen:raise ValueError('Duplicate or unrestricted followup')
    if any(r['query']['forced_move']==move for r in prior):raise ValueError('Alternative already searched; reuse it')
    board=c.analysis.board_for(decision);board.push_uci(move);seen.add((root,move));new_moves.append(move)
    j=copy.deepcopy(template);j['move']=move
    j['adjudication'].update(plan_id=a.plan_id,reasons=[{'selection_file':Path(file).name,'reason':row['reason']}],consumed_successes=sorted(refs.values(),key=lambda r:r['key']),reused_played=reuse,prior_plan_sha256=c.core.file_hash(a.prior_plan),supporting_root=supporting)
    jobs.append(c.core.seal(j))
   if not new_moves:raise ValueError('No new alternative requested')
   reused.append(reuse);decisions.append({'pair':root[0],'game':root[1],'ply':root[2],'played':played,'moves':[played]+new_moves,'reasons':[row['reason']],'reused_played':reuse})
 plan=copy.deepcopy(old);plan.update(plan_id=a.plan_id,plan_prefix=old['experiment']+'/adjudication/'+a.plan_id,jobs=jobs,bins=c.balanced(jobs),decisions=decisions,selection_files=selection_refs,reused_queries=reused,prior_plan_sha256=c.core.file_hash(a.prior_plan),controller_sha256=c.core.file_hash(c.__file__),dispatcher_sha256=c.core.file_hash(c.HERE/'adjudication_dispatch.py'),followup_preparer_sha256=c.core.file_hash(__file__),note='Only new forced alternatives; prior equal-work played results are hash-bound and reused. No scientific conclusions before review.')
 refs={r['key']:r for j in jobs for r in j['adjudication']['consumed_successes']};plan['consumed_successes']=sorted(refs.values(),key=lambda r:r['key']);c.validate_plan(plan)
 out=Path(a.output);out.mkdir(exist_ok=False);c.save_new(out/'plan.json',plan);(out/'dispatcher.py').write_bytes((c.HERE/'adjudication_dispatch.py').read_bytes());c.save_new(out/'batch-request.json',c.batch_request(plan,out/'plan.json',out/'dispatcher.py'));c.save_new(out/'preflight.json',{'new_queries':len(jobs),'reused_played_queries':len(reused),'bins':len(plan['bins']),'plan_sha256':c.core.file_hash(out/'plan.json'),'status':'prepared_needs_independent_review'})
 return {'new_queries':len(jobs),'reused_played_queries':len(reused),'bins':len(plan['bins'])}

if __name__=='__main__':
 p=argparse.ArgumentParser()
 for name in ['prior-plan','cache','plan-id','output']:p.add_argument('--'+name,required=True)
 p.add_argument('--selection',action='append',required=True)
 print(json.dumps(prepare(p.parse_args()),indent=2))
