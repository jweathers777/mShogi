import json,os,subprocess,sys,tempfile,unittest
from pathlib import Path
SCRIPT=Path(__file__).with_name('notify_exit.py')
class NotifyExit(unittest.TestCase):
 def run_case(self,code,queue_code=0):
  with tempfile.TemporaryDirectory() as tmp:
   d=Path(tmp);fake=d/'codex';capture=d/'captured.json'
   fake.write_text('#!'+sys.executable+'\nimport json,sys\nfrom pathlib import Path\nPath('+repr(str(capture))+').write_text(json.dumps(sys.argv[1:]))\nsys.exit('+str(queue_code)+')\n');fake.chmod(0o755)
   receipt=d/'receipt.json';command=[sys.executable,str(SCRIPT),'--thread','test-thread','--codex',str(fake),'--receipt',str(receipt),'--label','test','--state',str(d/'state.json'),'--',sys.executable,'-c',f'raise SystemExit({code})']
   p=subprocess.run(command,capture_output=True,text=True,timeout=10);r=json.loads(receipt.read_text());argv=json.loads(capture.read_text())
   self.assertEqual(argv[:3],['queue','--thread','test-thread']);self.assertEqual(argv[3],'--message')
   self.assertEqual(r['computation_returncode'],code);self.assertTrue(r['queue_attempted']);self.assertEqual(r['queue_accepted'],queue_code==0)
   self.assertIn('status='+('success' if code==0 else 'failure'),argv[4])
   before=capture.read_bytes();again=subprocess.run(command,capture_output=True,timeout=10);self.assertNotEqual(again.returncode,0);self.assertEqual(before,capture.read_bytes())
   return p.returncode
 def test_signal_queues_failure(self):
  import signal,time
  with tempfile.TemporaryDirectory() as tmp:
   d=Path(tmp);fake=d/'codex';capture=d/'captured.json'
   fake.write_text('#!'+sys.executable+'\nimport json,sys\nfrom pathlib import Path\nPath('+repr(str(capture))+').write_text(json.dumps(sys.argv[1:]))\n');fake.chmod(0o755)
   receipt=d/'receipt.json'
   command=[sys.executable,str(SCRIPT),'--thread','test-thread','--codex',str(fake),'--receipt',str(receipt),'--label','signal-test','--state',str(d/'state.json'),'--',sys.executable,'-c','import time; time.sleep(20)']
   p=subprocess.Popen(command,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
   deadline=time.monotonic()+5
   while time.monotonic()<deadline:
    if receipt.exists():
     try:
      if json.loads(receipt.read_text()).get('status')=='running':break
     except json.JSONDecodeError:pass
    time.sleep(.01)
   p.send_signal(signal.SIGTERM);stdout,stderr=p.communicate(timeout=10)
   r=json.loads(receipt.read_text());self.assertEqual(r['status'],'failure',stderr.decode());self.assertTrue(r['queue_accepted']);self.assertTrue(r['process_group_settled'] or (r.get('cleanup_error') and r.get('cleanup_review_required')),repr(r));self.assertEqual(r['signal'],signal.SIGTERM)
 def test_success_queues_once(self):self.assertEqual(self.run_case(0),0)
 def test_failure_queues_once(self):self.assertEqual(self.run_case(7),1)
 def test_queue_failure_is_visible(self):self.assertEqual(self.run_case(0,9),2)
if __name__=='__main__':unittest.main()
