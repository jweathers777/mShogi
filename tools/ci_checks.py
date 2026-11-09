#!/usr/bin/env python3
"""Small CI input/path checks; product test commands stay visible in workflows."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

PLATFORMS = ("ubuntu-24.04", "macos-15", "windows-2022")
BUNDLES = dict(zip(PLATFORMS, ("deb", "app", "nsis")))
SUITES = ("configuration", "search", "process", "persistence", "full")


def platforms(value):
    if value == "all":
        return list(PLATFORMS)
    if value in PLATFORMS:
        return [value]
    raise ValueError("Unsupported platform: " + str(value))


def affected(paths):
    """Union all changed paths, including deletions and both sides of renames."""
    shared = any(p in ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "tools/ci_checks.py")
                 or p.startswith((".cargo/", ".github/workflows/")) for p in paths)
    rust = shared or any(p.startswith("crates/") for p in paths)
    frontend = shared or any(p.startswith(("desktop/", "crates/mshogi-core/",
                                           "crates/mshogi-engine/",
                                           "crates/mshogi-desktop-session/")) for p in paths)
    return rust, frontend


def changed_paths(event, cwd=None):
    base = event.get("pull_request", {}).get("base", {}).get("sha") or event.get("before")
    # New branches and unavailable old commits conservatively check both areas.
    # Native Git has no GitHub API's 300-file limit. --no-renames retains an old
    # relevant path when a file moves into a documentation directory.
    if not base or set(base) == {"0"}:
        return None
    result = subprocess.run(["git", "diff", "--name-only", "--no-renames", "-z", base, "HEAD"],
                            check=False, capture_output=True, cwd=cwd)
    if result.returncode:
        return None
    return result.stdout.decode("utf-8", errors="surrogateescape").split("\0")[:-1]


def plan(kind, event_name, event, paths=None):
    inputs = event.get("inputs", {})
    if kind == "desktop":
        if event_name != "workflow_dispatch":
            raise ValueError("Desktop packages require explicit dispatch")
        if inputs.get("operation", "configuration") not in ("configuration", "host", "package"):
            raise ValueError("Unsupported desktop operation")
        return {"matrix": {"include": [{"os": os_name, "bundle": BUNDLES[os_name]}
                                       for os_name in platforms(inputs.get("platform", PLATFORMS[0]))]}}
    if kind != "rust":
        raise ValueError("Unknown workflow kind")
    if event_name == "workflow_dispatch":
        if inputs.get("suite", "configuration") not in SUITES:
            raise ValueError("Unsupported native suite")
        return {"matrix": {"os": platforms(inputs.get("platform", PLATFORMS[0]))},
                "rust": False, "frontend": False}
    if event_name not in ("push", "pull_request"):
        raise ValueError("Unsupported automatic event")
    rust, frontend = (True, True) if paths is None else affected(paths)
    return {"matrix": {"os": [PLATFORMS[0]]}, "rust": rust, "frontend": frontend}


class PolicyTests(unittest.TestCase):
    def test_documentation_finishes_without_product_campaign(self):
        self.assertEqual(affected(["docs/m5/report.md", "backlog/tasks/msh-1.md", "AGENTS.md"]),
                         (False, False))

    def test_search_only_and_shared_consumers(self):
        self.assertEqual(affected(["crates/mshogi-search/src/chess.rs"]), (True, False))
        for prefix in ["mshogi-core", "mshogi-engine", "mshogi-desktop-session"]:
            self.assertEqual(affected([f"crates/{prefix}/src/lib.rs"]), (True, True))

    def test_mixed_changes_and_shared_configuration(self):
        self.assertEqual(affected(["crates/mshogi-search/src/lib.rs", "desktop/src/App.tsx"]),
                         (True, True))
        for name in ["Cargo.lock", "Cargo.toml", "rust-toolchain.toml", ".cargo/config.toml",
                     ".github/workflows/desktop.yml", "tools/ci_checks.py"]:
            self.assertEqual(affected([name]), (True, True))

    def test_large_diff_does_not_hide_late_source_or_rename(self):
        paths = [f"docs/notes/{i}.md" for i in range(400)]
        self.assertEqual(affected(paths + ["crates/mshogi-search/old.rs", "docs/moved.rs"]),
                         (True, False))
        self.assertEqual(affected(paths + ["desktop/src/new.ts"]), (False, True))

    def test_unknown_or_new_base_is_conservative(self):
        for event in ["push", "pull_request"]:
            self.assertTrue(plan("rust", event, {})["rust"])
            self.assertTrue(plan("rust", event, {})["frontend"])
        self.assertEqual(changed_paths({"before": "0" * 40}), None)

    def test_actual_git_push_pr_rename_and_missing_base(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            def git(*args):
                return subprocess.run(["git", *args], cwd=root, check=True,
                                      capture_output=True, text=True).stdout.strip()
            git("init", "-q")
            git("config", "user.name", "CI policy test")
            git("config", "user.email", "ci-policy@example.invalid")
            (root / "crates/mshogi-search").mkdir(parents=True)
            (root / "crates/mshogi-search/old.rs").write_text("fixture\n")
            git("add", ".")
            git("commit", "-qm", "base")
            base = git("rev-parse", "HEAD")
            (root / "docs").mkdir()
            (root / "crates/mshogi-search/old.rs").rename(root / "docs/moved.rs")
            for i in range(400):
                (root / f"docs/{i}.md").write_text("documentation\n")
            (root / "desktop/src").mkdir(parents=True)
            (root / "desktop/src/changed.ts").write_text("fixture\n")
            git("add", ".")
            git("commit", "-qm", "mixed changes")
            paths = changed_paths({"before": base}, cwd=root)
            self.assertEqual(len(paths), 403)
            self.assertIn("crates/mshogi-search/old.rs", paths)
            self.assertIn("docs/moved.rs", paths)
            self.assertEqual(affected(paths), (True, True))
            self.assertEqual(changed_paths({"pull_request": {"base": {"sha": base}},
                                            "before": "unavailable"}, cwd=root), paths)
            self.assertIsNone(changed_paths({"before": "unavailable"}, cwd=root))

    def test_all_manual_platform_and_suite_combinations(self):
        for suite in SUITES:
            for platform in [*PLATFORMS, "all"]:
                result = plan("rust", "workflow_dispatch", {"inputs": {"suite": suite, "platform": platform}})
                self.assertEqual(result["matrix"]["os"], platforms(platform))
                self.assertFalse(result["rust"] or result["frontend"])
        for operation in ["configuration", "host", "package"]:
            for platform in [*PLATFORMS, "all"]:
                result = plan("desktop", "workflow_dispatch", {"inputs": {"operation": operation, "platform": platform}})
                self.assertEqual([r["os"] for r in result["matrix"]["include"]], platforms(platform))
                self.assertTrue(all(r["bundle"] == BUNDLES[r["os"]] for r in result["matrix"]["include"]))

    def test_bad_dispatch_is_rejected_not_silently_downgraded(self):
        for kind, key, value in [("rust", "suite", "shell"), ("rust", "platform", "self-hosted"),
                                 ("desktop", "operation", "unknown"), ("desktop", "platform", '"]')]:
            with self.assertRaises(ValueError):
                plan(kind, "workflow_dispatch", {"inputs": {key: value}})
        with self.assertRaises(ValueError):
            plan("desktop", "push", {})


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--kind", choices=["rust", "desktop"])
    args = parser.parse_args()
    if args.self_test:
        result = unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(PolicyTests))
        raise SystemExit(not result.wasSuccessful())
    if not args.kind:
        parser.error("--kind is required")
    event = json.loads(Path(os.environ["GITHUB_EVENT_PATH"]).read_text())
    event_name = os.environ["GITHUB_EVENT_NAME"]
    paths = changed_paths(event) if event_name in ("push", "pull_request") else None
    outputs = plan(args.kind, event_name, event, paths)
    for key, value in outputs.items():
        print(f"{key}={json.dumps(value, separators=(',', ':'))}")


if __name__ == "__main__":
    main()
