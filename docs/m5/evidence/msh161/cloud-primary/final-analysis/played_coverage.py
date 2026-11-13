"""Offline native-root coverage, not weakness classification or query execution."""
import argparse,collections,gzip,hashlib,json,sys
from pathlib import Path
import native_statistics as native
H=Path(__file__).resolve().parent.parent
if not __debug__:raise RuntimeError('Optimized Python is unsupported')

def digest(raw):return hashlib.sha256(raw).hexdigest()
def row_hash(row):return digest(json.dumps(row,sort_keys=True,separators=(',',':')).encode())
def key(row):return row['pair'],row['game'],row['ply']
def unique(rows):
    out={}
    for row in rows:
        k=key(row)
        if k in out:raise ValueError(f'Duplicate root: {k}')
        out[k]=row
    return out

def flag_fields(value,path=''):
    """Keep original flag locations and values; do not merge away provenance."""
    out={}
    if isinstance(value,dict):
        for k,v in value.items():
            here=f'{path}/{k}'
            if any(s in k.lower() for s in ('flag','conflict','uncertain')):out[here]=v
            elif isinstance(v,(dict,list)):out.update(flag_fields(v,here))
    elif isinstance(value,list):
        for i,v in enumerate(value):out.update(flag_fields(v,f'{path}/{i}'))
    return out

class Inputs:
    def __init__(self):self.bindings={};self.cache={}
    def read(self,relative,expected=None):
        path=H/relative
        if not path.exists():path=Path(str(path)+'.gz')
        if path not in self.cache:
            raw=path.read_bytes();expanded=gzip.decompress(raw) if path.suffix=='.gz' else raw
            self.bindings[str(path.relative_to(H))]={'sha256':digest(raw),'expanded_sha256':digest(expanded)}
            self.cache[path]=json.loads(expanded)
        if expected is not None and self.bindings[str(path.relative_to(H))]['sha256']!=expected:raise ValueError(f'Hash mismatch: {relative}')
        return self.cache[path]
    def pointer(self,relative,row):return {'source':relative,'row_key':dict(zip(('pair','game','ply'),key(row))),'canonical_row_sha256':row_hash(row)}

