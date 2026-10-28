#!/usr/bin/env python3
"""Verification-only, independently bounded real UCI0-9 admission probes.

Retain expected native fixtures separately from observed engine replies. No
engine score/outcome becomes a native result. Requires already built artifacts.
"""
import argparse
import hashlib
import json
import pathlib
import queue
import re
import subprocess
import threading
import time


class Probe:
    def __init__(self, executable):
        self.child = subprocess.Popen([str(executable)], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        self.lines = queue.Queue(maxsize=64)
        self.traffic = []
        self.errors = bytearray()
        self.observed_errors = 0
        self.origin = time.monotonic_ns()
        def read():
            while True:
                line = self.child.stdout.readline(4098)
                if not line:
                    self.lines.put(None)
                    return
                self.lines.put(line)
        def errors():
            while True:
                part = self.child.stderr.read(2048)
                if not part:
                    return
                self.observed_errors += len(part)
                self.errors.extend(part[:max(0, 4096-len(self.errors))])
        self.reader = threading.Thread(target=read, daemon=True)
        self.error_reader = threading.Thread(target=errors, daemon=True)
        self.reader.start()
        self.error_reader.start()
    def send(self, text):
        assert len(text.encode()) < 4096 and '\n' not in text
        self.child.stdin.write((text+'\n').encode('ascii'))
        self.child.stdin.flush()
        self.traffic.append(dict(at_ns=time.monotonic_ns()-self.origin, direction='command', text=text))
    def until(self, predicate, seconds=5):
        end=time.monotonic()+seconds
        while time.monotonic()<end:
            line=self.lines.get(timeout=max(.001,end-time.monotonic()))
            if line is None:
                raise RuntimeError('EOF before admission reply')
            assert len(line)<=4097, 'oversized admission output'
            text=line.decode('ascii').rstrip('\r\n')
            self.traffic.append(dict(at_ns=time.monotonic_ns()-self.origin,direction='response',text=text))
            assert len(self.traffic)<1024, 'admission traffic bound'
            if predicate(text):
                return text
        raise TimeoutError('admission deadline')
    def finish(self):
        if self.child.poll() is None:
            self.send('quit')
        try:
            self.child.wait(timeout=3)
        except subprocess.TimeoutExpired:
            self.child.kill()
            self.child.wait(timeout=3)
            raise RuntimeError('opponent did not quit; killed/reaped')
        self.child.stdin.close()
        self.reader.join(timeout=1)
        self.error_reader.join(timeout=1)
        return dict(traffic=self.traffic, exit=self.child.returncode, stderr=self.errors.decode('ascii',errors='backslashreplace'), observed_stderr_bytes=self.observed_errors)


def main():
    parser=argparse.ArgumentParser()
    for key in ['opponent','network','mshogi','fixtures','out']:
        parser.add_argument('--'+key,required=True,type=pathlib.Path)
    a=parser.parse_args()
    observed=[]
    def start():
        p=Probe(a.opponent.resolve());p.send('uci');p.until(lambda s:s=='uciok')
        for name,value in [('Threads','1'),('Hash','16'),('Ponder','false'),('MultiPV','1'),('EvalFile',str(a.network.resolve()))]:
            p.send(f'setoption name {name} value {value}')
        p.send('isready');p.until(lambda s:s=='readyok');p.send('ucinewgame');p.send('isready');p.until(lambda s:s=='readyok')
        return p
    fixtures=json.loads(a.fixtures.read_text())['fixtures']
    for fixture_id in ['XQ-F001-red','XQ-F001-black']:
        f=next(f for f in fixtures if f['id']==fixture_id)
        p=start()
        try:
            p.send('position fen '+f['fen']);p.send('go perft 1');p.until(lambda s:s.startswith('Nodes searched:'))
            moves=sorted(m.group(1) for e in p.traffic if e['direction']=='response' and (m:=re.fullmatch(r'([a-i][0-9][a-i][0-9]): 1',e['text'])))
            assert moves==sorted(f['legal_moves']), (fixture_id,moves)
            observed.append(dict(id=fixture_id,expected=dict(source=str(a.fixtures),legal_moves=f['legal_moves'],reasoning=f['reasoning']),observation=dict(legal_moves=moves),process=p.finish()))
        except BaseException:
            p.finish()
            raise
    fen='5k3/9/4R4/9/9/9/9/9/9/3K5 w - - 0 1'
    for label,position in [('standard','position startpos'),('native-P1','position fen '+fen)]:
        p=start()
        try:
            p.send(position);p.send('go depth 1 nodes 128');reply=p.until(lambda s:s.startswith('bestmove '));move=reply.split()[1]
            # Only native command admission validates this observed suggestion;
            # no comparator-generated expected move or outcome is introduced.
            native=subprocess.run([str(a.mshogi.resolve()),'--game','xiangqi','shell'],input=(position.replace('position ','position auto_valid_claims ',1)+'\nmove '+move+'\nrecord print\n').encode(),capture_output=True,timeout=10)
            assert native.returncode==0 and not native.stderr, native.stderr
            stdout=native.stdout.decode('ascii')
            assert 'played '+move in stdout.splitlines(), 'native accepted-action acknowledgement missing'
            records=[json.loads(line.removeprefix('record ')) for line in stdout.splitlines() if line.startswith('record ')]
            assert len(records)==1
            commands=records[0]['payload']['commands']
            events=records[0]['payload']['events']
            assert len(commands)==1 and commands[0]['command']==dict(kind='play',action=move) and commands[0]['result']=='accepted'
            board_events=[e for e in events if e['event']['kind']=='board_action']
            assert len(board_events)==1 and board_events[0]['event']['action']==move
            observed.append(dict(id=label,observed_reply=reply,native_acceptance=dict(exit=native.returncode,stdout=native.stdout.decode(),stderr=native.stderr.decode()),process=p.finish()))
        except BaseException:
            p.finish()
            raise
    p=start()
    try:
        p.send('position startpos');p.send('go depth 8 nodes 1000000');p.until(lambda s:s.startswith('info depth 1 '),seconds=5);p.send('stop');reply=p.until(lambda s:s.startswith('bestmove '),seconds=2)
        observed.append(dict(id='finite-work-explicit-stop',limits=dict(depth=8,nodes=1000000,ready_deadline_seconds=5,stop_reply_deadline_seconds=2),observed_reply=reply,meaning='Admission-only external stop probe; not mShogi native search limits, not a claim that the full search completed.',process=p.finish()))
    except BaseException:
        p.finish()
        raise
    output=dict(format='mshogi.xiangqi.playing-opponent-admission.v1',source='4c17cee11f888ae1d48a9494f2e2239f019f0a1f',observations=observed,network=dict(sha256=hashlib.sha256(a.network.read_bytes()).hexdigest()),meaning='Complete independent initial choices plus actual network-backed search/stop/quit and authoritative native admission, not WXF adjudication or strength evidence.')
    with a.out.open('x') as stream:
        json.dump(output,stream,indent=2);stream.write('\n')
    print('passed',len(observed),'actual opponent admission probes')


if __name__=='__main__':
    main()
