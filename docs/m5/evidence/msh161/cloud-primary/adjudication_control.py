#!/usr/bin/env python3
"""Prepare/submit/verify explicit cloud20M plans; prepare and local verify run no engines."""
import argparse,copy,json,os,re,subprocess,sys,time,concurrent.futures
from pathlib import Path
HERE=Path(__file__).resolve().parent;REPO=HERE.parents[4];STUDY=HERE.parent
sys.path[:0]=[str(REPO),str(STUDY)]
from tools.experiments import core,gcp
import analysis

def read(p):return core.decode(Path(p).read_bytes())
def save_new(p,value):
    with Path(p).open('xb') as f:f.write(core.encode(value))
def put_same(store,key,data):
    if not store.put(key,data) and store.get(key)!=data:raise ValueError('Immutable key conflict: '+key)
def balanced(jobs,n=8):
    bins=[[] for _ in range(n)];loads=[0]*n
    for j in sorted(jobs,key=lambda j:(-(2 if j.get('move') is None else 1),j['id'])):
        i=min(range(n),key=lambda i:(loads[i],i));bins[i].append(j['id']);loads[i]+=2 if j.get('move') is None else 1
    return bins

def prepare(a):
    if not re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9_.-]{0,95}',a.plan_id):raise ValueError('Unsafe plan ID')
    manifest=read(a.manifest);core.validate_manifest(manifest);index=read(a.index);store=core.LocalStore(a.cache)
    manifest_key=manifest['experiment']+'/manifest.json';manifest_bytes=store.get(manifest_key)
    if core.decode(manifest_bytes)!=manifest:raise ValueError('Local manifest differs from immutable original')
    consumed={};verified={};decisions={};selected={};selection_refs=[]
    def bind(item):
        job=item['job'];key=core.prefix(job)+'/success.json';raw=store.get(key)
        if core.digest(raw)!=item['success_sha256']:raise ValueError('Index success hash mismatch')
        if job['id'] not in verified:verified[job['id']]=core.verify_result(store,job)
        consumed[key]={'key':key,'sha256':core.digest(raw)}
        return verified[job['id']]
    for path in a.selection:
        selection_refs.append({'file':Path(path).name,'sha256':core.file_hash(path)})
        for row in read(path):
            pair=row['pair'];g=row['game'];ply=row['ply']
            if type(g)!=int or type(ply)!=int:raise ValueError('Invalid game/ply')
            if pair not in index:raise ValueError('Unknown cloud pair')
            p=index[pair];parent=p['game_pair'];screen=p['reference_screening']
            if parent['job'] not in manifest['jobs'] or screen['job'] not in manifest['jobs']:raise ValueError('Pair not in original manifest')
            bind(parent);bind(screen);refs=[consumed[core.prefix(v['job'])+'/success.json'] for v in [parent,screen]]
            artifact=parent['artifacts']['pair/report.json.gz'];report=Path(a.cache)/artifact['key']
            if row.get('report') and Path(row['report']).resolve()!=report.resolve():raise ValueError('Selected wrong cohort report')
            if pair not in decisions:
                r=analysis.load_report(report);ds,_=analysis.extract_decisions(r);decisions[pair]={(d['game'],d['ply']):d for d in ds}
            d=decisions[pair].get((g,ply))
            if d is None or d['origin']!='played':raise ValueError('Selection must identify played decision')
            moves=row['moves']
            if not isinstance(moves,list) or not moves or d['played'] not in moves and None not in moves:raise ValueError('Selection must include actual played move')
            for move in moves:
                if move is not None:analysis.check_move(move);board=analysis.board_for(d);board.push_uci(move)
            for prior in p['confirmations']:
                j=prior['job']
                if (j['game'],j['ply'])==(g,ply):
                    bind(prior);refs.append(consumed[core.prefix(j)+'/success.json'])
            key=(pair,g,ply);entry=selected.setdefault(key,{'decision':d,'moves':set(),'reasons':[],'screen':screen,'parent':parent,'refs':refs})
            entry['moves'].update(moves)
            if row.get('discovery'):entry['moves'].add(None)
            entry['reasons'].append({'selection_file':Path(path).name,'reason':row['reason']})
    if not selected:raise ValueError('Empty selection')
    jobs=[];rows=[]
    for (pair,g,ply),e in sorted(selected.items()):
        screen=e['screen'];base={k:copy.deepcopy(v) for k,v in screen['job'].items() if k not in ['id','schema','expected_outputs']}
        provenance={'plan_id':a.plan_id,'decision':e['decision'],'reasons':e['reasons'],'parent_success_sha256':e['parent']['success_sha256'],'screen_success_sha256':screen['success_sha256'],'consumed_successes':e['refs']}
        moves=set(e['moves'])
        if None in moves:moves.discard(e['decision']['played']) # discovery already queries played at20M
        for move in sorted(moves,key=lambda m:m or ''):
            j=core.seal(dict(base,kind='confirmation',nodes=20000000,game=g,ply=ply,move=move,adjudication=provenance,expected_outputs=['analysis/manifest.json','screen-v1.json']))
            core.validate_job(j);jobs.append(j)
        rows.append({'pair':pair,'game':g,'ply':ply,'played':e['decision']['played'],'moves':sorted(e['moves'],key=lambda m:m or ''),'reasons':e['reasons']})
    env=jobs[0]['environment']['cloud']
    if any(j['environment']['cloud']!=env for j in jobs):raise ValueError('Mixed cloud identities')
    config=gcp.validate_config(env['config']);prefix=manifest['experiment']+'/adjudication/'+a.plan_id
    plan={'schema':'msh161-adjudication-v1','plan_id':a.plan_id,'experiment':manifest['experiment'],'store_uri':config['store_uri'],'plan_prefix':prefix,'original_manifest':{'key':manifest_key,'sha256':core.digest(manifest_bytes)},'selection_files':selection_refs,'review_index_sha256':core.file_hash(a.index),'consumed_successes':sorted(consumed.values(),key=lambda x:x['key']),'decisions':rows,'jobs':jobs,'bins':balanced(jobs),'dispatcher_sha256':core.file_hash(HERE/'adjudication_dispatch.py'),'controller_sha256':core.file_hash(__file__),'cloud':env,'note':'Explicit20M comparisons/discovery. No automatic scientific confirmation. New discovered alternative requires a separately reviewed singleton plan.'}
    out=Path(a.output);out.mkdir(exist_ok=False);save_new(out/'plan.json',plan)
    (out/'dispatcher.py').write_bytes((HERE/'adjudication_dispatch.py').read_bytes())
    request=batch_request(plan,out/'plan.json',out/'dispatcher.py');save_new(out/'batch-request.json',request)
    save_new(out/'preflight.json',{'decisions':len(rows),'jobs':len(jobs),'queries':sum(2 if j['move'] is None else 1 for j in jobs),'bin_sizes':[len(b) for b in plan['bins']],'plan_sha256':core.file_hash(out/'plan.json'),'batch_request_sha256':core.file_hash(out/'batch-request.json'),'status':'prepared_no_engines'})
    return {'output':str(out),'decisions':len(rows),'jobs':len(jobs),'queries':sum(2 if j['move'] is None else 1 for j in jobs)}

