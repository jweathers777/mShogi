import pickle,collections,sys,statistics as st
ev=pickle.load(open(sys.argv[1],'rb'))
C=collections.Counter
def band(e): return 'ahead' if e>=.75 else 'behind' if e<=.25 else 'balanced'
ok=[e for e in ev if e['eb'] is not None and e['ep'] is not None and e['margin']>=.20]
print('major-margin events',len(ok),'games',len({e['key'] for e in ok}))
def tact(e):
    # material swing against mover within reference PV after the played move
    sw=e['sw']
    if not sw: return 'nopv'
    # sw[0] is after own move; look at min over plies 2..8 relative to after-own-move (even indices = after opp replies settle)
    worst=min(sw[1:7]) if len(sw)>1 else sw[0]
    final=sw[min(len(sw)-1,7)]
    if final<=-2: return 'material-loss<=8ply'
    if final<=-1: return 'pawn-loss<=8ply'
    if final>=1: return 'material-gain-yet-worse'
    return 'no-material-change'
for tr in (('ahead','balanced'),('balanced','behind'),('balanced','balanced')):
    s=[e for e in ok if (band(e['eb']),band(e['ep']))==tr]
    print('\n==',tr,'n',len(s),'games',len({e['key'] for e in s}))
    print(' phase',dict(C(e['phase'] for e in s)))
    print(' played type',C(e['ptype'] for e in s).most_common(9))
    print(' alt type',C(e['atype'] for e in s).most_common(9))
    print(' ref PV consequence',dict(C(tact(e) for e in s)))
    print(' in check',C(e['incheck'] for e in s))
    # own score awareness
    aw=C()
    for e in s:
        o=e['own']; n=e['own_next']
        if o is None or not n or n[0] is None: aw['na']+=1; continue
        d1=n[0]-o; d3=(n[-1]-o) if n[-1] is not None else d1
        aw['own drops>=100cp next move' if d1<=-100 else 'own drops>=100cp within 3' if d3<=-100 else 'own drop 30-100 within 3' if min(d1,d3)<=-30 else 'own score not falling']+=1
    print(' own-score reaction',dict(aw))
    own=[e['own'] for e in s if e['own'] is not None]
    q=st.quantiles(own,n=4); print(' own score at move: quartiles',[round(x) for x in q])
    print(' opp reply screening loss>=.10 (punishment missed):',sum(1 for e in s if (e['opp_loss'] or 0)>=.10),'/',len(s))
    print(' depth',C(e['depth'] for e in s).most_common(6))
    print(' result',dict(C(e['result'] for e in s)))
