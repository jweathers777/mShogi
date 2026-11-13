"""Offline check. Requires the existing python-chess package; no engine process."""
import sys,json,pathlib,hashlib,collections
import chess
if not __debug__:
    raise RuntimeError("Optimized Python disables audit checks")
h=pathlib.Path('docs/m5/evidence/msh161/cloud-primary');f=h/'final-analysis'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def key(r):return r['pair'],r['game'],r['ply']
def resolve(s):
 p=pathlib.Path(s)
 return p if p.exists() else h/p
n=json.load(open(h/'primary-cumulative.json'));expected={key(r):r for r in n['normalized_selected_events'] if r['candidate'] and r['status']=='supported_finite'}
seen=set();counts={};hashes={};samples=[]
for name in ['01-04','05-07','08-10']:
 p=f/f'behavior-coding-{name}.json'; hashes[str(p)]=sha(p);d=json.load(open(p));rs=d.get('events',d.get('entries'));counter=collections.Counter()
 for r in rs:
  if r['status']=='supported_finite':
   k=key(r);assert k in expected and k not in seen;seen.add(k)
   assert (r['played'],r['alternative'])==(expected[k]['played'],expected[k]['alternative']); counter['supported_roots']+=1
  qs=r['query_sources'];qs=qs if isinstance(qs,list) else list(qs.values())
  for q in qs:
   p=resolve(q['path']);assert sha(p)==q['sha256'];raw=json.load(open(p));dec=raw['decision'];b=chess.Board(dec['initial_fen'])
   for uci in dec['history']:b.push_uci(uci)
   assert b.fen()==r['root_fen'];assert len(dec['history'])==r['ply'];counter['query_hashes_and_history_roots']+=1
  lines=r.get('legal_reference_lines_san') or list(r['legal_lines'].values())
  for line in lines:
   b=chess.Board(r['root_fen'])
   for san in line:b.push_san(san);counter['legal_line_plies']+=1
   counter['legal_lines']+=1
 counts[name]=dict(counter)
assert seen==set(expected)
result={'status':'pass','scope':'All three packets; exact numerical support set, query hashes, full-history root identity, all stored SAN lines','python_chess_version':chess.__version__,'implementation_sha256':sha(pathlib.Path(__file__)),'source_hashes':hashes,'counts':counts,'supported_roots':len(seen),'limits':'Legal replay verifies possible lines; it does not prove a line is forced or a behavioral category is correct. Original native histories remain authoritative.'}
(f/'audit-behavior-evidence.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