def batch_request(plan,plan_path,dispatcher_path):
    code=Path(dispatcher_path).read_text()
    if core.digest(code.encode())!=plan['dispatcher_sha256']:raise ValueError('Dispatcher hash mismatch')
    config=plan['cloud']['config'];key=plan['plan_prefix']+'/plan.json'
    request=gcp.batch_config(plan['store_uri']+'/'+key,plan['cloud']['image'],config,len(plan['bins']))
    spec=request['taskGroups'][0]['taskSpec'];container=spec['runnables'][0]['container']
    container['entrypoint']='python3';container['commands']=['-c',code,'--store',plan['store_uri'],'--plan-key',key,'--plan-sha256',core.file_hash(plan_path)]
    return request

def submit(a):
    directory=Path(a.directory);plan=read(directory/'plan.json');validate_plan(plan);request=read(directory/'batch-request.json')
    if request!=batch_request(plan,directory/'plan.json',directory/'dispatcher.py'):raise ValueError('Saved request mismatch')
    config=plan['cloud']['config'];store=gcp.GCSStore(plan['store_uri'])
    if core.digest(store.get(plan['original_manifest']['key']))!=plan['original_manifest']['sha256']:raise ValueError('Original manifest changed')
    for ref in plan['consumed_successes']:
        if core.digest(store.get(ref['key']))!=ref['sha256']:raise ValueError('Consumed success changed')
    put_same(store,plan['plan_prefix']+'/plan.json',(directory/'plan.json').read_bytes())
    put_same(store,plan['plan_prefix']+'/dispatcher.py',(directory/'dispatcher.py').read_bytes())
    put_same(store,plan['plan_prefix']+'/batch-request.json',(directory/'batch-request.json').read_bytes())
    # Exclusive durable marker prevents ambiguous timeout/double submission retries.
    marker=directory/'submission.json';save_new(marker,{'status':'submission_started','job_name':a.job_name,'at':time.time(),'plan_sha256':core.file_hash(directory/'plan.json')})
    remote_key=plan['plan_prefix']+'/submission-started.json'
    if not store.put(remote_key,marker.read_bytes()):raise ValueError('Remote submission already attempted; reconcile manually')
    command=['gcloud','batch','jobs','submit',a.job_name,'--project',config['project'],'--location',config['region'],'--config',str(directory/'batch-request.json'),'--format=json']
    try:
        result=subprocess.run(command,capture_output=True,text=True,timeout=120)
        save_new(directory/'submission-result.json',{'returncode':result.returncode,'stdout':result.stdout,'stderr':result.stderr,'command':command})
        if result.returncode:raise RuntimeError('Submission failed; preserve records, no blind retry')
        response=core.decode(result.stdout.encode());put_same(store,plan['plan_prefix']+'/submission-result.json',core.encode(response));return response
    except BaseException as exc:
        save_new(directory/'submission-error.json',{'error':repr(exc),'note':'May be ambiguous; inspect exact Batch job identity before any retry'});raise

