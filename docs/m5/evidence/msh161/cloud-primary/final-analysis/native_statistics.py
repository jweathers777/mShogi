"""Offline native-outcome statistics. Never starts an engine or cloud job.
Run only after final cumulative comparison closure; --self-test uses fixtures only.
"""
import argparse,collections,gzip,hashlib,json,math,platform,random,sys
from pathlib import Path
HERE=Path(__file__).resolve().parent
H=HERE.parent
STUDY=H.parent
sys.path.insert(0,str(STUDY))
import analysis

# Assertions below are invariant checks; never permit Python to disable them.
if not __debug__:
    raise RuntimeError("Optimized Python is unsupported: required invariant checks would be disabled")

SEED=16120261113
REPLICATES=50000
SCORES={'wins':1.0,'draws':0.5,'losses':0.0}

def sha(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def read(path):return json.loads(Path(path).read_text())
def quantile(values,q):
    ordered=sorted(values);index=(len(ordered)-1)*q;i=math.floor(index);h=index-i
    return ordered[i] if i==len(ordered)-1 else ordered[i]*(1-h)+ordered[i+1]*h

def draw_pair_score(groups,families,rng):
    total=0.0
    for family in families:
        values=groups[family]
        for _ in range(len(values)):total+=values[rng.randrange(len(values))]
    return total/sum(map(len,groups.values()))

def bootstrap(pairs,replicates=REPLICATES):
    groups=collections.defaultdict(list)
    for pair in sorted(pairs,key=lambda x:(x['family'],x['pair'])):groups[pair['family']].append(pair['score'])
    families=sorted(groups);rng=random.Random(SEED);n=sum(map(len,groups.values()));samples=[]
    for _ in range(replicates):
        samples.append(draw_pair_score(groups,families,rng))
    return {'replicates':replicates,'seed':SEED,'point_score':sum(x['score'] for x in pairs)/n,
            'conditional_percentile95':[quantile(samples,.025),quantile(samples,.975)],
            'family_pair_counts':{f:len(groups[f]) for f in families},
            'draw_order':'replicate;lexicographic family;pair-count randrange draws over lexicographic pair IDs',
            'quantile':'linear index (B-1)*q; i=floor(index); (1-h)*sorted[i]+h*sorted[i+1]',
            'limitations':'Whole opening pairs resampled within fixed observed families. Conditional sample uncertainty, not random-Chess population or rating inference.'}

def closure_guard(v):
    if not (v.get('games_completed')==200 and len(v.get('native_games',[]))==200
            and v.get('batches_reviewed')==[f'{i:02d}' for i in range(1,11)]
            and v.get('remaining_missing_singletons')==0):
        raise ValueError('Final comparison closure is incomplete')

def bind_report(report,spec):
    if report['config']['id']!=spec['id'] or report['config']['openings']!=[spec['opening']]:
        raise ValueError('Native report does not match frozen pair/opening')
    if report['config']['claim_policy']!='auto_claim' or len(report['games'])!=2:
        raise ValueError('Wrong native controller/pair coverage')
    for gi,game in enumerate(report['games']):
        if (game['opening_index']!=0 or game['opening_id']!=spec['opening']['id']
                or game['pairing']!=gi or game['engines_by_color']!=([0,1] if gi==0 else [1,0])):
            raise ValueError('Native game identity/color mismatch')
    return analysis.extract_decisions(report)

def native_outcome(game):
    color='white' if game['engines_by_color'][0]==0 else 'black'
    conclusion=game['conclusion']['chess']
    return color, ('draws' if 'draw' in conclusion else ('wins' if conclusion['checkmate']['winner']==color else 'losses'))

def expect_rejection(fn):
    try:fn()
    except (ValueError,AssertionError):return
    raise AssertionError('Negative fixture was accepted')

def self_test():
    assert quantile([0,10],.25)==2.5 and quantile([10,0],.75)==7.5
    assert bootstrap([{'family':'f','pair':'p','score':.5}],100)['conditional_percentile95']==[.5,.5]
    unequal=[{'family':'a','pair':str(i),'score':1.0} for i in range(3)]+[{'family':'b','pair':'x','score':0.0}]
    assert bootstrap(unequal,100)['conditional_percentile95']==[.75,.75]
    varying=[{'family':'b','pair':'2','score':0.0},{'family':'a','pair':'1','score':.5},{'family':'b','pair':'1','score':1.0}]
    assert bootstrap(varying,100)==bootstrap(list(reversed(varying)),100)
    assert sum(SCORES[x] for x in ['wins','draws','losses'])/3==.5
    # Two equally weighted paired clusters: bootstrap support is 0, .5, 1;
    # 1000 deterministic replicates contain both extremes. Game-level resampling
    # would introduce quarter scores and lose this exact three-point support.
    class FixedDraws:
        def __init__(self,values):self.values=iter(values)
        def randrange(self,n):
            value=next(self.values)
            assert 0<=value<n
            return value
    # Enumeration of the four whole-pair draws has probabilities 1/4,1/2,1/4.
    assert [draw_pair_score({'a':[0.0,1.0]},['a'],FixedDraws(draw)) for draw in [(0,0),(0,1),(1,0),(1,1)]]==[0.0,.5,.5,1.0]
    clustered=[{'family':'a','pair':'1','score':0.0},{'family':'a','pair':'2','score':1.0}]
    got=bootstrap(clustered,1000)
    assert got['point_score']==.5 and got['conditional_percentile95']==[0.0,1.0]
    expect_rejection(lambda:closure_guard({'games_completed':180}))
    opening={'id':'fixture','fen':None,'moves':['e2e4']}
    initial={'rules_profile':'mshogi.chess.match.v1','initial_fen':analysis.START_FEN,'history_origin':'standard_start','events':[{'ply':0,'event':{'kind':'move','action':'e2e4'}}]}
    event=lambda c:{'request':{'actor':'black','command':c},'reply':{'rejection':None}}
    record={'rules_profile':'mshogi.chess.match.v1','initial_record_json':initial,'events':[event({'move':'e7e5'}),event({'claim':{'reason':'threefold_repetition','intended':'g1f3'}})]}
    game=lambda gi:{'opening_index':0,'opening_id':'fixture','pairing':gi,'engines_by_color':[0,1] if gi==0 else [1,0],'stop':'chess_conclusion','conclusion':{'chess':{'draw':'threefold_repetition'}},'failure':None,'match_record_json':record}
    report={'schema_version':1,'config':{'id':'p','openings':[opening],'claim_policy':'auto_claim'},'games':[game(0),game(1)]}
    ds,gs=bind_report(report,{'id':'p','opening':opening})
    assert [(d['origin'],d['played']) for d in ds if d['game']==0]==[('prepared','e2e4'),('played','e7e5')]
    assert gs[0]['complete_history']==['e2e4','e7e5']
    assert native_outcome(game(0))==('white','draws')
    checkmate=game(1);checkmate['conclusion']={'chess':{'checkmate':{'winner':'black'}}}
    assert native_outcome(checkmate)==('black','wins')
    expect_rejection(lambda:bind_report(report,{'id':'wrong','opening':opening}))
    expect_rejection(lambda:bind_report(report,{'id':'p','opening':dict(opening,moves=[])}))
    return {'status':'passed','checks':['independent linear-quantile fixture','degenerate paired score','fixed unequal family weights','input-order invariance','native WDL score mapping','nondegenerate pair bootstrap extremes','incomplete closure rejected','native source/opening negative fixtures','supplied/played and intended-unplayed claim fixture','paired color/result fixture']}

def run(output):
    v=read(H/'primary-cumulative.json');closure_guard(v)
    procedure=read(HERE/'procedure-review-v2.json');assert procedure['status']=='passed_independent_procedure_review_v2' and sha(HERE/'procedure.md')==procedure['procedure_sha256']
    source_selection=read(STUDY/'sources/main-selection.json');families={x['id']:x['family'] for x in source_selection['selected_roots']};assert len(families)==90
    specs={};source_bindings={'primary-cumulative.json':sha(H/'primary-cumulative.json'),'../sources/main-selection.json':sha(STUDY/'sources/main-selection.json'),'../analysis.py':sha(STUDY/'analysis.py'),'procedure.md':sha(HERE/'procedure.md'),'procedure-review-v2.json':sha(HERE/'procedure-review-v2.json')}
    for batch in range(1,11):
        path=H/f'batch-{batch:02d}-manifest.json';manifest=read(path);source_bindings[path.name]=sha(path)
        for job in manifest['jobs']:
            spec=job['spec'];pair=spec['id']
            if pair in specs:assert specs[pair]==spec
            specs[pair]=spec
    assert len(specs)==100
    games=[];seen=set();reports={};report_identities={};all_played=0;all_prepared=0
    prepared_openings=[s['opening']['id'] for s in specs.values() if s['phase']=='main-prepared']
    assert len(prepared_openings)==90 and len(set(prepared_openings))==90 and set(prepared_openings)==set(families)
    for item in sorted(v['native_games'],key=lambda x:(x['pair'],x['game'])):
        pair=item['pair'];gi=item['game'];key=(pair,gi);assert key not in seen;seen.add(key)
        art=item['native_report'];path=H/'local-cache'/art['key'];assert sha(path)==art['sha256']
        identity=(art['key'],art['sha256'])
        if pair in report_identities and report_identities[pair]!=identity:raise ValueError('Pair entries reference different native artifacts')
        if pair not in report_identities and identity in report_identities.values():raise ValueError('Native artifact reused across pair IDs')
        report_identities[pair]=identity
        if pair not in reports:
            report=analysis.load_report(path);assert report['config']['claim_policy']=='auto_claim'
            decisions,native_games=bind_report(report,specs[pair]);reports[pair]=(report,decisions,native_games)
        report,decisions,native_games=reports[pair];native=report['games'][gi];ng=native_games[gi]
        assert native['stop']=='chess_conclusion' and not native['failure']
        assert ng['conclusion']==item['native_conclusion'] and ng['pairing']==gi
        candidate_color,result=native_outcome(native)
        assert result==item['candidate_result']
        moves=[d for d in decisions if d['game']==gi and d['origin']=='played'];prefix=[d for d in decisions if d['game']==gi and d['origin']=='prepared']
        assert len(moves)==item['played_decisions'] and len(prefix)==item['supplied_context_decisions']
        all_played+=len(moves);all_prepared+=len(prefix)
        spec=specs[pair];prepared=spec['phase']=='main-prepared';assert prepared==(not pair.endswith('unaided'));family=families[spec['opening']['id']] if prepared else 'unaided'
        record=analysis.decode(native['match_record_json']);claims=[e for e in record['events'] if isinstance(e['request']['command'],dict) and 'claim' in e['request']['command']]
        assert len(claims)<=1 and all(e['reply'].get('rejection') is None for e in claims)
        games.append({'pair':pair,'game':gi,'family':family,'prepared':prepared,'opening_id':spec['opening']['id'],'candidate_color':candidate_color,'result':result,'score':SCORES[result],
                      'played_decisions':len(moves),'supplied_context_decisions':len(prefix),'native_conclusion':ng['conclusion'],
                      'claim_reason':claims[0]['request']['command']['claim']['reason'] if claims else None,
                      'claim_intended_move_unplayed':claims[0]['request']['command']['claim'].get('intended') if claims else None,
                      'complete_history_sha256':hashlib.sha256(json.dumps(ng['complete_history'],separators=(',',':')).encode()).hexdigest(),
                      'native_report_key':art['key'],'native_report_sha256':art['sha256']})
    assert all_played==v['played_decisions'] and all_prepared==v['supplied_context_decisions']
    grouped=collections.defaultdict(list)
    for game in games:grouped[game['pair']].append(game)
    pairs=[]
    for pair,items in sorted(grouped.items()):
        assert len(items)==2 and {x['candidate_color'] for x in items}=={'white','black'}
        assert len({(x['family'],x['prepared'],x['opening_id']) for x in items})==1
        pairs.append({'pair':pair,'family':items[0]['family'],'prepared':items[0]['prepared'],'score':sum(x['score'] for x in items)/2})
    prepared_pairs=[x for x in pairs if x['prepared']];assert len(prepared_pairs)==90
    def outcome(rows):return {'games':len(rows),'outcomes':{k:sum(x['result']==k for x in rows) for k in SCORES},'score_fraction':sum(x['score'] for x in rows)/len(rows)}
    strata={'all':outcome(games),'prepared':outcome([x for x in games if x['prepared']]),'unaided':outcome([x for x in games if not x['prepared']])}
    assert strata['all']['outcomes']==v['outcomes'] and strata['prepared']['outcomes']==v['prepared_outcomes'] and strata['unaided']['outcomes']==v['unaided_outcomes']
    family_results={f:outcome([x for x in games if x['family']==f]) for f in sorted({x['family'] for x in games if x['prepared']})}
    family_sensitivity={f:sum(x['score'] for x in prepared_pairs if x['family']!=f)/sum(x['family']!=f for x in prepared_pairs) for f in family_results}
    data={'status':'native_statistics_complete_needs_independent_result_review','procedure_sha256':sha(HERE/'procedure.md'),'implementation_sha256':sha(__file__),'python':sys.version,'python_chess':__import__('chess').__version__,
          'source_bindings':source_bindings,'self_tests':self_test(),'all_played_decisions_reconciled':all_played,'supplied_context_decisions_reconciled':all_prepared,
          'strata':strata,'prepared_pair_bootstrap':bootstrap(prepared_pairs),'leave_one_family_out_score':family_sensitivity,'family_results':family_results,
          'color_results':{c:outcome([x for x in games if x['candidate_color']==c]) for c in ['white','black']},
          'unaided_unique_complete_histories':{c:len({x['complete_history_sha256'] for x in games if not x['prepared'] and x['candidate_color']==c}) for c in ['white','black']},
          'claims':dict(collections.Counter(x['claim_reason'] for x in games if x['claim_reason'])),'games':games,'pairs':pairs,
          'limits':['Native outcomes only; no engine/reference score substitutes for a result.','Unaided repeats stay descriptive and outside the prepared interval.','This table does not classify weaknesses or complete all evidence-tier/behavioral coding obligations.','Not a human-rating/Elo estimate or proof of reference superiority.']}
    output=Path(output)
    with output.open('x') as f:json.dump(data,f,indent=2);f.write('\n')
    print(json.dumps({'status':data['status'],'strata':strata,'prepared_pair_bootstrap':data['prepared_pair_bootstrap']}))

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--self-test',action='store_true');p.add_argument('--output');a=p.parse_args()
    if a.self_test:print(json.dumps(self_test()))
    elif a.output:run(a.output)
    else:p.error('--self-test or --output is required')
