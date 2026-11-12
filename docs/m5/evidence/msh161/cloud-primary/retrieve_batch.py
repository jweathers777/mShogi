"""Read-only cloud retrieval and offline verification. No engines or cloud jobs."""
import argparse,concurrent.futures,json,time,hashlib,base64,sys
from pathlib import Path
REPO=Path(__file__).resolve().parents[5];sys.path.insert(0,str(REPO))
from tools.experiments.core import LocalStore,decode,encode,verify_result,prefix
from tools.experiments.cli import all_jobs
from google.cloud import storage
import google_crc32c

def main():
 ap=argparse.ArgumentParser();ap.add_argument('batch',type=int);args=ap.parse_args()
 h=Path(__file__).resolve().parent;manifest=decode((h/f'batch-{args.batch:02d}-manifest.json').read_bytes());config=decode((h/'cloud-config.json').read_bytes())
 bucket,_,base=config['store_uri'][5:].partition('/');base=base.rstrip('/')+'/'
 exp=manifest['experiment'];local=LocalStore(h/'local-cache');out=h/f'batch-{args.batch:02d}-review';out.mkdir(exist_ok=True)
 state={'status':'retrieving','experiment':exp,'started_epoch':time.time(),'local_store':str(local.root)}
 def save(): (out/'retrieval-state.json').write_bytes(encode(state))
 save();start=time.monotonic()
 try:
  client=storage.Client(project=config['project']);blobs=list(client.list_blobs(bucket,prefix=base+exp+'/'))
  inventory=[{'name':b.name,'generation':b.generation,'bytes':b.size,'crc32c':b.crc32c} for b in blobs]
  (out/'remote-inventory.json').write_bytes(encode(inventory));state.update(objects=len(blobs),bytes=sum(b.size for b in blobs));save()
  def fetch(blob):
   key=blob.name[len(base):]
   if local.exists(key):
    data=local.get(key)
    if len(data)!=blob.size or base64.b64encode(google_crc32c.Checksum(data).digest()).decode()!=blob.crc32c:raise ValueError('Existing cache differs: '+key)
   else:
    if blob.size>128*1024*1024:raise ValueError('Oversized object')
    data=blob.download_as_bytes(if_generation_match=blob.generation,checksum='crc32c',timeout=120)
    if len(data)!=blob.size:raise ValueError('Object size differs')
    if not local.put(key,data) and local.get(key)!=data:raise ValueError('Concurrent cache conflict')
   return key
  with concurrent.futures.ThreadPoolExecutor(32) as pool:
   for i,_ in enumerate(pool.map(fetch,blobs),1):
    if i%1000==0:print('retrieved',i,flush=True)
  jobs=all_jobs(manifest,local);index={};attempts=[]
  for job in jobs:
   result=verify_result(local,job);pair=job['spec']['id'];row=index.setdefault(pair,{'pair':pair,'opening':job['spec']['opening'],'confirmations':[]})
   entry={'job':job,'success_sha256':hashlib.sha256(local.get(prefix(job)+'/success.json')).hexdigest(),'success':str(local.path(prefix(job)+'/success.json')),'artifacts':{name:{**value,'local':str(local.path(value['key']))} for name,value in result['artifacts'].items()}}
   if job['kind']=='confirmation':row['confirmations'].append(entry)
   else:row[job['kind']]=entry
  (out/'review-index.json').write_bytes(encode(index))
  state.update(status='retrieved_verified_needs_independent_review',verified_jobs=len(jobs),pairs=len(index),seconds=time.monotonic()-start,inventory_sha256=hashlib.sha256((out/'remote-inventory.json').read_bytes()).hexdigest(),method='32generation-pinnedCRC32Cdownloads;existingcacheCRCchecked;allstatic/derivedsuccessartifactSHA256verified',no_engines=True)
 except BaseException as exc:state.update(status='failed_preserved_for_review',error=repr(exc));raise
 finally:state['ended_epoch']=time.time();save()
 print(json.dumps({k:v for k,v in state.items() if k!='error'}),flush=True)
if __name__=='__main__':main()
