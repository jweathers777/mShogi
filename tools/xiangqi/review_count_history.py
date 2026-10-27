#!/usr/bin/env python3
"""Independent fixture arithmetic, not a second product rules implementation.

Only the reviewed Rook/General histories are supported. No native/reference
outcome is used. Optional --output retains the diagnostic observations.
"""
import argparse
import json
from pathlib import Path
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--output', type=Path)
args = parser.parse_args()
p = Path(__file__).resolve().parents[2] / 'tests/fixtures/xiangqi/natural-count.json' 
data=json.loads(p.read_text())
reports=[]
for f in data['fixtures']:
    board={}
    fen=f['root_fen'].split()
    for rank,row in zip(range(9,-1,-1),fen[0].split('/')):
        file=0
        for c in row:
            if c.isdigit():file+=int(c)
            else:board[(file,rank)]=c;file+=1
        assert file==9
    side=fen[1]
    def coord(s):return (ord(s[0])-97,int(s[1]))
    def clear(a,b):
        if a[0]!=b[0] and a[1]!=b[1]:return False
        step=((b[0]>a[0])-(b[0]<a[0]),(b[1]>a[1])-(b[1]<a[1]))
        a=(a[0]+step[0],a[1]+step[1])
        while a!=b:
            if a in board:return False
            a=(a[0]+step[0],a[1]+step[1])
        return True
    def key():return (side,tuple(sorted(board.items())))
    occurrence={key():1};checks={'red':[],'black':[]};captures=[];known=0;since={'red':0,'black':0};observed=[]
    for ply,m in enumerate(f['moves'],1):
        a,b=coord(m[:2]),coord(m[2:]);piece=board.pop(a);actor='red' if piece.isupper() else 'black';assert actor==('red' if side=='w' else 'black')
        capture=board.pop(b,None);board[b]=piece
        if capture:captures.append(ply);known=0;since={'red':0,'black':0}
        else:known+=1
        enemy_king=next(x for x,c in board.items() if c==('k' if actor=='red' else 'K'))
        checking=any(c==('R' if actor=='red' else 'r') and clear(x,enemy_king) for x,c in board.items())
        if checking:checks[actor].append(ply)
        if checking and not capture:since[actor]+=1
        side='b' if side=='w' else 'w';k=key();occurrence[k]=occurrence.get(k,0)+1
        for expected in f.get('expected_claim_boundaries',[]):
            if expected['prefix_plies']==ply:
                count=known-max(0,since[expected['claimant']]-10)
                assert count==expected['count'],(f['id'],ply,count,expected)
                assert (count>=100)==expected['eligible']
                observed.append({'ply':ply,'claimant':expected['claimant'],'count':count,'eligible':count>=100})
    if 'red_check_plies' in f['expected']:
        assert checks['red']==f['expected']['red_check_plies'],(f['id'],checks)
        assert checks['black']==f['expected']['black_check_plies']
        assert captures==f['expected']['captures']
        assert max(occurrence.values())==f['expected']['repetition_max_occurrences']
    if 'capture_ply' in f['expected']:assert captures==[f['expected']['capture_ply']]
    reports.append({'id':f['id'],'method':'independent placement-only replay and straight-line Rook check/capture/count arithmetic; no native or reference calls','red_check_plies':checks['red'],'black_check_plies':checks['black'],'capture_plies':captures,'maximum_position_occurrences':max(occurrence.values()),'claim_boundaries':observed,'status':'pass'})
if args.output:
    args.output.write_text(json.dumps(reports,indent=2)+'\n')
print('PASS: four source-count fixtures; independent placement/check/count arithmetic; no history outcome engine used')
