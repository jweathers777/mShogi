"""Versioned execution/evidence services. No game rules or evaluation live here."""
import concurrent.futures
import copy
import hashlib
import importlib.metadata
import json
import os
import platform
import re
import resource
import socket
import tempfile
import time
import uuid
import urllib.request
from pathlib import Path

SCHEMA = 1
KINDS = {'game_pair', 'reference_screening', 'confirmation'}

def encode(value):
    return (json.dumps(value, sort_keys=True, separators=(',', ':'), allow_nan=False)+'\n').encode()

def decode(data):
    def unique(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise ValueError('duplicate JSON key: '+key)
            result[key] = value
        return result
    return json.loads(data, object_pairs_hook=unique)

def digest(data):
    return hashlib.sha256(data).hexdigest()

def file_hash(path):
    h = hashlib.sha256()
    with open(path, 'rb') as stream:
        for block in iter(lambda: stream.read(1024*1024), b''):
            h.update(block)
    return h.hexdigest()

def safe_key(key):
    if not isinstance(key, str) or not key or key.startswith('/') or any(x in ('', '.', '..') for x in key.split('/')) or '\\' in key:
        raise ValueError('unsafe storage key')
    return key

def job_id(job):
    body = {k:v for k,v in job.items() if k != 'id'}
    return digest(encode(body))

def seal(job):
    job = copy.deepcopy(job)
    job['schema'] = SCHEMA
    job['id'] = job_id(job)
    return job

def validate_job(job):
    if job.get('schema') != SCHEMA or job.get('kind') not in KINDS or job.get('id') != job_id(job):
        raise ValueError('unknown or altered job')
    if not re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9_.-]{0,95}', job.get('experiment', '')):
        raise ValueError('invalid experiment ID')
    if not re.fullmatch(r'[0-9a-f]{40}', job.get('source_revision', '')):
        raise ValueError('source revision must be full Git SHA')
    if not isinstance(job.get('expected_outputs'), list) or not job['expected_outputs']:
        raise ValueError('expected outputs required')
    for name in job['expected_outputs']:
        safe_key(name)
    build = job.get('build', {})
    if not build.get('source') or not build.get('compiler') or not build.get('architecture') or not build.get('build_flags'):
        raise ValueError('build source/compiler/architecture/flags required')
    for name in ('runner', 'candidate' if job['kind']=='game_pair' else 'stockfish19'):
        if name not in build.get('engines', {}):
            raise ValueError('missing engine '+name)
    for engine in build['engines'].values():
        if not re.fullmatch('[0-9a-f]{64}', engine.get('sha256', '')) or not engine.get('source') or not Path(engine.get('path', '')).is_absolute():
            raise ValueError('engine identity requires absolute path/hash/source')
    if not job.get('environment') or not job.get('source_files'):
        raise ValueError('environment and source file hashes required')
    for name, sha in job['source_files'].items():
        safe_key(name)
        if not re.fullmatch('[0-9a-f]{64}', sha):
            raise ValueError('invalid source hash')
    if job['kind'] == 'game_pair':
        spec = job.get('spec', {})
        if spec.get('opponent') not in build['engines'] or not spec.get('opening'):
            raise ValueError('pair opening/opponent required')
    else:
        if not 1 <= job.get('nodes', 0) <= 20000000:
            raise ValueError('invalid node limit')
        if job['kind']=='reference_screening' and job['nodes'] != 500000:
            raise ValueError('frozen screen uses 500000 nodes')
        if 'report' not in job.get('inputs', {}):
            raise ValueError('analysis requires verified report input')
    for name, value in job.get('inputs', {}).items():
        if not re.fullmatch(r'[a-z][a-z0-9_-]*', name): raise ValueError('unsafe input name')
        if set(value) != {'job', 'path'} or not re.fullmatch('[0-9a-f]{64}', value['job']):
            raise ValueError('invalid input reference')
        safe_key(value['path'])
    return job

class LocalStore:
    def __init__(self, root):
        self.root = Path(root).resolve()
        self.root.mkdir(parents=True, exist_ok=True)
    def path(self, key):
        path = self.root / safe_key(key)
        if not path.resolve().is_relative_to(self.root):
            raise ValueError('store symlink escape')
        return path
    def put(self, key, data):
        path = self.path(key)
        path.parent.mkdir(parents=True, exist_ok=True)
        fd, name = tempfile.mkstemp(prefix='.pending-', dir=path.parent)
        try:
            with os.fdopen(fd, 'wb') as stream:
                stream.write(data); stream.flush(); os.fsync(stream.fileno())
            try:
                os.link(name, path)
            except FileExistsError:
                return False
            return True
        finally:
            os.unlink(name)
    def get(self, key):
        return self.path(key).read_bytes()
    def exists(self, key):
        return self.path(key).is_file()
    def list(self, prefix):
        safe_key(prefix.rstrip('/'))
        return sorted(str(p.relative_to(self.root)) for p in self.root.rglob('*') if p.is_file() and str(p.relative_to(self.root)).startswith(prefix) and not p.name.startswith('.pending-'))
    def delete(self, key, expected):
        # Caller owns claim; administrative recovery is only permitted with all workers stopped.
        path = self.path(key)
        try:
            if path.read_bytes() != expected:
                raise ValueError('claim ownership changed')
            path.unlink()
            return True
        except FileNotFoundError:
            return False