def collect(final=False):
    inputs=Inputs();v=inputs.read('primary-cumulative.json')
    if final:native.closure_guard(v)
    procedure_review=inputs.read('final-analysis/procedure-review-v2.json')
    if digest((H/'final-analysis/procedure.md').read_bytes())!=procedure_review['procedure_sha256']:raise ValueError('Procedure changed')
    for source in ('autoclaim-restoration.json','batch-09-review/method-amendment-v1.1.json','batch-09-review/audit-expansion-admission.json','batch-09-review/audit-expansion-notation-erratum.json'):
        inputs.read(source)
    selected=unique(v['normalized_selected_events']);overlays={}
    for source in sorted({x['source'] for x in selected.values()}):
        overlays[source]=unique(inputs.read(source)['decisions'])
    output=[];contexts=[];all_native=set();base_keys=set();deep_keys=set();tier_counts=collections.Counter()
    specs={}
    for batch in range(1,11):
        for j in inputs.read(f'batch-{batch:02}-manifest.json')['jobs']:
            spec=j['spec']
            if spec['id'] in specs and specs[spec['id']]!=spec:raise ValueError('Manifest spec conflict')
            specs[spec['id']]=spec
        folder=f'batch-{batch:02}-review'
        names=['dispositions-00-04.json','evidence-05-09.json'] if batch<=2 else ['gameplay-00-04.json','gameplay-05-09.json']
        base={}
        for name in names:
            source=f'{folder}/{name}'
            for k,row in unique(inputs.read(source)['decisions']).items():
                if k in base:raise ValueError('Duplicate base panel root')
                base[k]=(row,source)
        plan=inputs.read(f'batch-{batch:02}-adjudication/plan.json');planned=unique(plan['decisions']);deep_keys.update(planned)
        idx=inputs.read(f'{folder}/review-index.json')
        for pair,entry in sorted(idx.items()):
            art=entry['game_pair']['artifacts']['pair/report.json.gz'];raw=(H/'local-cache'/art['key']).read_bytes()
            if digest(raw)!=art['sha256']:raise ValueError('Native artifact mismatch')
            report=native.analysis.load_report(H/'local-cache'/art['key']);ds,gs=native.bind_report(report,specs[pair])
            screenart=entry['reference_screening']['artifacts']['screen-v1.json'];screen=inputs.read('local-cache/'+screenart['key'],screenart['sha256'])
            screens=unique([dict(r,pair=pair,game=r['decision']['game'],ply=r['decision']['ply']) for r in screen['decisions']])
            canonical={(pair,d['game'],d['ply']):d for d in ds}
            if set(screens)!=set(canonical):raise ValueError('Screen/native root coverage mismatch')
            five={(pair,j['job']['game'],j['job']['ply']) for j in entry['confirmations']}
            for k,d in canonical.items():
                if k in all_native:raise ValueError('Duplicate native root')
                all_native.add(k);s=screens[k]
                if s['decision']!=d:raise ValueError('Screen/native identity mismatch')
                identity={'pair':pair,'game':d['game'],'ply':d['ply'],'played':d['played'],'engine_index':d['engine_index'],'origin':d['origin'],'mover':d['mover'],'native_event':d['native_event'],'history_sha256':row_hash({'initial_fen':d['initial_fen'],'history_origin':d['history_origin'],'history':d['history']}),'native_report':{'key':art['key'],'sha256':art['sha256']}}
                if d['origin']=='prepared':contexts.append(identity);continue
                if k not in base:raise ValueError(f'Missing base disposition: {k}')
                b,source=base[k];base_keys.add(k)
                if b['played']!=d['played']:raise ValueError('Base played identity mismatch')
                tier='selected_20M' if k in planned else ('matched_5M' if k in five else 'screening_500k')
                if pair=='main-09-00' and d['game']==0 and d['ply'] in (57,75):tier='admitted_mixed_expansion_20M_without_5M'
                item=dict(identity,evidence_tier=tier,base_disposition=b.get('status',b.get('disposition')),base_source=inputs.pointer(source,b),base_source_flags=flag_fields(b),screen_source={'source':'local-cache/'+screenart['key'],'sha256':screenart['sha256'],'row_key':{'game':d['game'],'ply':d['ply']}},screen_flags=flag_fields(s),selected20m=k in planned,confirmed_nonerror=False)
                if k in selected:
                    n=selected[k];exact=overlays[n['source']][k]
                    if n['played']!=d['played'] or exact['played']!=d['played'] or n['candidate']!=(d['engine_index']==0):raise ValueError('Final overlay native identity mismatch')
                    item.update(final_disposition=n['status'],normalized_selected_row=n,final_source=inputs.pointer(n['source'],exact),final_source_flags=flag_fields(exact))
                else:item['final_disposition']='screening_only_unresolved' if tier=='screening_500k' else b.get('status',b.get('disposition'))
                tier_counts[tier]+=1;output.append(item)
        if set(base)!=base_keys.intersection(set(base)):raise ValueError('Extra base-ledger roots')
    roots=unique(output)
    if not set(selected)<=set(roots):raise ValueError('Selected row outside native played roots')
    if final and set(selected)!=deep_keys:raise ValueError('Final selected/deep-plan coverage mismatch')
    if final and (len(output)!=v['played_decisions'] or len(contexts)!=v['supplied_context_decisions']):raise ValueError('Cumulative native counts mismatch')
    return {'status':'complete_needs_independent_coverage_review' if final else 'input_schema_validation_only','source_bindings':inputs.bindings,'implementation_sha256':digest(Path(__file__).read_bytes()),'native_statistics_sha256':digest(Path(native.__file__).read_bytes()),'analysis_sha256':digest(Path(native.analysis.__file__).read_bytes()),'counts':{'played_roots':len(output),'supplied_context_roots':len(contexts),'evidence_tiers':dict(tier_counts),'selected_normalized_rows_available':len(selected),'planned20m_roots':len(deep_keys)},'normalization':'Native pair/game/ply is unique. Base status or disposition is retained verbatim with exact row hash. Final selected status comes from the closed cumulative normalization and links its exact source row. Flags retain source JSON paths. Screening-only roots remain unresolved, never confirmed non-errors. Same-choice and subthreshold labels do not establish universal non-error. Prefixes are context only. Mixed tier marks only the two newly expanded Batch 09 roots; 19 retained 5M cell roots and 20M trigger remain in their original tiers.','limits':'Coverage reconciliation only, not behavior coding, threshold inference, rankings or new scientific measurements. Source flags and exact source rows remain authoritative; absent normalized tags do not clear earlier uncertainty. Existing successful query receipt audits are reused; no duplicate full artifact hash pass.','played_roots':output,'supplied_context_roots':contexts}

def self_test():
    rows=[{'pair':'p','game':0,'ply':1}];assert len(unique(rows))==1
    try:unique(rows*2)
    except ValueError:pass
    else:raise AssertionError('Duplicate accepted')
    assert flag_fields({'played':{'flags':['x']},'alternative':{'flags':[]}})=={'/played/flags':['x'],'/alternative/flags':[]}
    try:native.closure_guard({'games_completed':180})
    except ValueError:pass
    else:raise AssertionError('Incomplete closure accepted')
    return {'status':'passed','checks':['duplicate roots rejected','source flag paths preserved including empty values','incomplete closure rejected']}

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--self-test',action='store_true');p.add_argument('--validate-inputs',action='store_true');p.add_argument('--output');a=p.parse_args()
    if a.self_test:print(json.dumps(self_test()))
    elif a.validate_inputs:
        data=collect(False);print(json.dumps({'status':data['status'],'counts':data['counts']}))
    elif a.output:
        data=collect(True);raw=json.dumps(data,indent=2).encode();path=Path(a.output)
        with path.open('xb') as f:f.write(gzip.compress(raw,mtime=0) if path.suffix=='.gz' else raw)
        print(json.dumps({'status':data['status'],'counts':data['counts'],'output_sha256':digest(path.read_bytes())}))
    else:p.error('Choose --self-test, --validate-inputs or --output')
