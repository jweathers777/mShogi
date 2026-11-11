#!/usr/bin/env python3
"""Freeze100paired games into10reviewable batches after reviewed source admission."""
import hashlib,json
from pathlib import Path
HERE=Path(__file__).resolve().parent
SEED='msh161-main-coverage-v1'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def save(p,r):
 with p.open('x') as f:json.dump(r,f,indent=2);f.write('\n')
def main():
 roots=json.loads((HERE/'sources/main-roots.json').read_text());assert len(roots)==90 and len({r['id'] for r in roots})==90
 forbidden={r['id'] for r in json.loads((HERE/'sources/qualification-roots.json').read_text())};assert not forbidden & {r['id'] for r in roots}
 ordered=sorted(roots,key=lambda r:hashlib.sha256((SEED+r['id']).encode()).hexdigest())
 root=HERE/'main';root.mkdir(exist_ok=True);schedules=[]
 for index in range(10):
  pairs=[{'id':f'main-{index+1:02d}-{j:02d}','phase':'main-prepared','opponent':'oxidation060','opening':r} for j,r in enumerate(ordered[index*9:(index+1)*9])]
  pairs.append({'id':f'main-{index+1:02d}-unaided','phase':'main-unaided','opponent':'oxidation060','opening':{'id':f'unaided-main-{index+1:02d}','fen':None,'moves':[]}})
  schedule=root/f'batch-{index+1:02d}-schedule.json';save(schedule,{'stage':'main','seed':SEED,'pairs':pairs});schedules.append({'index':index+1,'path':str(schedule),'sha256':sha(schedule)})
 files=['main-method.md','main_batch.py','prepare_main.py','analysis.py','run_batch.py','screen_v1.py','summarize_analysis.py','phase_tags.py','build.json','sources/main-roots.json','sources/main-selection.json','../../competitive-gameplay-diagnosis.md']
 save(HERE/'main-freeze.json',{'method':'main-method.md','opponent':'oxidation060','main_games':200,'prepared_roots':90,'unaided_pairs':10,'seed':SEED,'qualification_separate':True,'files':{name:sha(HERE/name) for name in files},'schedules':schedules,'no_production_changes':True})
if __name__=='__main__':main()
