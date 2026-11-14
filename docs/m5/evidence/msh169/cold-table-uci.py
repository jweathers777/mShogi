#!/usr/bin/env python3
"""UCI wrapper that makes an engine start each search with an empty table.

Measurement tool of MSH-169 (Engine Session Persistence). It sends
`ucinewgame` before each `go`, which is what the one-shot desktop engine
process did in effect. Set MSHOGI_ENGINE to the engine binary.
"""
import os
import subprocess
import sys
import threading

engine = subprocess.Popen(
    [os.environ["MSHOGI_ENGINE"], *sys.argv[1:]],
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
    text=True,
    bufsize=1,
)


def forward_output():
    for line in engine.stdout:
        sys.stdout.write(line)
        sys.stdout.flush()


threading.Thread(target=forward_output, daemon=True).start()
for line in sys.stdin:
    if line.split()[:1] == ["go"]:
        engine.stdin.write("ucinewgame\n")
    engine.stdin.write(line)
    engine.stdin.flush()
    if line.strip() == "quit":
        break
engine.stdin.close()
sys.exit(engine.wait())
