from pathlib import Path, PurePosixPath
from zipfile import ZipFile
import json,hashlib,sys
def normalized(data):
 return ('\n'.join(line.rstrip() for line in data.decode().splitlines()).rstrip()+'\n').encode()
source='91eb13e2451af77150b82b62a448bf97e9d5ac58'
root=Path('docs/review/evidence/msh124')
summary_path=root/'vector-platform-audit.json'
summary=json.loads(summary_path.read_text()) if summary_path.exists() else {}
shots=['normal-initial-chess-normal-size','normal-initial-chess-minimum-size','normal-shogi-initial-sente','normal-shogi-minimum-size','normal-xiangqi-initial-red-normal-size','normal-xiangqi-initial-red-minimum-size','normal-moves-chess-1180-keyboard-history','normal-moves-chess-760-keyboard-history','normal-shogi-moves-shogi-1180-keyboard-history','normal-shogi-moves-shogi-760-keyboard-history','normal-xiangqi-moves-xiangqi-1180-keyboard-history','normal-xiangqi-moves-xiangqi-760-keyboard-history','normal-xiangqi-support-xiangqi-760x650','normal-shogi-chooser-shogi-760']
for platform in (sys.argv[1:] or ['linux','windows','macos']):
 path=Path('/private/tmp/msh124-'+platform+'-vector.zip');z=ZipFile(path);names=set(z.namelist());out=root/('vector-ci-macos' if platform=='macos' else 'vector-'+platform);out.mkdir(exist_ok=True)
 assert json.loads(z.read('gui-packages/normal/manifest.json'))['source_revision']==source
 normal=json.loads(z.read('gui-packages/normal/manifest.json'));verified=0;omitted=[]
 for name,h in normal['files'].items():
  key='gui-packages/normal/'+str(PurePosixPath(name.replace('\\','/')))
  if key in names: assert hashlib.sha256(z.read(key)).hexdigest()==h;verified+=1
  else:omitted.append(name)
 for key in ['package-smoke.json','gui-packages/normal/manifest.json']:
  (out/Path(key).name).write_bytes(normalized(z.read(key)))
 result={'source_revision':source,'run':41208986515,'artifact_zip_sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'normal_manifest_files_verified':verified,'manifest_files_absent_from_artifact':omitted,'normal_smoke':json.loads(z.read('package-smoke.json')),'scope':'macOS CI verifies package/engine only; local native Mac CUA evidence is separate.' if platform=='macos' else 'Actual installed normal and separately labeled verification package GUI interaction with native input and read-only DOM observations.'}
 if platform!='macos':
  report=json.loads(z.read('gui-evidence/gui-report.json'));assert report['source_revision']==source and report['status']=='passed';assert len(report['phases'])==15 and all(p['status']=='passed' for p in report['phases'])
  for name,h in report['files'].items():assert hashlib.sha256(z.read('gui-evidence/'+str(PurePosixPath(name.replace('\\','/'))))).hexdigest()==h,name
  (out/'gui-report.json').write_bytes(normalized(z.read('gui-evidence/gui-report.json')));(out/'verification-manifest.json').write_bytes(normalized(z.read('gui-packages/verification/manifest.json')))
  focuses=[];layouts=[];support=[];choosers=[]
  for p in report['phases']:
   for event in p.get('events',[]):
    op=event['operation'];v=event.get('observation')
    if op.startswith('Actual keyboard Moves focus outline'):
     assert v['focused'] and v['focusVisible'];assert v['outlineWidth']>0
     assert all(v['outline'][k]>=v['clip'][k]-.5 for k in ['left','top']) and all(v['outline'][k]<=v['clip'][k]+.5 for k in ['right','bottom']);focuses.append(v)
    elif op=='Observe one dark application shell, shared physical slots and initial cards':layouts.append(v)
    elif op.startswith('Keyboard About /'):support.append(v)
    elif op.startswith('Supported-only native game dropdown'):choosers.append(v)
  assert len(focuses)==6 and len(layouts)==6
  for size in [(1180,820),(760,650)]:
   group=[v for v in layouts if (v['viewport']['width'],v['viewport']['height'])==size];assert len(group)==3
   assert len({json.dumps(v['cards'],sort_keys=True) for v in group})==1
  for shot in shots:
   name=shot+'.webview.png';(out/name).write_bytes(z.read('gui-evidence/'+str(PurePosixPath(name.replace('\\','/')))))
  result.update(gui_report_file_hashes_verified=len(report['files']),phases=[{'phase':p['phase'],'status':p['status'],'host_sha256':p['host_sha256'],'engine_sha256':p['engine_sha256']} for p in report['phases']],focus_measurements=focuses,initial_layouts=layouts,support_interactions=support,chooser_interactions=choosers,screenshots_extracted=[s+'.webview.png' for s in shots])
 summary[platform]=result
summary_path.write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps({k:{'normal_manifest_files_verified':v['normal_manifest_files_verified'],'omitted':v['manifest_files_absent_from_artifact'],'gui_files_verified':v.get('gui_report_file_hashes_verified'),'phases':len(v.get('phases',[])),'focuses':len(v.get('focus_measurements',[]))} for k,v in summary.items()},indent=2))
