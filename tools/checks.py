#!/usr/bin/env python3
"""Shared local/CI checks. Hooks only check whitespace, syntax and formatting."""
import argparse
import json
import os
from pathlib import Path
import shlex
import shutil
import subprocess
import sys
import tempfile
import tomllib

sys.dont_write_bytecode = True
from ci_checks import affected, changed_paths, tooling_affected


def git(root, *args, input=None):
    return subprocess.run(['git', *args], cwd=root, input=input,
                          check=True, capture_output=True).stdout


def paths_from(data):
    return data.decode('utf-8', errors='surrogateescape').split('\0')[:-1]


def changed(root, base=None, staged=False):
    if staged:
        return paths_from(git(root, 'diff', '--cached', '--name-only', '--no-renames', '-z'))
    ancestor = git(root, 'merge-base', base, 'HEAD').decode().strip()
    return sorted(set(paths_from(git(root, 'diff', '--name-only', '--no-renames', '-z', ancestor))
                      + paths_from(git(root, 'ls-files', '--others', '--exclude-standard', '-z'))))


def format_commands(root, paths):
    """Use Cargo's edition/config handling, selecting packages without compiling."""
    manifests = sorted(root.glob('crates/*/Cargo.toml'))
    host = root / 'desktop/src-tauri/Cargo.toml'
    if host.exists():
        manifests.append(host)
    broad = any(p in ('Cargo.toml', 'rust-toolchain.toml', 'rust-toolchain',
                      'rustfmt.toml', '.rustfmt.toml') or p.startswith('.cargo/') for p in paths)
    commands = []
    for manifest in manifests:
        prefix = manifest.parent.relative_to(root).as_posix() + '/'
        relevant = broad or any(p.startswith(prefix) and
                               (p.endswith('.rs') or Path(p).name in
                                ('Cargo.toml', 'rustfmt.toml', '.rustfmt.toml')) for p in paths)
        if relevant:
            name = tomllib.loads(manifest.read_text())['package']['name']
            commands.append((['cargo', 'fmt', '--manifest-path',
                              manifest.relative_to(root).as_posix(), '-p', name, '--', '--check'], '.'))
    return commands


# Named suites are also used by CI. Package/GUI campaigns remain explicit.
SUITES = {
    'rust-format': [(['cargo', 'fmt', '--all', '--', '--check'], '.')],
    'rust-lint': [(['cargo', 'clippy', '--workspace', '--all-targets', '--locked', '--', '-D', 'warnings'], '.')],
    'rust-test': [(['cargo', 'test', '--workspace', '--locked'], '.')],
    'search-smoke': [(['cargo', 'test', '--release', '--locked', '-p', 'mshogi-search', '--lib'], '.')],
    'frontend': [(['npm', 'run', 'check'], 'desktop'), (['npm', 'test'], 'desktop')],
    'host': [(['npm', 'run', 'engine:prepare'], 'desktop'),
             (['cargo', 'fmt', '--all', '--', '--check'], 'desktop/src-tauri'),
             (['cargo', 'clippy', '--locked', '--all-targets', '--', '-D', 'warnings'], 'desktop/src-tauri'),
             (['cargo', 'test', '--locked'], 'desktop/src-tauri'),
             (['cargo', 'test', '--locked', '--features', 'desktop-verification'], 'desktop/src-tauri')],
    'tooling': [([sys.executable, 'tools/ci_checks.py', '--self-test'], '.'),
                ([sys.executable, '-B', '-m', 'unittest', 'discover', '-s', 'tools', '-p', 'test_checks.py', '-v'], '.')],
}


def integration_suites(paths):
    rust, frontend = affected(paths)
    suites = ['rust-format', 'rust-lint', 'rust-test'] if rust else []
    if frontend:
        suites.append('frontend')
    if any(p.startswith('desktop/src-tauri/') and not p.endswith('.md') for p in paths):
        suites.append('host')
    if tooling_affected(paths):
        suites.append('tooling')
    return suites


def python_tests(root, paths):
    """Select script tests that need no external oracle."""
    groups = [('tools/strength/', 'tools/strength/tests'),
              ('tools/experiments/', 'tools/experiments/tests'),
              ('desktop/scripts/', 'desktop/scripts')]
    commands = [([sys.executable, '-B', '-m', 'unittest', 'discover', '-s', tests, '-p', 'test_*.py'], '.')
            for prefix, tests in groups if any(p.startswith(prefix) and p.endswith('.py') for p in paths)
            and any((root / tests).glob('test_*.py'))]
    if any(p == 'tools/rules_check_helpers.py' or
           (p.startswith('tools/xiangqi/') and p.endswith('.py')) for p in paths):
        commands.append(([sys.executable, '-B', 'tools/xiangqi/test_reference.py',
                          'LocalSentinels'], '.'))
    return commands


def execute(root, commands, dry_run=False, env=None):
    for command, directory in commands:
        print(f'[{directory}] {shlex.join(command)}', flush=True)
        if not dry_run:
            # Resolve npm.cmd on Windows without using a shell on other platforms.
            command = [shutil.which(command[0]) or command[0], *command[1:]]
            subprocess.run(command, cwd=root / directory, check=True, env=env)


