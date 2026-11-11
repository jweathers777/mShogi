"""Create a minimal, hash-verified worker image context; never copy live evidence."""
import argparse
import shutil
from pathlib import Path
from .core import decode, encode, file_hash, validate_manifest

def package(manifest, repo, binaries, output):
    validate_manifest(manifest)
    if output.exists(): raise FileExistsError(output)
    output.mkdir(parents=True)
    sources={}; engines={}
    for job in manifest['jobs']:
        for name,sha in job['source_files'].items():
            if name in sources and sources[name]!=sha:raise ValueError('source identity conflict')
            sources[name]=sha
        for engine in job['build']['engines'].values():
            path=Path(engine['path'])
            if path.parent!=Path('/opt/engines'):raise ValueError('cloud executables must live in /opt/engines')
            if path.name in engines and engines[path.name]!=engine['sha256']:raise ValueError('engine name collision')
            engines[path.name]=engine['sha256']
    for name,sha in sources.items():
        source=repo/name
        if file_hash(source)!=sha:raise ValueError('source changed: '+name)
        target=output/name;target.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(source,target)
    for name,sha in engines.items():
        source=binaries/name
        if file_hash(source)!=sha:raise ValueError('binary differs: '+name)
        target=output/'build-context/bin'/name;target.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(source,target);target.chmod(0o755)
    for name in ('Dockerfile','requirements.txt'):
        source=repo/'tools/experiments'/name;target=output/'tools/experiments'/name;target.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(source,target)
    receipt={'sources':sources,'engines':engines,'packaging':{name:file_hash(repo/'tools/experiments'/name) for name in ('Dockerfile','requirements.txt')}}
    (output/'package-receipt.json').write_bytes(encode(receipt))
    return receipt

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--manifest',required=True);p.add_argument('--binaries',required=True);p.add_argument('--output',required=True);args=p.parse_args()
    package(decode(Path(args.manifest).read_bytes()),Path(__file__).resolve().parents[2],Path(args.binaries),Path(args.output))
if __name__=='__main__':main()
