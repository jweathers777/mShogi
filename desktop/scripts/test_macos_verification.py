"""Stable QA bundle tests; no macOS application is launched or approved."""
import importlib.util
import io
from pathlib import Path
import plistlib
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location("macos_verification", Path(__file__).with_name("macos-verification.py"))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


class StableMacVerificationTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="msh121-fixed-qa-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.scope = patch.object(module, "ROOT", self.root)
        self.scope.start()
        self.addCleanup(self.scope.stop)
        source = self.root / "desktop/src-tauri/target/release/bundle/macos/mShogi Verification.app/Contents"
        (source / "MacOS").mkdir(parents=True)
        (source / "MacOS/mshogi-desktop").write_bytes(b"actual QA executable fixture")
        (source / "MacOS/mshogi").write_bytes(b"actual native engine fixture")
        (source / "Info.plist").write_bytes(plistlib.dumps({"CFBundleExecutable": "mshogi-desktop", "LSEnvironment": {"MSHOGI_DESKTOP_FIXTURE": "obsolete-per-app"}}))
        self.commands = []
        self.processes = patch.object(module.subprocess, "run", side_effect=self.command)
        self.processes.start()
        self.addCleanup(self.processes.stop)

    def command(self, args, **kwargs):
        self.commands.append(args)
        if args[0] == "ps":
            return subprocess.CompletedProcess(args, 0, stdout="", stderr="")
        if args[0] != "codesign":
            raise AssertionError(f"Unexpected process (no launches permitted): {args}")
        return subprocess.CompletedProcess(args, 0, stdout="", stderr="actual signing diagnostic fixture")

    def prepare(self, kind="qa", gate=False):
        with patch.object(module, "running", return_value=[]), patch.object(module.subprocess, "run", side_effect=self.command):
            return module.prepare(kind, gate)

    def test_fixed_three_identities_and_paths_without_fixture_suffix(self):
        self.assertEqual(module.IDENTITIES, {
            "normal": ("mShogi.app", "org.mshogi.desktop"),
            "qa": ("mShogi Verification.app", "org.mshogi.desktop.verification.r6.shogipromotion"),
            "synthetic": ("mShogi Synthetic.app", "org.mshogi.desktop.verification.r6.synthetic"),
        })
        self.assertIn("target/release/bundle/macos", str(module.bundle("normal")))
        self.assertEqual(module.bundle("qa").parent, module.bundle("synthetic").parent)

    def test_all_fixture_selections_preserve_every_signed_bundle_byte(self):
        before = self.prepare()
        original_commands = list(self.commands)
        for fixture in sorted(module.FIXTURES):
            selected = module.select("qa", fixture)
            self.assertEqual(module.selector("qa").read_bytes(), (fixture + "\n").encode())
            self.assertLessEqual(module.selector("qa").stat().st_size, 128)
            self.assertEqual(selected["fixture"], fixture)
            with patch.object(module.subprocess, "run", side_effect=self.command):
                after = module.inspect("qa")
            for field in ("path", "bundle_id", "environment", "files"):
                self.assertEqual(after[field], before[field])
        self.assertEqual([args for args in self.commands if "--sign" in args],
                         [args for args in original_commands if "--sign" in args])
        self.assertEqual(len([args for args in self.commands if "--sign" in args]), 1)
        self.assertEqual(set(before["environment"]), {"MSHOGI_DESKTOP_FIXTURE_FILE"})

    def test_synthetic_is_separate_and_normal_has_no_selector(self):
        self.prepare("synthetic")
        self.assertEqual(module.selector("synthetic").read_text(), "synthetic\n")
        for kind, fixture in (("synthetic", "standard"), ("qa", "synthetic"), ("normal", "standard")):
            with self.assertRaises(ValueError):
                module.select(kind, fixture)

    def test_bad_tokens_leave_previous_selector_and_bundle_unchanged(self):
        before = self.prepare()
        for token in ("unknown", "standard\n", "../claim", "claim\x00", "x" * 129, "標準"):
            with self.assertRaises(ValueError):
                module.select("qa", token)
            self.assertEqual(module.selector("qa").read_bytes(), b"standard\n")
        with patch.object(module.subprocess, "run", side_effect=self.command):
            self.assertEqual(module.inspect("qa")["files"], before["files"])

    def test_interrupted_atomic_replace_preserves_last_complete_selector(self):
        self.prepare()
        with patch.object(module.os, "replace", side_effect=OSError("interrupted")):
            with self.assertRaisesRegex(OSError, "interrupted"):
                module.select("qa", "claim")
        self.assertEqual(module.selector("qa").read_bytes(), b"standard\n")
        self.assertEqual(list(module.directory().glob(".qa-fixture-*")), [])

    def test_prepare_refuses_running_owned_bundle_before_copy_or_sign(self):
        with patch.object(module, "running", return_value=["123"]), patch.object(module.shutil, "copytree") as copy, patch.object(module.subprocess, "run") as run:
            with self.assertRaisesRegex(RuntimeError, "Close.*natively"):
                module.prepare("qa")
            copy.assert_not_called()
            run.assert_not_called()

    def test_select_refuses_running_bundle_without_changing_selector_or_app(self):
        before = self.prepare()
        with patch.object(module, "running", return_value=["123"]):
            with self.assertRaisesRegex(RuntimeError, "Close.*natively"):
                module.select("qa", "claim")
        self.assertEqual(module.selector("qa").read_bytes(), b"standard\n")
        self.assertEqual(module.inspect("qa")["files"], before["files"])

    def test_inspect_cli_rejects_wrong_identity_even_when_signature_verifies(self):
        self.prepare()
        plist = module.bundle("qa") / "Contents/Info.plist"
        info = plistlib.loads(plist.read_bytes())
        info["CFBundleIdentifier"] = "unrelated.app.identity"
        plist.write_bytes(plistlib.dumps(info))
        with patch.object(sys, "argv", ["macos-verification.py", "inspect", "qa"]), patch("sys.stdout", new_callable=io.StringIO) as output:
            with self.assertRaises(SystemExit) as failure:
                module.main()
        self.assertEqual(failure.exception.code, 1)
        self.assertIn("unrelated.app.identity", output.getvalue())

    def test_process_detection_matches_exact_executable_not_other_names(self):
        expected = str(module.bundle("qa") / "Contents/MacOS/mshogi-desktop")
        output = f"42 {expected}\n43 /other/mShogi Verification.app/Contents/MacOS/mshogi-desktop\n"
        with patch.object(module.subprocess, "run", return_value=subprocess.CompletedProcess([], 0, output, "")):
            self.assertEqual(module.running(module.bundle("qa")), ["42"])

    def test_failed_signing_keeps_existing_valid_staged_app(self):
        before = self.prepare()
        with patch.object(module, "running", return_value=[]), patch.object(module.subprocess, "run", side_effect=subprocess.CalledProcessError(1, "codesign")):
            with self.assertRaises(subprocess.CalledProcessError):
                module.prepare("qa")
        with patch.object(module.subprocess, "run", side_effect=self.command):
            self.assertEqual(module.inspect("qa")["files"], before["files"])

    def test_gate_is_explicit_qa_only_and_external_to_signed_app(self):
        with self.assertRaisesRegex(RuntimeError, "Compile.*gate"):
            self.prepare(gate=True)
        module.directory().mkdir(parents=True, exist_ok=True)
        gate = module.directory() / "r6-engine-gate"
        gate.write_bytes(b"external gate")
        gate.chmod(0o700)
        info = self.prepare(gate=True)
        self.assertEqual(info["environment"]["MSHOGI_VERIFICATION_ENGINE"], str(gate))
        self.assertEqual(info["environment"]["MSHOGI_R6_DELAY_MS"], "5000")
        self.assertNotIn("MSHOGI_DESKTOP_FIXTURE", info["environment"])
        with self.assertRaises(ValueError):
            self.prepare("synthetic", gate=True)


if __name__ == "__main__":
    unittest.main()