def validate_plan(plan):
    if plan['schema']!='msh161-adjudication-v1':raise ValueError('Wrong plan schema')
    ids=[j['id'] for j in plan['jobs']]
    if len(ids)!=len(set(ids)) or len(plan['bins'])!=8 or sorted(ids)!=sorted(k for b in plan['bins'] for k in b):raise ValueError('Invalid bins/job coverage')
    for j in plan['jobs']:
        core.validate_job(j)
        if j['kind']!='confirmation' or j['nodes']!=20000000 or j['experiment']!=plan['experiment'] or j['environment']['cloud']!=plan['cloud']:raise ValueError('Wrong job kind/budget/cohort')
        if j['adjudication']['plan_id']!=plan['plan_id']:raise ValueError('Wrong plan provenance')

def verify(a):
    plan=read(a.plan);validate_plan(plan)
    store=core.LocalStore(a.cache) if a.cache else gcp.GCSStore(plan['store_uri']);totals={};results=[];attempts=[]
    if store.get(plan['plan_prefix']+'/plan.json')!=Path(a.plan).read_bytes():raise ValueError('Published plan mismatch')
    for ref in [plan['original_manifest']]+plan['consumed_successes']:
        if core.digest(store.get(ref['key']))!=ref['sha256']:raise ValueError('Plan dependency changed')
    for j in plan['jobs']:
        r=core.verify_result(store,j);results.append({'job':j['id'],'success_sha256':core.digest(store.get(core.prefix(j)+'/success.json'))})
        for k in ['wall_seconds','child_cpu_seconds']:totals[k]=totals.get(k,0)+r[k]
        artifacts=r['artifacts'];qs=[core.decode(store.get(v['key'])) for k,v in artifacts.items() if k.startswith('analysis/g') and '-attempt' in k]
        expected=[j['move']] if j.get('move') else [None,j['adjudication']['decision']['played']]
        if sorted(q['forced_move'] or '' for q in qs)!=sorted(m or '' for m in expected):raise ValueError('Incomplete query coverage')
        for q in qs:
            if q['decision']!=j['adjudication']['decision'] or q['nodes_requested']!=20000000 or q['engine_sha256']!=j['build']['engines']['stockfish19']['sha256']:raise ValueError('Query context/budget mismatch')
            if q['status']!='complete' or q['exit_code']!=0 or not q['process_group_settled'] or q['raw_truncated']:raise ValueError('Query execution not clean')
            if q['assessment']['score'] is not None and not q.get('pv_independent_legal'):raise ValueError('PV legality not established')
            if q['assessment']['score'] is None and 'no_nonbound_score_aligned_with_bestmove' not in q['assessment']['uncertainty']:raise ValueError('Missing score uncertainty')
            if q['forced_move'] and q['assessment']['bestmove']!=q['forced_move']:raise ValueError('Forced root differs')
        if store.exists(core.prefix(j)+'/claim.json'):raise ValueError('Unresolved plan claim')
        for key in store.list(core.prefix(j)+'/attempts/'):
            if key.endswith('/receipt.json') and '/artifacts/' not in key:
                receipt=core.decode(store.get(key));attempts.append({'key':key,'status':receipt['status'],'wall_seconds':receipt.get('wall_seconds',0),'child_cpu_seconds':receipt.get('child_cpu_seconds',0)})
    result={'plan_sha256':core.file_hash(a.plan),'jobs':len(results),'decisions':len(plan['decisions']),'results':results,'successful_job_totals':totals,'attempts':attempts,'all_attempt_wall_seconds':sum(r['wall_seconds'] for r in attempts),'failed_attempts':sum(r['status']!='complete' for r in attempts),'status':'complete_needs_scientific_review','confirmed_events':False}
    save_new(a.output,result);return {'jobs':len(results),'status':result['status']}

