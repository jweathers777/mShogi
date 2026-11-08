#!/usr/bin/env python3
"""One fixed-sample MSH-49 analysis. Exact paired-sign tails, no chess rules."""
import argparse
import gzip
import hashlib
import json
import math
from pathlib import Path
import subprocess
import tempfile
import sys
sys.dont_write_bytecode = True
from run_matches import eligible

HERE=Path(__file__).resolve().parent

def sha(raw):return hashlib.sha256(raw).hexdigest()
def read(p):return json.loads(Path(p).read_text())

def sign_test(differences):
    positive=sum(d>0 for d in differences);negative=sum(d<0 for d in differences)
    n=positive+negative
    numerator=sum(math.comb(n,k) for k in range(positive,n+1)) if n else 1
    denominator=2**n if n else 1
    return {'positive':positive,'negative':negative,'ties':len(differences)-n,'non_ties':n,
            'tail_numerator':numerator,'tail_denominator':denominator,
            'p':numerator/denominator,'passes_0_05':20*numerator<=denominator}

def analyse(args):
    ledger=read(args.ledger);freeze=read(args.freeze);protocol=read(HERE/'protocol-v1.json')
    assert ledger['protocol_sha256']==freeze['protocol_sha256']==sha((HERE/'protocol-v1.json').read_bytes())
    formal=[r for r in ledger['runs'] if r['phase']=='formal']
    reasons=[]
    if ledger['inflight']:reasons.append('unsettled invocation')
    if ledger['spent_seconds']>protocol['budget']['local_match_wall_seconds']:reasons.append('match budget exceeded')
    if len(formal)!=2 or {r['arm'] for r in formal}!={'relative','external'}:reasons.append('two fixed formal arms not complete')
    if any(not r['complete'] or any(not p['eligible'] for p in r['pairs']) for r in formal):reasons.append('incomplete/ineligible arm')
    if reasons:
        return {'eligible':False,'passed':False,'reasons':reasons,'spent_seconds':ledger['spent_seconds'],'no_acceptance_inference':'Incomplete data never passes; no game/pair replaced or omitted'}
    results={}
    for run in formal:
        assert run['executables_sha256']==freeze['executables_sha256']
        assert run['catalog_sha256']==freeze['catalog_sha256'][run['arm']]
        catalog=read(freeze['catalog_paths'][run['arm']])
        assert sha(Path(freeze['catalog_paths'][run['arm']]).read_bytes())==run['catalog_sha256']
        count=64 if run['arm']=='relative' else 32
        expected=[(i,p) for i in range(count) for p in (['candidate'] if run['arm']=='relative' else ['baseline','candidate'])]
        assert [(p['sample'],p['participant']) for p in run['pairs']]==expected
        values={}
        for receipt in run['pairs']:
            folder=Path(args.reports_root)/run['arm'] if args.reports_root else Path(run['output'])
            raw=gzip.decompress((folder/receipt['report']).read_bytes())
            assert sha(raw)==receipt['report_sha256']
            outer=json.loads(raw);assert sha(outer['report_json'].encode())==outer['report_sha256']
            report=json.loads(outer['report_json'])
            assert report['config']['openings']==[catalog['samples'][receipt['sample']]['opening']]
            assert report['config']['engine_work'][0]==protocol['candidate_and_baseline_limits']
            expected_second=protocol['stockfish_limits'] if run['arm']=='external' else protocol['candidate_and_baseline_limits']
            assert report['config']['engine_work'][1]==expected_second
            # Independent native replay is read-only; no engine is launched here.
            with tempfile.TemporaryDirectory(prefix='msh49-replay-') as temporary:
                path=Path(temporary)/'report.json';path.write_bytes(raw)
                result=subprocess.run([args.runner,'experiment-external-replay',str(path)],capture_output=True)
                assert result.returncode==0,result.stderr.decode(errors='replace')
            checked_hp,reasons=eligible(report,report['config'])
            assert not reasons and checked_hp==receipt['halfpoints'],reasons
            hp=2*report['summaries'][0]['wins'][0]+report['summaries'][0]['draws']
            assert hp==receipt['halfpoints']
            values[(receipt['sample'],receipt['participant'])]=hp
        if run['arm']=='relative':
            scores=[values[(i,'candidate')] for i in range(count)];total=sum(scores)
            test=sign_test([n-2 for n in scores]);score_gate=total>=180
            results['relative']={'pairs':count,'candidate_halfpoints':total,'score_fraction':total/256,'score_gate':score_gate,'sign_test':test,'passed':score_gate and test['passes_0_05'],'pentanomial':[scores.count(k) for k in range(5)]}
        else:
            candidate=[values[(i,'candidate')] for i in range(count)];baseline=[values[(i,'baseline')] for i in range(count)]
            difference=[a-b for a,b in zip(candidate,baseline)];test=sign_test(difference)
            score_gate=sum(candidate)>=32;mean_gate=sum(difference)>0
            results['external']={'roots':count,'candidate_halfpoints':sum(candidate),'baseline_halfpoints':sum(baseline),'candidate_score_fraction':sum(candidate)/128,'baseline_score_fraction':sum(baseline)/128,'difference_halfpoints':sum(difference),'score_gate':score_gate,'positive_mean_gate':mean_gate,'sign_test':test,'passed':score_gate and mean_gate and test['passes_0_05'],'root_differences':difference}
    return {'eligible':True,'passed':all(r['passed'] for r in results.values()),'spent_seconds':ledger['spent_seconds'],'results':results,'scope':'Approved finite-resource opening-distribution study; exact tests concern nonzero paired signs, not mean CIs or human Elo'}

def self_test():
    assert sign_test([])['p']==1 and not sign_test([])['passes_0_05']
    assert sign_test([1]*4)['p']==1/16 and not sign_test([1]*4)['passes_0_05']
    assert sign_test([1]*5)['p']==1/32 and sign_test([1]*5)['passes_0_05']
    assert sign_test([1,1,-1,-1])['p']==11/16
    assert sign_test([0,1,0,-1])['ties']==2

if __name__=='__main__':
    self_test()
    p=argparse.ArgumentParser();p.add_argument('--self-test',action='store_true')
    p.add_argument('--ledger');p.add_argument('--freeze');p.add_argument('--runner');p.add_argument('--reports-root')
    a=p.parse_args()
    if a.self_test:print('exact sign-tail independent examples passed')
    else:
        if not(a.ledger and a.freeze and a.runner):p.error('ledger,freeze,runner required')
        print(json.dumps(analyse(a),indent=2))
