import json,gzip,collections,pickle,sys
c=json.load(gzip.open('final-analysis/played-coverage.json.gz'))
ph=json.load(gzip.open('final-analysis/phase-summary.json.gz'))
cum=json.load(open('primary-cumulative.json'))
sel=json.load(open('../sources/main-selection.json'))
fam={r['id']:(r['family'],r.get('opening_name')) for r in sel['selected_roots']}
phase={(r['pair'],r['game'],r['ply']):(r['phase'],r['material'],r['candidate']) for r in ph['played_roots']}
cov={(r['pair'],r['game'],r['ply']):r for r in c['played_roots']}
srcs=sorted({(r['pair'],r['screen_source']['source']) for r in c['played_roots']})
def q(x):
    if x is None: return None
    fr=x.get('selected_report') or x.get('final_report') or {}
    return dict(E=x.get('expected_score_mover'),best=x.get('bestmove'),score=fr.get('score'),depth=fr.get('depth'),pv=(fr.get('pv') or [])[:12],flags=x.get('flags'))
rows=[];games={}
for pair,src in srcs:
    s=json.load(open(src))
    for d in s['decisions']:
        dd=d['decision']
        key=(pair,dd['game'],dd['ply'])
        cv=cov.get(key)
        n=(cv or {}).get('normalized_selected_row') or {}
        p=phase.get(key,(None,None,None))
        row=dict(pair=pair,game=dd['game'],ply=dd['ply'],mover=dd['mover'],eng=dd['engine_index'],origin=dd['origin'],
                 played=dd['played'],opening=dd.get('opening_id'),hist_origin=dd.get('history_origin'),
                 pq=q(d['queries'].get('played')),uq=q(d['queries'].get('unrestricted')),loss=d.get('raw_expected_score_loss'),
                 flags=d.get('flags'),tier=(cv or {}).get('evidence_tier'),final=(cv or {}).get('final_disposition'),base=(cv or {}).get('base_disposition'),
                 nstatus=n.get('status'),nsev=n.get('severity'),nmargin=n.get('margin'),nalt=n.get('alternative'),
                 phase=p[0],material=p[1],cand=p[2])
        rows.append(row)
        g=games.setdefault((pair,dd['game']),dict(hist=None,fen=dd['initial_fen'],opening=dd.get('opening_id')))
        h=dd['history']+[dd['played']]
        if g['hist'] is None or len(h)>len(g['hist']): g['hist']=h
for ng in cum['native_games']:
    g=games[(ng['pair'],ng['game'])]
    g.update(result=ng['candidate_result'],conclusion=ng['native_conclusion'],claim=ng['actual_optional_claim_draw'],batch=ng['batch'],report=ng['native_report']['path'])
    g['family']=fam.get(g['opening'],('unaided',None))
pickle.dump((rows,games),open(sys.argv[1],'wb'))
print(len(rows),len(games))
print(collections.Counter((r['origin'],r['cand'],r['eng']) for r in rows))
print(collections.Counter(g['family'][0] for g in games.values()))