def prefix(job):
    return job['experiment']+'/jobs/'+job['id']

def verify_result(store, job):
    validate_job(job)
    result = decode(store.get(prefix(job)+'/success.json'))
    if result.get('job') != job or result.get('status') != 'complete':
        raise ValueError('result identity mismatch')
    if not set(job['expected_outputs']).issubset(result.get('artifacts', {})):
        raise ValueError('missing required result artifacts')
    for path, artifact in result['artifacts'].items():
        safe_key(path)
        if artifact['key'] != prefix(job)+'/attempts/'+result['attempt']+'/artifacts/'+path:
            raise ValueError('artifact outside owning attempt')
        data = store.get(artifact['key'])
        if len(data) != artifact['bytes'] or digest(data) != artifact['sha256']:
            raise ValueError('corrupt result artifact: '+path)
    return result

def environment():
    value = {'platform': platform.platform(), 'architecture': platform.machine(), 'python': platform.python_version(), 'hostname': socket.gethostname(), 'cpu_count': os.cpu_count(), 'load': list(os.getloadavg())}
    value['packages']={}
    for name in ('chess','google-cloud-storage','google-cloud-pubsub'):
        try: value['packages'][name]=importlib.metadata.version(name)
        except importlib.metadata.PackageNotFoundError: pass
    if hasattr(os, 'sched_getaffinity'):
        value['cpu_affinity'] = sorted(os.sched_getaffinity(0))
    if Path('/proc/cpuinfo').exists():
        value['cpuinfo'] = Path('/proc/cpuinfo').read_text()
    return value

def run_job(job, store, repo, executor=None):
    validate_job(job)
    base = prefix(job)
    if store.exists(base+'/success.json'):
        return verify_result(store, job)
    if store.exists(job['experiment']+'/cancel.json'):
        raise RuntimeError('experiment cancelled; no new jobs launched')
    claim = encode({'token':uuid.uuid4().hex, 'pid':os.getpid(), 'host':socket.gethostname(), 'started_epoch':time.time()})
    if not store.put(base+'/claim.json', claim):
        raise RuntimeError('job claimed; running or interrupted, reconcile before recovery')
    attempt = uuid.uuid4().hex
    started = time.monotonic()
    receipt = {'schema':SCHEMA, 'job':job, 'attempt':attempt, 'status':'failed', 'started_epoch':time.time(), 'environment':environment()}
    usage = resource.getrusage(resource.RUSAGE_CHILDREN)
    store.put(base+'/attempts/'+attempt+'/started.json', encode(receipt))
    try:
        # Recheck after claim: a racing completed worker must never be recomputed.
        if store.exists(base+'/success.json'):
            return verify_result(store, job)
        actual = receipt['environment']
        expected = job['environment']
        if expected.get('cloud'):
            observed={}
            for field in ('machine-type','zone','id','cpu-platform'):
                request=urllib.request.Request('http://metadata.google.internal/computeMetadata/v1/instance/'+field,headers={'Metadata-Flavor':'Google'})
                with urllib.request.urlopen(request,timeout=5) as response: observed[field]=response.read(4096).decode()
            actual['gcp']=observed
            actual['batch']={k:v for k,v in os.environ.items() if k in ('BATCH_TASK_INDEX','BATCH_TASK_COUNT','BATCH_JOB_UID','BATCH_TASK_RETRY_ATTEMPT')}
            config=expected['cloud']['config']
            if observed['machine-type'].rsplit('/',1)[-1] != config['machine_type']:
                raise ValueError('actual cloud machine differs from frozen allocation')
            if not observed['zone'].rsplit('/',1)[-1].startswith(config['region']+'-'):
                raise ValueError('actual cloud region differs from frozen allocation')
        if expected.get('architecture') != actual['architecture']:
            raise ValueError('worker CPU architecture differs from frozen environment')
        if expected.get('system') != platform.system():
            raise ValueError('worker operating system differs from frozen environment')
        for path, sha in job['source_files'].items():
            if file_hash(Path(repo)/path) != sha:
                raise ValueError('source file differs: '+path)
        for engine in job['build']['engines'].values():
            if file_hash(engine['path']) != engine['sha256']:
                raise ValueError('engine executable differs: '+engine['path'])
        with tempfile.TemporaryDirectory(prefix='msh-experiment-') as temp:
            root = Path(temp); output = root/'output'; output.mkdir(); inputs = {}
            try:
                for name, ref in job.get('inputs', {}).items():
                    parent = decode(store.get(job['experiment']+'/jobs/'+ref['job']+'/success.json'))
                    if parent['job']['id'] != ref['job'] or parent['job']['experiment'] != job['experiment']:
                        raise ValueError('dependency receipt substituted')
                    parent = verify_result(store, parent['job'])
                    artifact = parent['artifacts'][ref['path']]
                    target = root/('input-'+name)
                    # Frozen report reader selects gzip by extension.
                    if ref['path'].endswith('.gz'): target = target.with_suffix('.gz')
                    target.write_bytes(store.get(artifact['key'])); inputs[name] = target
                if executor is None:
                    from .chess import execute
                    executor = execute
                receipt['result'] = executor(job, output, inputs, Path(repo))
                if not receipt['result'].get('complete'):
                    raise ValueError('adapter did not establish completeness')
                for required in job['expected_outputs']:
                    if not (output/required).is_file():
                        raise ValueError('missing expected output '+required)
                receipt['status'] = 'complete'
            finally:
                artifacts = {}
                for path in sorted(output.rglob('*')):
                    if not path.is_file(): continue
                    if path.is_symlink(): raise ValueError('artifact symlink refused')
                    data = path.read_bytes(); name = str(path.relative_to(output))
                    key = base+'/attempts/'+attempt+'/artifacts/'+name
                    if not store.put(key, data): raise RuntimeError('attempt artifact collision')
                    artifacts[name] = {'key':key, 'sha256':digest(data), 'bytes':len(data)}
                receipt['artifacts'] = artifacts
    except BaseException as exc:
        receipt.update(status='failed', error=repr(exc))
        raise
    finally:
        end = resource.getrusage(resource.RUSAGE_CHILDREN)
        receipt.update(ended_epoch=time.time(), wall_seconds=time.monotonic()-started, child_cpu_seconds=end.ru_utime+end.ru_stime-usage.ru_utime-usage.ru_stime, child_maxrss=end.ru_maxrss)
        try:
            store.put(base+'/attempts/'+attempt+'/receipt.json', encode(receipt))
            if receipt['status']=='complete':
                if not store.put(base+'/success.json', encode(receipt)):
                    verify_result(store, job)
        finally:
            store.delete(base+'/claim.json', claim)
    return verify_result(store, job)

