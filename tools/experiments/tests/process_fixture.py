"""Bounded execution-only child; never a scientific engine/result."""
import json
from pathlib import Path
import sys
import time
from tools.experiments.cli import install_signal_handlers
from tools.experiments import core
from tools.experiments.chess import _modules

def main():
    install_signal_handlers()
    mode,root=sys.argv[1],Path(sys.argv[2])
    if mode=='query':
        analysis=_modules(Path(__file__).resolve().parents[3])['analysis']
        decision={'initial_fen':analysis.START_FEN,'history':[],'played':'e2e4','game':0,'ply':0}
        result=analysis.query(root/'fake-engine',decision,1,None,root/'inflight.json',timeout=10)
        (root/'query-result.json').write_text(json.dumps(result))
    else:
        job=core.decode((root/'job.json').read_bytes())
        def execute(job,out,inputs,repo):
            time.sleep(.1)
            (out/'report.json.gz').write_bytes(b'process fixture')
            return {'complete':True}
        core.run_job(job,core.LocalStore(root/'store'),root,execute)
if __name__=='__main__':main()
