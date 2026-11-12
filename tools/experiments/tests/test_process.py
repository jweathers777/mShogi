import json
import os
import platform
import signal
import subprocess
import sys
import tempfile
import time
import unittest
from pathlib import Path
from unittest.mock import patch
from types import SimpleNamespace
from tools.experiments import cli, core
from tools.experiments.tests import test_core

REPO=Path(__file__).resolve().parents[3]
class ProcessTests(unittest.TestCase):
    def test_sigterm_reaps_detached_query_child(self):
        with tempfile.TemporaryDirectory() as name:
            root=Path(name);binary=root/'fake-engine'
            binary.write_text('#!'+sys.executable+'\nimport os,time\nfrom pathlib import Path\nPath('+repr(str(root/'child.pid'))+').write_text(str(os.getpid()))\ntime.sleep(20)\n')
            binary.chmod(0o755)
            p=subprocess.Popen([sys.executable,'-m','tools.experiments.tests.process_fixture','query',str(root)],cwd=REPO)
            try:
                deadline=time.monotonic()+5
                while not (root/'child.pid').exists() and p.poll() is None and time.monotonic()<deadline:time.sleep(.02)
                self.assertTrue((root/'child.pid').exists())
                pid=int((root/'child.pid').read_text());p.send_signal(signal.SIGTERM);self.assertEqual(p.wait(timeout=5),0)
                value=json.loads((root/'query-result.json').read_text());self.assertEqual(value['status'],'failed');self.assertTrue(value['process_group_settled'])
                with self.assertRaises(ProcessLookupError):os.kill(pid,0)
            finally:
                if p.poll() is None:p.kill();p.wait()
    def test_queue_failure_is_separate_from_success(self):
        with tempfile.TemporaryDirectory() as name:
            args=SimpleNamespace(thread='fixture',store=name)
            with patch('subprocess.run',side_effect=FileNotFoundError('no codex')):
                self.assertEqual(cli.notify(args,lambda:{'success':True}),{'success':True})
            record=core.decode(next(Path(name).glob('notifications/*.json')).read_bytes())
            self.assertEqual(record['returncode'],0);self.assertFalse(record['notification_accepted']);self.assertIn('notification_error',record)
    def test_eight_process_jobs_and_restart(self):
        fixture=test_core.CoreTests();fixture.setUp()
        try:
            roots=[];processes=[]
            for index in range(8):
                root=fixture.root/str(index);root.mkdir();job=core.seal(dict(fixture.job,experiment='parallel-'+str(index),source_files={'source':core.file_hash(fixture.source)}));(root/'source').write_bytes(fixture.source.read_bytes());(root/'job.json').write_bytes(core.encode(job));roots.append(root)
                processes.append(subprocess.Popen([sys.executable,'-m','tools.experiments.tests.process_fixture','run',str(root)],cwd=REPO))
            self.assertTrue(all(p.wait(timeout=10)==0 for p in processes))
            for root in roots:
                job=core.decode((root/'job.json').read_bytes());store=core.LocalStore(root/'store');before=core.verify_result(store,job)
                subprocess.run([sys.executable,'-m','tools.experiments.tests.process_fixture','run',str(root)],cwd=REPO,check=True,timeout=10)
                self.assertEqual(before,core.verify_result(store,job))
        finally:fixture.doCleanups()
if __name__=='__main__':unittest.main()
