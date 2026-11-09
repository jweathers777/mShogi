import sys,json,time,subprocess,hashlib
from pathlib import Path
b=Path('docs/m5/evidence/msh139');sys.path.insert(0,str(b));import assessment as a
def main():
 v=a.read(b/'verification.json');assert not v['inflight'];t=time.monotonic();rows=a.read(b/'quality-inputs.json');results=a.read(b/'quality-results.json');audit=[]
 for row in rows:
  if row['kind']=='tablebase':
   r=subprocess.run([v['build']['candidate']['path'],'shell'],input='position fen '+row['fen']+'\nlegal\nquit\n',capture_output=True,text=True,timeout=5)
   assert r.returncode==0 and not r.stderr
   legal=next(l for l in r.stdout.splitlines() if l.startswith('legal ')).split()[2:]
   assert len(legal)==len(set(legal)) and set(legal)==set(row['tb_moves'])
   audit.append({'id':row['id'],'native_legal_set_matches_all_tablebase_moves':True,'legal':legal})
 row=next(x for x in rows if x['id']=='POZ40');chosen=next(x for x in results['results'] if x['id']=='POZ40')['move'];r=subprocess.run([v['build']['candidate']['path'],'shell'],input='position startpos\nmove '+' '.join(row['history']+[chosen])+'\nstatus\nquit\n',capture_output=True,text=True,timeout=5)
 assert r.returncode==0 and not r.stderr and 'checkmate' in r.stdout
 out={'native_tablebase_move_sets':audit,'puzzle_POZ40_alternative':{'move':chosen,'native_status':r.stdout.splitlines()[-1],'accepted_equivalent_mate':True},'executed_quality_script_sha256':results['executed_code_sha256'],'quality_complete':len(results['results'])==20,'seconds':time.monotonic()-t,'caveat':'A short attempted Arasan handshake overlapped early tactical diagnostics and hit sandbox semaphore errors; all raw outcomes retained. No match overlapped. Repeat affectedearlychecks separately if needed; do not infer tacticalrating.'}
 a.save(b/'quality-audit.json',out)
 v.setdefault('diagnostics',[]).append({'kind':'quality_native_audit','seconds':out['seconds'],'results':'quality-audit.json','complete':True});a.save(b/'verification.json',v)
 print(json.dumps(out,indent=2))

if __name__=='__main__':
 with a.lock():main()
