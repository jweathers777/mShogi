"""Necessary geometry only: no claim to enumerate full shogi positions."""
from collections import defaultdict

def sub(a,b): return a[0]-b[0], a[1]-b[1]
def cross(a,b): return a[0]*b[1]-a[1]*b[0]
def between(a,b,c):
    # b strictly between a and c
    return cross(sub(b,a),sub(c,a))==0 and (b[0]-a[0])*(b[0]-c[0])+(b[1]-a[1])*(b[1]-c[1]) < 0
def may_threaten(a,k,forward):
    dx,dy=sub(k,a)
    return (dx==0 or dy==0) or (abs(dx)==abs(dy)==1) or (abs(dx)==1 and dy==2*forward)
def joint_fit(points):
    return max(p[0] for p in points)-min(p[0] for p in points)<=8 and max(p[1] for p in points)-min(p[1] for p in points)<=8
def candidate_components(own,enemy,enemyforward):
    results=[]
    for r in range(1,9):
        points=[(enemy[0]+dx,enemy[1]+dy) for dx,dy in ((r,0),(0,r),(-r,0),(0,-r))]
        points=[p for p in points if p!=own and cross(sub(p,own),sub(enemy,own))!=0 and may_threaten(p,own,enemyforward) and joint_fit([own,enemy,p])]
        graph={a:[] for a in points}
        for a in points:
            for b in points:
                if a==b: continue
                dx,dy=sub(b,a)
                if abs(dx)!=abs(dy): continue
                # Cannot leap own king; cannot leave same enemy-king ray: axes already distinct.
                if between(a,own,b): continue
                # If a is strictly between own king and b, it already blocks
                # any ray check that the arrival at b could evade.
                if between(own,a,b): continue
                if joint_fit([own,enemy,a,b]): graph[a].append(b)
        # Enumerate simple directed cycles, all <=4 points; every actual closed
        # bishop walk contains one. Keep vertex sets and SCCs separately.
        cycles=set()
        for start in points:
            def visit(path):
                for b in graph[path[-1]]:
                    if b==start and len(path)>1:
                        cycles.add(tuple(sorted(path)))
                    elif b not in path:
                        visit(path+[b])
            visit([start])
        for cycle in sorted(cycles):
            results.append((r,cycle))
    return results

def enumerate_pairs():
    K=(0,0)
    pairs=[]
    for x in range(-8,9):
        for y in range(-8,9):
            L=(x,y)
            if max(abs(x),abs(y))<=1: continue
            own=candidate_components(L,K,-1)
            enemy=candidate_components(K,L,1)
            for a in own:
                for b in enemy:
                    if not joint_fit([K,L,*a[1],*b[1]]): continue
                    pairs.append((L,a,b))
    return pairs

if __name__=='__main__':
    pairs=enumerate_pairs()
    print('paired necessary-geometry candidates',len(pairs))
    print('cycle sizes', sorted(set((len(a[1]),len(b[1])) for _,a,b in pairs)))
    for L,a,b in pairs:
        if L[0]>=0:
            print(L,'S',a,'T',b,'shared',sorted(set(a[1])&set(b[1])))
