"""Minimal bounded UCI client for the bench and blunder parts. No game rules here."""
import os
import queue
import subprocess
import threading
import time


class UciError(RuntimeError):
    pass


def parse_info(line):
    """Fields of one UCI info line: depth, seldepth, nodes, time, score, pv."""
    tokens = line.split()
    out = {}
    for key in ('depth', 'seldepth', 'nodes', 'time', 'nps'):
        if key in tokens:
            out[key] = int(tokens[tokens.index(key) + 1])
    if 'score' in tokens:
        i = tokens.index('score')
        out['score'] = {'kind': tokens[i + 1], 'value': int(tokens[i + 2])}
    if 'pv' in tokens:
        out['pv'] = tokens[tokens.index('pv') + 1:]
    return out


def position_command(fen, moves):
    base = 'position startpos' if fen in (None, 'startpos') else 'position fen ' + fen
    return base + (' moves ' + ' '.join(moves) if moves else '')


class Engine:
    """One engine process. Reads stdout on a thread so every wait has a deadline."""

    def __init__(self, path, args=(), options=(), timeout=30.0):
        self.path = path
        self.proc = subprocess.Popen([path, *args], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                     stderr=subprocess.DEVNULL, text=True, bufsize=1)
        self.lines = queue.Queue()
        threading.Thread(target=self._pump, daemon=True).start()
        self.send('uci')
        self.identity = [l for l in self.wait_for('uciok', timeout) if l.startswith('id ')]
        for name, value in options:
            self.send('setoption name %s value %s' % (name, value))
        self.ready(timeout)

    def _pump(self):
        for line in self.proc.stdout:
            self.lines.put(line.rstrip('\n'))
        self.lines.put(None)

    def send(self, text):
        self.proc.stdin.write(text + '\n')
        self.proc.stdin.flush()

    def wait_for(self, prefix, timeout):
        deadline = time.monotonic() + timeout
        seen = []
        while True:
            remaining = deadline - time.monotonic()
            if remaining <= 0:
                raise UciError('timeout waiting for %r' % prefix)
            try:
                line = self.lines.get(timeout=remaining)
            except queue.Empty:
                continue
            if line is None:
                raise UciError('engine exited waiting for %r' % prefix)
            seen.append(line)
            if line.split(' ', 1)[0] == prefix:
                return seen

    def ready(self, timeout=30.0):
        self.send('isready')
        self.wait_for('readyok', timeout)

    def search(self, fen, moves, go, timeout):
        """Run one search; return bestmove, last info fields and driver wall seconds."""
        self.send('ucinewgame')
        self.send(position_command(fen, moves))
        self.ready()
        started = time.monotonic()
        self.send('go ' + go)
        lines = self.wait_for('bestmove', timeout)
        wall = time.monotonic() - started
        info = {}
        for line in lines:
            if line.startswith('info ') and ' depth ' in ' ' + line:
                info = parse_info(line)
        return {'bestmove': lines[-1].split()[1], 'info': info, 'wall_seconds': wall}

    def close(self):
        try:
            self.send('quit')
            self.proc.wait(timeout=5)
        except (OSError, subprocess.TimeoutExpired):
            self.proc.kill()
            self.proc.wait()


def file_sha256(path):
    import hashlib
    h = hashlib.sha256()
    with open(path, 'rb') as stream:
        for block in iter(lambda: stream.read(1 << 20), b''):
            h.update(block)
    return h.hexdigest()


def default_workers():
    """Performance cores on Apple silicon, else physical/logical cores, less one."""
    try:
        out = subprocess.run(['sysctl', '-n', 'hw.perflevel0.physicalcpu'], capture_output=True, text=True, timeout=5)
        if out.returncode == 0 and out.stdout.strip().isdigit():
            return max(1, int(out.stdout.strip()))
    except (OSError, subprocess.TimeoutExpired):
        pass
    return max(1, (os.cpu_count() or 2) - 1)
