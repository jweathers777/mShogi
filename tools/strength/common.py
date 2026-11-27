"""Shared inputs and evidence for the strength commands."""
import datetime
import gzip
import json
import subprocess
from pathlib import Path

from tools.experiments.core import environment as host_environment, file_hash

REPO = Path(__file__).resolve().parents[2]
DATA = Path(__file__).resolve().parent / 'data'
HELPER = 'target/release/examples/strength'
PROFILES = {
    'chess': 'mshogi.chess.match.v1', 'shogi': 'mshogi.shogi.csa36.v1',
    'xiangqi': 'mshogi.xiangqi.wxf2018.v1', 'tori': 'mshogi.tori.modern.v1',
    'chu': 'mshogi.chu.jcsa2019.v1',
}
TENJIKU = {'adams': 'mshogi.tenjiku.adams1999-modern.v1', 'muller': 'mshogi.tenjiku.muller-completed.v1'}


def profile(game, selection):
    if game == 'tenjiku' and selection in TENJIKU:
        return TENJIKU[selection]
    if game in PROFILES and selection is None:
        return PROFILES[game]
    raise ValueError('--profile adams|muller is required only for --game tenjiku')


def game_options(parser):
    parser.add_argument('--game', choices=[*PROFILES, 'tenjiku'], required=True)
    parser.add_argument('--profile', choices=list(TENJIKU))
    parser.add_argument('--helper', default=HELPER, help='compiled strength example')


def git(*args):
    try:
        out = subprocess.run(['git', '-C', str(REPO), *args], capture_output=True, text=True)
    except FileNotFoundError:
        return None
    return out.stdout.strip() if out.returncode == 0 else None


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat(timespec='seconds')


def environment():
    return dict(host_environment(), utc=utc(), harness_revision=git('rev-parse', 'HEAD'),
                harness_dirty=bool(git('status', '--porcelain', '--', 'tools/strength')))


def engine_identity(path):
    identity = {'path': str(path), 'sha256': file_hash(path)}
    side = Path(str(path) + '.identity.json')
    if side.exists():
        identity['build'] = json.loads(side.read_text())
    return identity


def write_new(path, value):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open('x') as stream:
        json.dump(value, stream, indent=1, allow_nan=False)
        stream.write('\n')


def load_json(path):
    path = Path(path)
    with (gzip.open(path, 'rt') if path.suffix == '.gz' else path.open()) as stream:
        return json.load(stream)
