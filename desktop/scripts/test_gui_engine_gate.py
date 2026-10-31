"""External execution gate checks, not packaged GUI acceptance or engine strength."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import unittest


class EngineGateTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp = tempfile.TemporaryDirectory(prefix='mshogi-r6-external-gate-')
        cls.gate = Path(cls.temp.name) / ('engine-gate.exe' if os.name == 'nt' else 'engine-gate')
        subprocess.run(['rustc', '--edition=2024', str(Path(__file__).with_name('verification') / 'engine-gate.rs'), '-o', str(cls.gate)], check=True, capture_output=True, timeout=60)

    @classmethod
    def tearDownClass(cls):
        cls.temp.cleanup()

    def env(self, delay):
        return {**os.environ, 'MSHOGI_R6_REAL_ENGINE': sys.executable, 'MSHOGI_R6_DELAY_MS': str(delay)}

    def test_forwards_actual_arguments_and_pipe_bytes_after_bounded_delay(self):
        code = 'import json,sys;print(json.dumps({"args":sys.argv[1:],"stdin":sys.stdin.read()}))'
        started = time.monotonic()
        result = subprocess.run([str(self.gate), '-c', code, 'complete action', '--internal'], input='exact original request\n', text=True, capture_output=True, env=self.env(100), timeout=5, check=True)
        self.assertGreaterEqual(time.monotonic() - started, .09)
        self.assertEqual(json.loads(result.stdout), {'args': ['complete action', '--internal'], 'stdin': 'exact original request\n'})

    def test_excessive_delay_is_rejected_without_launching_target(self):
        result = subprocess.run([str(self.gate), '-c', 'print("MUST NOT RUN")'], text=True, capture_output=True, env=self.env(5001), timeout=5)
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn('MUST NOT RUN', result.stdout)
        self.assertIn('exceeds external verification bound', result.stderr)

    def test_cancelling_during_gate_does_not_start_an_engine(self):
        marker = Path(self.temp.name) / 'should-not-exist'
        child = subprocess.Popen([str(self.gate), '-c', f'from pathlib import Path;Path({str(marker)!r}).write_text("bad")'], env=self.env(1500))
        try:
            time.sleep(.1)
            child.kill(); child.wait(timeout=5)
            time.sleep(.1)
            self.assertFalse(marker.exists())
        finally:
            if child.poll() is None:
                child.kill(); child.wait(timeout=5)

    def test_cancelling_after_start_retires_the_actual_child(self):
        marker = Path(self.temp.name) / 'child-started'
        code = f'import os,time;from pathlib import Path;Path({str(marker)!r}).write_text(str(os.getpid()));time.sleep(30)'
        child = subprocess.Popen([str(self.gate), '-c', code], env=self.env(0))
        try:
            deadline = time.monotonic() + 5
            while not marker.exists() and time.monotonic() < deadline:
                time.sleep(.01)
            self.assertTrue(marker.exists())
            real_pid = int(marker.read_text())
            child.kill(); child.wait(timeout=5)
            if os.name != 'nt':
                self.assertEqual(real_pid, child.pid, 'Unix gate must exec, without a separate orphanable intermediary')
            else:
                # Windows uses a kill-on-close job; query exact PID existence.
                deadline = time.monotonic() + 5
                while time.monotonic() < deadline:
                    result = subprocess.run(['tasklist', '/FI', f'PID eq {real_pid}', '/FO', 'CSV', '/NH'], text=True, capture_output=True, timeout=5)
                    if f'"{real_pid}"' not in result.stdout:
                        break
                    time.sleep(.05)
                self.assertNotIn(f'"{real_pid}"', result.stdout)
        finally:
            if child.poll() is None:
                child.kill(); child.wait(timeout=5)
