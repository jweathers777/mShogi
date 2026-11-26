"""Check policy and real Git/index behavior; never build or test product code."""
import contextlib
import io
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest
from unittest.mock import patch

import checks
from ci_checks import affected, tooling_affected

SOURCE = Path(__file__).resolve().parents[1]


class SelectionTests(unittest.TestCase):
    def test_docs_and_backlog_do_not_select_product_suites(self):
        paths = ['AGENTS.md', 'docs/development.md', 'backlog/tasks/msh-274.md',
                 'crates/mshogi-core/README.md', 'desktop/README.md']
        self.assertEqual(checks.integration_suites(paths), [])

    def test_rust_consumers_frontend_host_and_tooling(self):
        self.assertEqual(checks.integration_suites(['crates/mshogi-search/src/lib.rs']),
                         ['rust-format', 'rust-lint', 'rust-test'])
        self.assertIn('frontend', checks.integration_suites(['crates/mshogi-core/src/lib.rs']))
        self.assertEqual(checks.integration_suites(['desktop/src/App.tsx']), ['frontend'])
        self.assertEqual(checks.integration_suites(['desktop/src-tauri/src/main.rs']), ['frontend', 'host'])
        self.assertEqual(checks.integration_suites(['tools/checks.py']), ['tooling'])
        self.assertEqual(affected(['tools/ci_checks.py']), (False, False))
        self.assertTrue(tooling_affected(['.githooks/pre-commit']))

    def test_product_commands_fail_without_continuing(self):
        with patch('checks.subprocess.run', side_effect=subprocess.CalledProcessError(4, 'cargo')) as run:
            with self.assertRaises(subprocess.CalledProcessError), contextlib.redirect_stdout(io.StringIO()):
                checks.execute(SOURCE, checks.SUITES['rust-lint'] + checks.SUITES['rust-test'])
            self.assertEqual(run.call_count, 1)


class GitTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='mshogi-check-test-')
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name) / 'repo with spaces'
        self.root.mkdir()
        self.git('init', '-q')
        self.git('config', 'user.name', 'Checks test')
        self.git('config', 'user.email', 'checks@example.invalid')
        self.git('config', 'core.autocrlf', 'false')
        for path in ['tools/checks.py', 'tools/ci_checks.py', '.githooks/pre-commit', 'rust-toolchain.toml']:
            self.write(path, (SOURCE / path).read_text())
        self.write('Cargo.toml', '[workspace]\nmembers = ["crates/one", "crates/two"]\nexclude = ["desktop/src-tauri"]\nresolver = "3"\n')
        for folder, name in [('crates/one', 'one'), ('crates/two', 'two'), ('desktop/src-tauri', 'host')]:
            self.write(folder + '/Cargo.toml', f'[package]\nname = "{name}"\nversion = "0.1.0"\nedition = "2024"\n' + ('[workspace]\n' if name == 'host' else ''))
            self.write(folder + '/src/lib.rs', 'pub fn value() -> u8 {\n    1\n}\n')
        self.git('add', '.')
        self.git('commit', '-qm', 'base')
        self.base = self.git('rev-parse', 'HEAD').decode().strip()

    def git(self, *args):
        return checks.git(self.root, *args)

    def write(self, path, content):
        file = self.root / path
        file.parent.mkdir(parents=True, exist_ok=True)
        file.write_text(content)

    def test_package_selection_and_global_configuration(self):
        cmds = checks.format_commands(self.root, ['crates/one/src/lib.rs'])
        self.assertEqual(len(cmds), 1)
        self.assertIn('one', cmds[0][0])
        self.assertEqual(len(checks.format_commands(self.root, ['rust-toolchain.toml'])), 3)
        host = checks.format_commands(self.root, ['desktop/src-tauri/src/lib.rs'])
        self.assertIn('host', host[0][0])
        self.assertEqual(checks.format_commands(self.root, ['docs/note.md']), [])

    def test_changed_paths_include_deletion_rename_untracked_and_local_edits(self):
        self.git('mv', 'crates/one/src/lib.rs', 'crates/one/src/moved.rs')
        self.write('docs/new note.md', 'new\n')
        self.write('crates/two/src/lib.rs', 'pub fn other() {}\n')
        paths = checks.changed(self.root, base=self.base)
        self.assertIn('crates/one/src/lib.rs', paths)
        self.assertIn('crates/one/src/moved.rs', paths)
        self.assertIn('crates/two/src/lib.rs', paths)
        self.assertIn('docs/new note.md', paths)
        with self.assertRaises(subprocess.CalledProcessError):
            checks.changed(self.root, base='missing-ref')

    def test_python_partial_staging_uses_index_and_preserves_both_versions(self):
        self.write('example.py', 'def broken(:\n')
        self.git('add', 'example.py')
        self.write('example.py', 'answer = 42\n')
        with self.assertRaises(SyntaxError):
            checks.hook(self.root)
        self.git('add', 'example.py')
        self.write('example.py', 'def broken(:\n')
        before = self.git('write-tree')
        checks.hook(self.root)
        self.assertEqual(before, self.git('write-tree'))
        self.assertEqual((self.root / 'example.py').read_text(), 'def broken(:\n')
        self.assertFalse((self.root / '__pycache__').exists())

    def test_real_rustfmt_checks_staged_package_not_working_tree(self):
        path = 'crates/one/src/lib.rs'
        good = (self.root / path).read_text()
        self.write(path, 'pub fn value()->u8{2}\n')
        self.git('add', path)
        self.write(path, good)
        before = self.git('write-tree')
        with self.assertRaises(subprocess.CalledProcessError), contextlib.redirect_stdout(io.StringIO()):
            checks.hook(self.root)
        self.assertEqual(before, self.git('write-tree'))
        self.assertEqual((self.root / path).read_text(), good)
        self.write(path, good.replace('1', '2'))
        self.git('add', path)
        self.write(path, 'pub fn broken(\n')
        # An unrelated malformed package must not be formatted.
        self.write('crates/two/src/lib.rs', 'pub fn also_broken(\n')
        with contextlib.redirect_stdout(io.StringIO()):
            checks.hook(self.root)
        self.assertFalse((self.root / 'target').exists())

    def test_initial_commit_and_deleted_python(self):
        self.write('example.py', 'answer = 42\n')
        self.git('add', 'example.py')
        self.git('commit', '-qm', 'python')
        self.git('rm', 'example.py')
        checks.hook(self.root)
        initial = self.root / 'initial'
        initial.mkdir()
        checks.git(initial, 'init', '-q')
        (initial / 'notes.md').write_text('first commit\n')
        checks.git(initial, 'add', 'notes.md')
        checks.hook(initial)

    def test_installed_hook_blocks_bad_commit_and_keeps_checkpoint_push_free(self):
        with contextlib.redirect_stdout(io.StringIO()):
            checks.install(self.root)
            checks.install(self.root)
        self.write('notes.md', 'bad trailing space \n')
        self.git('add', 'notes.md')
        result = subprocess.run(['git', 'commit', '-qm', 'bad'], cwd=self.root, capture_output=True)
        self.assertNotEqual(result.returncode, 0)
        self.write('notes.md', 'good\n')
        self.git('add', 'notes.md')
        self.git('commit', '-qm', 'good')
        self.assertFalse((self.root / '.githooks/pre-push').exists())
        self.assertFalse((self.root / 'target').exists())
        self.assertFalse((self.root / 'desktop/node_modules').exists())

    def test_installer_preserves_existing_hook_configuration(self):
        self.git('config', 'core.hooksPath', '/custom/hooks')
        with self.assertRaises(RuntimeError):
            checks.install(self.root)
        self.assertEqual(self.git('config', '--get', 'core.hooksPath').strip(), b'/custom/hooks')
        self.git('config', '--unset', 'core.hooksPath')
        (self.root / '.git/hooks/pre-commit').write_text('#!/bin/sh\nexit 0\n')
        with self.assertRaises(RuntimeError):
            checks.install(self.root)

    def test_hook_in_linked_worktree(self):
        with contextlib.redirect_stdout(io.StringIO()):
            checks.install(self.root)
        other = Path(self.temp.name) / 'linked worktree'
        self.git('worktree', 'add', '-qb', 'linked', str(other))
        (other / '.githooks/pre-commit').chmod(0o755)
        (other / 'notes.md').write_text('linked worktree\n')
        checks.git(other, 'add', 'notes.md')
        checks.git(other, 'commit', '-qm', 'linked commit')
        self.assertEqual(checks.git(other, 'config', '--get', 'core.hooksPath').strip(), b'.githooks')


if __name__ == '__main__':
    unittest.main()
