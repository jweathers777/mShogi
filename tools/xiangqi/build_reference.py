#!/usr/bin/env python3
"""Build an isolated, network-free, source-pinned developer board comparator."""
import argparse
import hashlib
import json
from pathlib import Path
import platform
import shlex
import subprocess
import sys

PIN = '9d7833aa113626b3ea63ad83f49fb9b456bd5839'
ROOT = Path(__file__).resolve().parents[2]
HARNESS = Path(__file__).with_name('pikafish_board_probe.cpp')


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def build(source, output, compiler):
    source = source.resolve()
    if subprocess.check_output(['git', '-C', str(source), 'rev-parse', 'HEAD'], text=True).strip() != PIN:
        raise ValueError('wrong reference pin')
    if subprocess.check_output(['git', '-C', str(source), 'status', '--porcelain', '--untracked-files=no'], text=True):
        raise ValueError('tracked source differs from audited pin')
    output.mkdir(parents=True, exist_ok=False)
    commands = []
    log = output / 'build.log'

    def run(args, **kwargs):
        commands.append(args)
        with log.open('ab') as stream:
            stream.write((shlex.join(args) + '\n').encode())
            subprocess.run(args, stdout=stream, stderr=stream, check=True, **kwargs)

    isolated = output / 'source'
    run(['git', 'clone', '--shared', '--no-hardlinks', str(source), str(isolated)])
    # This add-only patch is the entire change to the audited upstream tree.
    body = HARNESS.read_text()
    patch = output / 'harness.patch'
    patch.write_text('diff --git a/src/mshogi_board_probe.cpp b/src/mshogi_board_probe.cpp\n'
                     'new file mode 100644\n--- /dev/null\n+++ b/src/mshogi_board_probe.cpp\n'
                     f'@@ -0,0 +1,{len(body.splitlines())} @@\n'
                     + ''.join('+' + line + '\n' for line in body.splitlines()))
    run(['git', '-C', str(isolated), 'apply', '--check', str(patch)])
    run(['git', '-C', str(isolated), 'apply', str(patch)])
    binary = output / 'pikafish-board-probe'
    files = ['src/mshogi_board_probe.cpp', 'src/position.cpp', 'src/movegen.cpp',
             'src/attacks.cpp', 'src/bitboard.cpp', 'src/nnue/features/k6r2nc2.cpp']
    flags = ['-std=c++17', '-O2', '-g', '-DIS_64BIT', '-ffunction-sections', '-fdata-sections', '-pthread']
    flags += ['-Wl,-dead_strip'] if sys.platform == 'darwin' else ['-Wl,--gc-sections']
    command = [compiler, *flags, '-I' + str(isolated / 'src'),
               *(str(isolated / name) for name in files), '-o', str(binary)]
    run(command)
    upstream_files = sorted(path for path in (isolated / 'src').rglob('*')
                            if path.is_file() and path.name != 'mshogi_board_probe.cpp')
    manifest = {
        'format': 'mshogi.xiangqi.board-reference-admission.v1',
        'upstream': 'https://github.com/official-pikafish/Pikafish',
        'revision': PIN,
        'tree': subprocess.check_output(['git', '-C', str(isolated), 'rev-parse', 'HEAD^{tree}'], text=True).strip(),
        'clean_tracked_base': True,
        'platform': platform.platform(), 'machine': platform.machine(),
        'compiler': subprocess.check_output([compiler, '--version'], text=True).strip(),
        'commands': commands, 'build_log_sha256': sha(log),
        'patch_sha256': sha(patch), 'harness_sha256': sha(HARNESS),
        'source_sha256': {str(path.relative_to(isolated)): sha(path) for path in upstream_files},
        'license_sha256': sha(isolated / 'Copying.txt'),
        'license': 'GPL-3.0-or-later source/harness; isolated developer executable only; no product dependency or distributed binary',
        'binary_path': str(binary), 'binary_sha256': sha(binary),
        'options': {'assertions': True, 'NNUE': 'no network acquired, loaded or evaluated', 'search': False, 'threads': 1},
        'operation': 'Position::set, MoveList<ALL> (EVASIONS under check), do_move/undo_move; perft using same complete list',
        'dialect': 'six-field FEN RACPNBK/racpnbk, rank9 to rank0; w=Red, b=Black; a0-i9 complete board moves; literal - - fields',
        'exclusions': ['rule_judge', 'repetition/chase', 'natural-count adjudication', 'game outcomes',
                       'NNUE/search/evaluation/strength', 'royal capture/missing kings', 'unreachable-square structural diagrams'],
        'interpretation': 'board comparator only, not rules authority; native fixture expectations must remain independently established',
    }
    (output / 'admission.json').write_text(json.dumps(manifest, indent=2) + '\n')
    print(json.dumps({'binary': str(binary), 'manifest': str(output / 'admission.json')}))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--compiler', default='clang++')
    args = parser.parse_args()
    try:
        build(args.source, args.output, args.compiler)
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        print('BUILD FAILED: ' + str(error), file=sys.stderr)
        sys.exit(1)
