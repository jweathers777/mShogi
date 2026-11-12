"""Prepare all primary pairs as ten reviewed batches from tested worker source.
No engine or cloud operation. Original pair ordering and scientific inputs preserved.
"""
import hashlib,json,subprocess
from pathlib import Path
REPO=Path(__file__).resolve().parents[5]
OUT=Path(__file__).resolve().parent
SOURCE=Path('/private/tmp/msh161-worker-source-51351c48')
REV='51351c48ea1adf9ac96011aec1440e44ceb741a4'
PYTHON='/private/tmp/msh162-controller/bin/python'
IMAGE='us-central1-docker.pkg.dev/mshogi/mshogi-experiments/worker@sha256:ad95d85a8f555acaaef2adafecb92b0277983fb46ce6da83c3c192267dcc43f7'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def save(p,d):
 with p.open('x') as f:json.dump(d,f,indent=2);f.write('\n')

def main():
 export=read(OUT/'worker-source-export.json')
 assert all(sha(SOURCE/n)==h for n,h in export['files'].items())
 freeze=read(REPO/'docs/m5/evidence/msh161/main-freeze.json')
 batches=[];seen=[]
 for entry in freeze['schedules']:
  index=entry['index'];schedule=REPO/'docs/m5/evidence/msh161/main'/Path(entry['path']).name
  assert sha(schedule)==entry['sha256']
  specs=read(schedule)['pairs'];ids=[s['id'] for s in specs];assert len(specs)==10
  experiment=f'msh161-primary-v1-batch-{index:02d}'
  temporary=SOURCE/f'primary-full-{index:02d}.json'
  subprocess.run([PYTHON,'-m','tools.experiments.cli','prepare','--build',str(REPO/'docs/experiments/linux-build.json'),'--experiment',experiment,'--source-revision',REV,'--cloud-config',str(OUT/'cloud-config.json'),'--image',IMAGE,'--output',str(temporary)],cwd=SOURCE,check=True,capture_output=True)
  manifest=read(temporary)
  assert len(manifest['pipelines'])==100 and manifest['cohort']=='gcp-primary' and manifest['pilot_pairs'] is None
  manifest['jobs']=[j for j in manifest['jobs'] if j['spec']['id'] in ids]
  roots=[j for j in manifest['jobs'] if j['kind']=='game_pair']
  assert [j['spec'] for j in roots]==specs and len(manifest['jobs'])==20
  manifest['pipelines']=[j['id'] for j in roots]
  manifest['primary_batch']={'index':index,'total_batches':10,'cohort':'msh161-primary-v1','original_schedule_sha256':entry['sha256'],'full_cohort_pairs':100,'full_cohort_games':200,'reason':'Ownerapprovedfullstudy;20gamereviewcadence;notpilotselection'}
  path=OUT/f'batch-{index:02d}-manifest.json';save(path,manifest)
  batches.append({'index':index,'experiment':experiment,'manifest':str(path.relative_to(REPO)),'sha256':sha(path),'pair_ids':ids,'game_pair_job_ids':manifest['pipelines']});seen+=ids
 assert len(seen)==len(set(seen))==100
 save(OUT/'cohort-freeze.json',{'cohort':'msh161-primary-v1','games':200,'pairs':100,'fresh_primary':True,'pilot_or_mac_results_reused':False,'review_every_games':20,'source_revision':REV,'worker_image':IMAGE,'config_sha256':sha(OUT/'cloud-config.json'),'original_study_freeze_sha256':sha(REPO/'docs/m5/evidence/msh161/main-freeze.json'),'batches':batches,'launch_policy':'Submitonebatchat atimeafterreview;max8dedicatedC3workers;consequential20Madjudicationseparate;noautomaticwholecampaignlaunch','owner_authorization':'Human12November2026: Readthehandoffandproceedwiththefullcloudstudy.','cost_scope':'Full200gamesandnecessaryanalysis authorized;recordactualresource/cost. PilotUS30restrictiondoesnotdefinefullstudyscope. No16workerexpansion.'})
 print('Prepared10primarybatchmanifests;all100frozenpairs/200games;nojobslaunched')
if __name__=='__main__':main()
