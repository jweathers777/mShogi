"""External POSIX child-lifetime observer; never bundled with the application."""
from pathlib import Path
import json
import os
import time

if os.name != 'posix':
    raise SystemExit('POSIX PID observation only; this is not Windows runtime evidence.')

pid_path = Path('/tmp/mshogi-shutdown-child.pid')
quit_path = Path('/tmp/mshogi-quit-request-ms')
output = Path('/tmp/mshogi-shutdown-timing.json')
for path in (pid_path, quit_path, output):
    path.unlink(missing_ok=True)

deadline = time.monotonic() + 60
while not pid_path.exists():
    assert time.monotonic() < deadline, 'fixture did not start'
    time.sleep(.005)
pid = int(pid_path.read_text())
started = pid_path.stat().st_mtime_ns
last_alive = None
while True:
    try:
        os.kill(pid, 0)
    except ProcessLookupError:
        break
    last_alive = time.time_ns()
    assert time.monotonic() < deadline, 'child did not disappear'
    time.sleep(.005)

exited = time.time_ns()
quit_ns = int(quit_path.read_text()) * 1_000_000 if quit_path.exists() else None
proof = {
    'pid': pid, 'started_at_ns': started, 'last_alive_at_ns': last_alive,
    'quit_requested_at_ns': quit_ns, 'gone_at_ns': exited,
    'lifetime_ms': (exited - started) / 1e6,
    'quit_to_gone_ms': None if quit_ns is None else (exited - quit_ns) / 1e6,
}
proof['conclusive'] = (quit_ns is not None and last_alive is not None
    and started < quit_ns < exited < started + 15_000_000_000
    and last_alive >= quit_ns)
output.write_text(json.dumps(proof, indent=2) + '\n')
print(json.dumps(proof, indent=2))
assert proof['conclusive'], 'exit not distinguished from watchdog/natural timeout'
