"""Reconstruct local frozen execution fixtures. STS source stays outside this package.
Usage: python reconstruct.py --sts /path/STS1-STS15_LAN_v3.epd --out /tmp/reconstructed
Requires python-chess1.11.2. Verify selected line hashes and exact output hashes.
"""
import argparse,json,hashlib
from pathlib import Path
import chess
p=Path(__file__).resolve().parent;a=argparse.ArgumentParser();a.add_argument('--sts',type=Path,required=True);a.add_argument('--out',type=Path,required=True);args=a.parse_args();args.out.mkdir(parents=True,exist_ok=True)
source=args.sts.read_bytes();fetch=json.loads((p/'fetch-manifest.json').read_text());expected=next(x['sha256']for x in fetch if x['name']=='sts-v3.epd');assert hashlib.sha256(source).hexdigest()==expected,'Source content changed; do not silently rebase expectations'
lines=source.decode().splitlines();meta=json.loads((p/'metadata.json').read_text())
for part in ['development','heldout']:
 rows=json.loads((p/(part+'-public.json')).read_text());refs=json.loads((p/(part+'-sts-references.json')).read_text())
 for x in refs:
  line=lines[x['source_line']-1];assert hashlib.sha256(line.encode()).hexdigest()==x['source_sha256'];b,ops=chess.Board.from_epd(line);assert ops['id']==x['source_id'];pts=dict(zip(str(ops['c9']).split(),map(int,str(ops['c8']).split())))
  for key in ['fen','initial_fen','source_root_fen']:x[key]=b.fen()
  x['expected_root_fen']=b.fen(en_passant='fen')
  x['expectation']={'tier':'published-strategic-preference','preferred_moves':[b.parse_uci(u).uci()for u,v in pts.items()if v==max(pts.values())],'published_points':pts,'unlisted':'unresolved; no automatic zero'}
  rows.append(x)
 raw=(json.dumps(sorted(rows,key=lambda x:x['id']),indent=2)+'\n').encode();sha=hashlib.sha256(raw).hexdigest();assert sha==meta[part]['sha256'],(part,'reconstruction differs',sha);(args.out/(part+'.json')).write_bytes(raw)
print('Both frozen execution fixtures reconstructed byte-identically. Original scratch paths are provenance; adapt raw receipt paths separately if needed.')
