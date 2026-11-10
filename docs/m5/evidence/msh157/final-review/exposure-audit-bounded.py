from pathlib import Path
import json,chess,subprocess,re,gzip,tarfile,io,hashlib,time,collections
P=Path('/tmp/msh156-data');roots=[r for part in ['development','heldout']for r in json.loads((P/(part+'.json')).read_text())];placements=collections.defaultdict(set);ids=collections.defaultdict(set)
def add(b,id):
 for q in [b,b.mirror(),b.transform(chess.flip_horizontal),b.mirror().transform(chess.flip_horizontal)]:placements[q.board_fen()].add(id)
for r in roots:
 b=chess.Board(r['expected_root_fen']);add(b,r['id'])
 for seq in [r['expectation'].get('continuation',[])]+[p['line'].split()for p in r['expectation'].get('published_analysis',{}).get('pvs',[])]:
  q=b.copy()
  for u in seq:q.push_uci(u);add(q,r['id'])
 if r['source_kind']=='puzzle' or (r['source_kind']=='tablebase'and len(r['source_id'])==5):ids[r['source_id']].add(r['id'])
 # STS source ids are quoted full names; include verbatim.
 if r['source_kind']=='sts':ids[r['source_id']].add(r['id'])
fenpat=re.compile(rb'(?<![A-Za-z0-9/])(?:[prnbqkPRNBQK1-8]{1,8}/){7}[prnbqkPRNBQK1-8]{1,8}(?![A-Za-z0-9/])');idpat=re.compile(rb'(?<![A-Za-z0-9])(?:'+b'|'.join(re.escape(s.encode())for s in ids)+rb')(?![A-Za-z0-9])')
files=[]
for line in Path('/tmp/msh157-historical-tree.txt').read_text().splitlines():
 fields,name=line.split('\t',1);a=fields.split();size=int(a[3])if a[3]!='-'else 0
 if not name.startswith(('docs/','crates/','tests/','tools/','backlog/','experiments/','scripts/')):continue
 if any(name.endswith(e)for e in ['.png','.jpg','.jpeg','.gif','.ico','.icns','.woff','.ttf','.pdf','.mp4','.zip','.dll','.exe','.dylib']):continue
 files.append((a[2],name,size))
hits=[];scanned=[];skips=[];total=0;limit=600*1024**2

def scan(name,data):
 global total
 total+=len(data)
 if total>limit:raise RuntimeError('decompressed cap reached')
 bs=set(m.group().decode()for m in fenpat.finditer(data));matched=set()
 for v in bs:
  for id in placements.get(v,[]):matched.add((id,'board-or-known-line'))
 for source_id,root_ids in ids.items():
  if source_id.encode() in data and re.search(rb'(?<![A-Za-z0-9])'+re.escape(source_id.encode())+rb'(?![A-Za-z0-9])',data):
   for id in root_ids:matched.add((id,'source-id-candidate'))
 for id,kind in sorted(matched):hits.append({'root_id':id,'partition':next(r['partition']for r in roots if r['id']==id),'path':name,'match':kind})
 scanned.append({'path':name,'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest()})
 if len(scanned)%100==0:
  Path('/tmp/msh157-exposure-progress.json').write_text(json.dumps({'payloads':len(scanned),'bytes':total,'hits':len(hits),'last_path':name})+'\n')
p=subprocess.Popen(['git','cat-file','--batch'],stdin=subprocess.PIPE,stdout=subprocess.PIPE)
for blob,name,size in files:
 p.stdin.write((blob+'\n').encode());p.stdin.flush();header=p.stdout.readline().split();n=int(header[2]);data=p.stdout.read(n);assert p.stdout.read(1)==b'\n'
 if name.endswith('.tar.gz'):
  with tarfile.open(fileobj=io.BytesIO(data),mode='r:gz')as t:
   for member in t:
    if member.isfile():scan(name+'!'+member.name,t.extractfile(member).read())
 elif name.endswith('.gz'):
  try:scan(name+'!decompressed',gzip.decompress(data))
  except Exception as e:skips.append({'path':name,'error':str(e)})
 else:scan(name,data)
p.stdin.close();p.wait()
result={'revision':'fb87ca2','historical_files':len(files),'scanned_payloads':len(scanned),'uncompressed_bytes':total,'skips':skips,'hits':hits,'scope':'all nonimage/nonbinary source/docs/backlog/tools files plus gzip and tar.gz payloads; sourceID or canonical board-placement match for selected roots and knownsolution/multiPV continuations; conservative board-only matches may differ side/counters','no_candidate_results_accessed':True,'frozen_execution_unchanged':True}
Path('/tmp/msh157-exposure-audit.json').write_text(json.dumps(result,indent=2)+'\n');Path('/tmp/msh157-exposure-scanned.json').write_text(json.dumps(scanned,indent=2)+'\n');print(json.dumps({'files':len(files),'payloads':len(scanned),'bytes':total,'hits':len(hits),'matched_roots':len({h['root_id']for h in hits}),'skips':skips},indent=2))
