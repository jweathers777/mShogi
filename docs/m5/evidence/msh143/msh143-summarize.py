import re,json
from pathlib import Path
text=Path('/private/tmp/msh142/msh143-baseline-cpu.txt').read_text().split('Call graph:\n',1)[1].split('Total number in stack',1)[0]
stack=[];total=0;inc={}
groups={'quiescence':['Engine10quiescence'],'exchange':['chess_conventional8exchange'],'evaluation':['chess_conventional_eval7dynamic'],'native_turn':['SearchCursor4turn'],'native_node':['SearchCursor4node'],'native_preview':['SearchCursor7preview'],'native_push':['SearchCursor13push_with_key','SearchCursor4push','SearchCursor12push_preview'],'native_capture_generation':['Position23captures_and_promotions','Position17legal_captures_to'],'native_mate_queries':['Position18queen_contact_mate','Position29has_queen_contact_mate_threat'],'native_apply':['Position5apply'],'native_structure':['Position18validate_structure'],'clock':['Instant7elapsed'],'key_comparison':['SearchKey','same_future','same_repetitions'],'alloc':['malloc','realloc','alloc::','RawVec','dealloc','_xzm_free','_free']}
def finish():
 global total
 s=stack.pop();weight=s['n']-s['children'];assert weight>=0
 total+=weight
 path=' '.join([p['name'] for p in stack]+[s['name']])
 for k,needles in groups.items():
  if any(n in path for n in needles):inc[k]=inc.get(k,0)+weight
for line in text.splitlines():
 m=re.match(r'^([ +!:|.]*)(\d+) (.+?)  \(in ',line)
 if not m:continue
 d=len(m[1]);n=int(m[2])
 while stack and stack[-1]['d']>=d:finish()
 if stack:stack[-1]['children']+=n
 stack.append(dict(d=d,n=n,name=m[3],children=0))
while stack:finish()
r={'sample_weight':total,'inclusive_overlapping_buckets':{k:{'weight':v,'percent':round(100*v/total,2)} for k,v in sorted(inc.items(),key=lambda x:-x[1])},'caveat':'Inclusive categories overlap. Inlined functions disappear. 4second wall sample of5 sequential1s searches at22bf634; root correctness tests possibly contended; qualitative only.'}
Path('/private/tmp/msh142/msh143-baseline-cpu-summary.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r,indent=2))