def validate_manifest(manifest):
    if manifest.get('schema') != SCHEMA or not manifest.get('jobs'):
        raise ValueError('invalid experiment manifest')
    jobs = {j['id']:validate_job(j) for j in manifest['jobs']}
    if len(jobs) != len(manifest['jobs']): raise ValueError('duplicate jobs')
    if any(j['experiment'] != manifest['experiment'] for j in jobs.values()): raise ValueError('mixed experiment identities')
    for job in jobs.values():
        for ref in job.get('inputs', {}).values():
            if ref['job'] not in jobs: raise ValueError('missing dependency')
            if jobs[ref['job']]['kind'] != 'game_pair': raise ValueError('current Chess adapter requires native pair report dependency')
            if ref['path'] not in jobs[ref['job']]['expected_outputs']: raise ValueError('input is not a declared parent output')
    visited = set()
    def visit(key, chain):
        if key in chain: raise ValueError('dependency cycle')
        for ref in jobs[key].get('inputs', {}).values(): visit(ref['job'], chain|{key})
        visited.add(key)
    for key in jobs: visit(key, set())
    pipelines = manifest.get('pipelines', [])
    if len(pipelines) != len(set(pipelines)) or any(p not in jobs for p in pipelines): raise ValueError('invalid pipelines')
    roots={key for key,j in jobs.items() if not j.get('inputs')}
    if set(pipelines) != roots or any(jobs[key]['kind'] != 'game_pair' for key in roots):
        raise ValueError('pipeline roots must cover every independent game pair')
    if any(j['kind'] != 'game_pair' and len(j.get('inputs',{})) != 1 for j in jobs.values()):
        raise ValueError('analysis requires exactly one report dependency')
    return jobs

def status(manifest, store):
    jobs = validate_manifest(manifest)
    rows = []
    for job in jobs.values():
        base = prefix(job); state = 'pending'; detail = None
        if store.exists(base+'/success.json'):
            try: verify_result(store, job); state='completed'
            except (ValueError, FileNotFoundError, KeyError) as exc: state='invalid'; detail=str(exc)
        elif store.exists(base+'/claim.json'): state='running_or_interrupted'
        elif store.list(base+'/attempts/'): state='failed'
        rows.append({'id':job['id'], 'kind':job['kind'], 'state':state, 'detail':detail})
    return rows