def syntax(root, paths, staged=False):
    for path in paths:
        if not path.endswith('.py'):
            continue
        if staged:
            entry = git(root, 'ls-files', '--stage', '--', path)
            if not entry or not entry.startswith(b'100'):
                continue  # Deleted files and symlinks are not Python source.
            source = git(root, 'show', ':' + path)
        else:
            file = root / path
            if not file.is_file() or file.is_symlink():
                continue
            source = file.read_bytes()
        compile(source, path, 'exec')  # No imports, execution or bytecode files.


def hook(root):
    paths = changed(root, staged=True)
    subprocess.run(['git', 'diff', '--cached', '--check'], cwd=root, check=True)
    syntax(root, paths, staged=True)
    if not any(p.endswith('.rs') or Path(p).name in
               ('Cargo.toml', 'rust-toolchain.toml', 'rust-toolchain', 'rustfmt.toml', '.rustfmt.toml')
               or p.startswith('.cargo/') for p in paths):
        return
    # Export only formatting inputs from the index, not working-tree content.
    # checkout-index handles linked worktrees, spaces, deletions and partial staging.
    names = paths_from(git(root, 'ls-files', '-z'))
    names = [p for p in names if p.endswith('.rs') or Path(p).name in
             ('Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml', 'rust-toolchain', 'rustfmt.toml', '.rustfmt.toml')
             or p.startswith('.cargo/')]
    with tempfile.TemporaryDirectory(prefix='mshogi-format-') as folder:
        snapshot = Path(folder)
        git(root, 'checkout-index', '--prefix=' + snapshot.as_posix() + '/', '-z', '--stdin',
            input=('\0'.join(names) + '\0').encode('utf-8', errors='surrogateescape'))
        execute(snapshot, format_commands(snapshot, paths),
                env=dict(os.environ, CARGO_NET_OFFLINE="true", RUSTUP_AUTO_INSTALL="0"))


def install(root):
    configured = subprocess.run(['git', 'config', '--get', 'core.hooksPath'], cwd=root,
                                capture_output=True, text=True)
    if configured.returncode not in (0, 1):
        raise RuntimeError('Cannot read core.hooksPath')
    if configured.stdout.strip() not in ('', '.githooks'):
        raise RuntimeError('Existing core.hooksPath is configured; integrate it explicitly before installing.')
    if not configured.stdout.strip():
        directory = Path(git(root, 'rev-parse', '--git-path', 'hooks').decode().strip())
        if not directory.is_absolute():
            directory = root / directory
        if directory.exists() and any(p.is_file() and not p.name.endswith('.sample') for p in directory.iterdir()):
            raise RuntimeError('Existing Git hooks found; integrate them explicitly before installing.')
    hook_path = root / '.githooks/pre-commit'
    hook_path.chmod(hook_path.stat().st_mode | 0o111)
    subprocess.run(['git', 'config', '--local', 'core.hooksPath', '.githooks'], cwd=root, check=True)
    print('Installed .githooks/pre-commit. No pre-push hook or automatic builds/tests.')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('mode', choices=['run', 'hook', 'install-hooks'])
    parser.add_argument('--base', default='origin/main', help='Integration diff base (merge-base plus local changes)')
    parser.add_argument('--suite', action='append', choices=SUITES, help='Explicit CI/local suite; repeatable')
    parser.add_argument('--ci', action='store_true', help='Use the GitHub event diff instead of --base')
    parser.add_argument('--scripts-only', action='store_true', help='Only Python syntax and relevant script tests')
    parser.add_argument('--dry-run', action='store_true', help='Print commands without running checks')
    args = parser.parse_args()
    if args.mode != 'run' and (args.suite or args.dry_run or args.ci or args.scripts_only):
        parser.error('Check selection options apply only to run')
    if args.suite and (args.ci or args.scripts_only):
        parser.error('Explicit --suite cannot be combined with --ci or --scripts-only')
    root = Path(git(Path.cwd(), 'rev-parse', '--show-toplevel').decode().strip())
    if args.mode == 'install-hooks':
        install(root)
    elif args.mode == 'hook':
        hook(root)
    else:
        if args.suite:
            commands = [command for suite in dict.fromkeys(args.suite) for command in SUITES[suite]]
        else:
            if args.ci:
                event = json.loads(Path(os.environ['GITHUB_EVENT_PATH']).read_text())
                paths = changed_paths(event, cwd=root)
                if paths is None:
                    paths = paths_from(git(root, 'ls-files', '-z'))
            else:
                paths = changed(root, base=args.base)
            suites = [] if args.scripts_only else integration_suites(paths)
            commands = [command for suite in suites for command in SUITES[suite]]
            commands += python_tests(root, paths)
            if not args.dry_run:
                if not args.ci:
                    subprocess.run(['git', 'diff', '--check', git(root, 'merge-base', args.base, 'HEAD').decode().strip()],
                                   cwd=root, check=True)
                syntax(root, paths)
            print('Changed-path checks: ' + (', '.join(suites) or 'no product suites'))
        execute(root, commands, args.dry_run)


if __name__ == '__main__':
    try:
        main()
    except (subprocess.CalledProcessError, OSError, ValueError, SyntaxError, RuntimeError, KeyError) as error:
        print(f'checks: {error}', file=sys.stderr)
        sys.exit(1)
