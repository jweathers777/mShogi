#!/usr/bin/env python3
"""Bounded, digest-pinned Linux native build; no games and no opponent rebuild."""
import argparse, hashlib, json, shutil, subprocess, time
from pathlib import Path

REPO = Path(__file__).resolve().parents[6]
HERE = Path(__file__).resolve().parent

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def main():
    p=argparse.ArgumentParser();p.add_argument('--revision',required=True);p.add_argument('--workspace',required=True);a=p.parse_args()
    work=Path(a.workspace);work.mkdir(exist_ok=False)
    if (HERE/'linux-build-state.json').exists() or (HERE/'linux-build.json').exists():
        raise FileExistsError('Preserve and reconcile prior build records before a new attempt')
    container='msh161-policy-native-build'
    container_attempted=False
    state={'native_revision':a.revision,'started_epoch':time.time(),'status':'running','commands':[]}
    def save(): (HERE/'linux-build-state.json').write_text(json.dumps(state,indent=2)+'\n')
    def run(command, timeout=1800):
        state['commands'].append(command);save()
        with (work/'build.log').open('ab') as log:
            subprocess.run(command,stdout=log,stderr=subprocess.STDOUT,check=True,timeout=timeout)
    save()
    try:
        old=json.loads((REPO/'docs/experiments/linux-build.json').read_text())
        src=work/'source';out=work/'bin';out.mkdir()
        run(['git','clone','--no-hardlinks','--no-checkout',str(REPO),str(src)],120)
        run(['git','-C',str(src),'checkout','--detach',a.revision],120)
        assert subprocess.check_output(['git','-C',str(src),'rev-parse','HEAD'],text=True).strip()==a.revision
        assert not subprocess.check_output(['git','-C',str(src),'status','--porcelain'])
        for name in ('oxidation060','stockfish19'):
            engine=old['engines'][name];source=Path('/private/tmp/msh162-linux-bin')/Path(engine['path']).name
            assert sha(source)==engine['sha256'];shutil.copy2(source,out/source.name)
        existing=subprocess.run(['/usr/local/bin/docker','container','inspect',container],capture_output=True,timeout=30)
        if existing.returncode == 0: raise RuntimeError('Named build container already exists; reconcile before retry')
        command=['/usr/local/bin/docker','run','--rm','--platform','linux/amd64','--name','msh161-policy-native-build',
            '-v',str(src)+':/src:ro','-v','/private/tmp/msh162-linux-target:/target',
            '-v',str(out)+':/output','-e','CARGO_TARGET_DIR=/target','-e','RUSTFLAGS=-C target-cpu=x86-64',
            '-e','GIT_CONFIG_COUNT=1','-e','GIT_CONFIG_KEY_0=safe.directory','-e','GIT_CONFIG_VALUE_0=/src',old['build_image'],
            'sh','-c','set -eu; apt-get update -qq; apt-get install -y -qq git gcc g++ libc6-dev pkg-config; rustc --version; cargo --version; dpkg-query -W git gcc g++ libc6-dev pkg-config; cd /src; timeout 1200 cargo build --release --locked -j2 -p mshogi-console --bin mshogi -p mshogi-runner --bin mshogi-match; cp /target/release/mshogi /target/release/mshogi-match /output/']
        container_attempted=True
        run(command,1500)
        build=old.copy();build['engines']={k:dict(v) for k,v in old['engines'].items()};build['source']=a.revision
        for name in ('candidate','runner'):
            build['engines'][name].update(source=a.revision,sha256=sha(out/Path(build['engines'][name]['path']).name))
        (HERE/'linux-build.json').write_text(json.dumps(build,indent=2)+'\n')
        state.update(status='success',build_sha256=sha(HERE/'linux-build.json'),binaries=str(out),source_checkout=str(src))
    except BaseException as exc:
        state.update(status='failure',error=repr(exc))
        if container_attempted:
            try:
                cleanup=subprocess.run(['/usr/local/bin/docker','rm','-f',container],capture_output=True,text=True,timeout=30)
                state['container_cleanup']={'returncode':cleanup.returncode,'stdout':cleanup.stdout,'stderr':cleanup.stderr}
                check=subprocess.run(['/usr/local/bin/docker','container','inspect',container],capture_output=True,text=True,timeout=30)
                state['container_absent_after_cleanup']=check.returncode != 0 and 'No such' in check.stderr
            except Exception as cleanup_error: state['container_cleanup_error']=repr(cleanup_error)
        raise
    finally:
        state.update(finished_epoch=time.time(),log=str(work/'build.log'));save()
if __name__=='__main__':main()