def fetch_plan(plan,cache):
    remote=gcp.GCSStore(plan['store_uri']);local=core.LocalStore(cache)
    for key in [plan['plan_prefix']+'/plan.json',plan['original_manifest']['key']]+[r['key'] for r in plan['consumed_successes']]:
        put_same(local,key,remote.get(key))
    def fetch(job):
        for key in remote.list(core.prefix(job)+'/'):
            put_same(local,key,remote.get(key))
        core.verify_result(local,job)
    with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:
        list(pool.map(fetch,plan['jobs']))

def wait_verify(a):
    plan=read(a.plan);validate_plan(plan)
    submission=read(Path(a.plan).parent/'submission-result.json')
    if submission['returncode']!=0:raise ValueError('No successful submission receipt')
    submitted=core.decode(submission['stdout'].encode())
    if not gcp._matches_job_resource(submitted['name'],plan['cloud']['config']['project'],plan['cloud']['config']['region'],a.job_name) or not submitted.get('uid'):raise ValueError('Wrong submitted Batch job')
    expected=batch_request(plan,a.plan,Path(a.plan).parent/'dispatcher.py')
    if read(Path(a.plan).parent/'batch-request.json')!=expected:raise ValueError('Submitted request differs')
    if submitted['taskGroups'][0]['taskSpec']['runnables']!=expected['taskGroups'][0]['taskSpec']['runnables']:raise ValueError('Submitted runnable differs')
    out=Path(a.output_directory);out.mkdir(exist_ok=False)
    save_new(out/'started.json',{'job_name':a.job_name,'plan_sha256':core.file_hash(a.plan),'at':time.time()})
    try:
        result=gcp.wait_for_completion(a.job_name,plan['cloud']['config'],a.subscription)
        save_new(out/'cloud-completion.json',result)
        if result['job']['uid']!=submitted['uid'] or not gcp._matches_job_resource(result['job']['name'],plan['cloud']['config']['project'],plan['cloud']['config']['region'],a.job_name):raise ValueError('Completion identity differs from submission')
        fetch_plan(plan,a.cache)
        value=verify(argparse.Namespace(plan=a.plan,cache=a.cache,output=str(out/'verification.json')))
        completion={'status':'complete_needs_scientific_review','verification':value,'plan_sha256':core.file_hash(a.plan),'verification_sha256':core.file_hash(out/'verification.json')}
        remote=gcp.GCSStore(plan['store_uri'])
        put_same(remote,plan['plan_prefix']+'/verification.json',(out/'verification.json').read_bytes())
        put_same(remote,plan['plan_prefix']+'/completion.json',core.encode(completion))
        save_new(out/'completion.json',completion);return value
    except BaseException as exc:
        if hasattr(exc,'result'):save_new(out/'cloud-completion.json',exc.result)
        save_new(out/'failure.json',{'error':repr(exc),'at':time.time(),'note':'No automatic retry or scientific confirmation'})
        raise

def main():
    p=argparse.ArgumentParser();s=p.add_subparsers(dest='command',required=True)
    q=s.add_parser('prepare');q.add_argument('--manifest',required=True);q.add_argument('--index',required=True);q.add_argument('--cache',required=True);q.add_argument('--selection',action='append',required=True);q.add_argument('--plan-id',required=True);q.add_argument('--output',required=True)
    q=s.add_parser('submit');q.add_argument('--directory',required=True);q.add_argument('--job-name',required=True);q.add_argument('--spending-approved',action='store_true',required=True)
    q=s.add_parser('wait-verify');q.add_argument('--plan',required=True);q.add_argument('--job-name',required=True);q.add_argument('--subscription',required=True);q.add_argument('--cache',required=True);q.add_argument('--output-directory',required=True)
    q=s.add_parser('verify');q.add_argument('--plan',required=True);q.add_argument('--cache');q.add_argument('--output',required=True)
    a=p.parse_args();print(json.dumps(globals()[a.command.replace('-','_')](a),indent=2))
if __name__=='__main__':main()
