"""Measure playing strength with native rules, reproducible inputs and bounded work."""
import argparse
import json
import os
import platform
import shutil
import subprocess
from pathlib import Path

from .common import REPO, git, utc
from tools.experiments.core import file_hash

def build_engine(args):
    rev = git('rev-parse', '--verify', args.revision + '^{commit}')
    if not rev:
        raise SystemExit('unknown revision ' + args.revision)
    root = REPO / 'target' / 'strength'
    dest = root / 'engines' / rev[:12]
    exe = dest / 'mshogi'
    if exe.exists() and not args.force:
        print(exe)
        return
    src = root / 'src' / rev[:12]
    if src.exists():
        shutil.rmtree(src)
    src.mkdir(parents=True)
    archive = subprocess.Popen(['git', '-C', str(REPO), 'archive', '--format=tar', rev], stdout=subprocess.PIPE)
    extract = subprocess.Popen(['tar', '-x', '-f', '-', '-C', str(src)], stdin=archive.stdout)
    archive.stdout.close()  # git archive receives SIGPIPE if tar stops early
    if extract.wait() != 0 or archive.wait() != 0:
        raise SystemExit('source extraction failed')
    env = dict(os.environ, CARGO_TARGET_DIR=str(root / 'build-target'))
    subprocess.run(['cargo', 'build', '--release', '--locked', '-p', 'mshogi-console', '--bin', 'mshogi'],
                   cwd=src, env=env, check=True)
    rustc = subprocess.run(['rustc', '--version'], cwd=src, capture_output=True, text=True).stdout.strip()
    dest.mkdir(parents=True, exist_ok=True)
    shutil.copy2(root / 'build-target' / 'release' / 'mshogi', exe)
    Path(str(exe) + '.identity.json').write_text(json.dumps(
        {'source_revision': rev, 'rustc': rustc, 'profile': 'release', 'built_utc': utc(),
         'sha256': file_hash(exe), 'host': platform.machine() + '-' + platform.system()}, indent=1) + '\n')
    shutil.rmtree(src)
    print(exe)


# ---------------------------------------------------------------- main

def main(argv=None):
    ap = argparse.ArgumentParser(prog='python3 -m tools.strength.cli', description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest='command', required=True)

    from . import native_match, workflow, batch
    native_match.options(sub.add_parser('match', help='paired predecessor or time-odds match'))
    native_match.options(sub.add_parser('selfplay', help='collect paired native games with one frozen engine'), selfplay=True)
    workflow.options(sub)
    fit = sub.add_parser('texel', help='run the shared fitter for a game with an evaluation model')
    fit.add_argument('--game', choices=['chess','shogi','xiangqi','tori','chu'], required=True)
    fit.add_argument('--examples', type=Path, default=Path('target/release/examples'))
    fit.add_argument('operation', choices=['fit','check','stats'])
    fit.add_argument('files', nargs='+')
    fit.set_defaults(func=lambda a: subprocess.run([(a.examples / (a.game+'_texel')).resolve(), a.operation, *a.files], check=True))
    work = sub.add_parser('tenjiku-work', help='profile Tenjiku search workloads for MSH-202')
    work.add_argument('--examples', type=Path, default=Path('target/release/examples'))
    work.add_argument('limits', nargs='*', help='depth nodes [extended]')
    work.set_defaults(func=lambda a: subprocess.run([(a.examples / 'tenjiku_search_work').resolve(), *a.limits], check=True))
    batch.options(sub)
    e = sub.add_parser('build-engine', help='build a frozen mshogi engine from a Git revision into target/strength')
    e.add_argument('revision')
    e.add_argument('--force', action='store_true')
    e.set_defaults(func=build_engine)

    args = ap.parse_args(argv)
    args.func(args)


if __name__ == '__main__':
    main()
