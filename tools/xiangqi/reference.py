#!/usr/bin/env python3
"""Strict bounded JSONL adapter for an admitted board-only Pikafish executable.

This developer tool is neither a product dependency nor a Xiangqi outcome
oracle. It does not generate expected fixture facts. No rules history API,
search, evaluation, protocol engine or NNUE network is used.
"""
import argparse
import collections
import hashlib
import json
import os
from pathlib import Path
import re
import selectors
import shlex
import signal
import subprocess
import sys
import time

PIN = '9d7833aa113626b3ea63ad83f49fb9b456bd5839'
ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / 'docs/m4/xiangqi/reference/pikafish-admission.json'
MOVE = re.compile(r'[a-i][0-9][a-i][0-9]\Z')
MAX_REQUEST = 4096
MAX_RESPONSE = 65536
MAX_DIAGNOSTIC = 4096


class ProbeFailure(ValueError):
    def __init__(self, kind, detail):
        self.kind, self.detail = kind, detail
        super().__init__(kind + ': ' + detail)


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def unique_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError('duplicate JSON field: ' + key)
        result[key] = value
    return result


def strict_json(data):
    return json.loads(data, object_pairs_hook=unique_object,
                      parse_constant=lambda text: (_ for _ in ()).throw(ValueError('nonfinite JSON: ' + text)))


def fen_domain(fen, *, root=True):
    """Independent syntax/material/geography guard; not historical reachability.

    The upstream parser additionally rejects a non-moving General already
    attacked by the moving side. Source-audited material/placement restrictions
    deliberately exclude some useful native structural diagnostics.
    """
    if not isinstance(fen, str) or not fen.isascii() or len(fen) > 200:
        raise ValueError('reference-domain: FEN must be bounded ASCII')
    fields = fen.split(' ')
    if len(fields) != 6 or fields[1] not in ('w', 'b') or fields[2:4] != ['-', '-']:
        raise ValueError('reference-domain: exact six-field FEN, w/b and literal - - required')
    if any(not re.fullmatch(r'0|[1-9][0-9]*', field) for field in fields[4:]):
        raise ValueError('reference-domain: counters must be canonical unsigned decimal')
    if int(fields[5]) < 1 or int(fields[5]) > 100000 or root and int(fields[4]) > 119:
        raise ValueError('reference-domain: declared counter/fullmove exceeds pinned parser domain')
    ranks = fields[0].split('/')
    if len(ranks) != 10:
        raise ValueError('reference-domain: ten ranks required')
    pieces = {}
    for rank, row in zip(range(9, -1, -1), ranks):
        file, previous_digit = 0, False
        for token in row:
            if token in '123456789':
                if previous_digit:
                    raise ValueError('reference-domain: noncanonical adjacent empty runs')
                file += int(token)
                previous_digit = True
            elif token in 'RACPNBKracpnbk':
                if file >= 9:
                    raise ValueError('reference-domain: rank overflow')
                pieces[(file, rank)] = token
                file += 1
                previous_digit = False
            else:
                raise ValueError('reference-domain: unknown piece or notation alias')
        if file != 9:
            raise ValueError('reference-domain: each rank has nine intersections')
    counts = collections.Counter(pieces.values())
    for side in ('upper', 'lower'):
        for kind, maximum in {'R': 2, 'A': 2, 'C': 2, 'P': 5, 'N': 2, 'B': 2, 'K': 1}.items():
            piece = kind if side == 'upper' else kind.lower()
            if counts[piece] > maximum or kind == 'K' and counts[piece] != 1:
                raise ValueError('reference-domain: invalid material count for ' + piece)
    elephant = {(2, 0), (6, 0), (0, 2), (4, 2), (8, 2), (2, 4), (6, 4)}
    advisor = {(3, 0), (5, 0), (4, 1), (3, 2), (5, 2)}
    for (file, rank), piece in pieces.items():
        red = piece.isupper()
        normalized = (file, rank if red else 9 - rank)
        if piece.upper() == 'K' and not (3 <= normalized[0] <= 5 and 0 <= normalized[1] <= 2):
            raise ValueError('reference-domain: General outside native palace')
        if piece.upper() == 'A' and normalized not in advisor:
            raise ValueError('reference-domain: Advisor on unreachable color-complex intersection')
        if piece.upper() == 'B' and normalized not in elephant:
            raise ValueError('reference-domain: Elephant on unreachable intersection')
        if piece.upper() == 'P' and (normalized[1] < 3 or normalized[1] < 5 and file % 2):
            raise ValueError('reference-domain: Soldier on unreachable intersection')
    return fields


def validate_request(request):
    if not isinstance(request, dict) or set(request) - {'root_fen', 'moves', 'perft_depth'} or 'root_fen' not in request:
        raise ValueError('request requires root_fen; only moves and perft_depth are optional')
    fen_domain(request['root_fen'])
    moves = request.get('moves', [])
    depth = request.get('perft_depth', 0)
    if not isinstance(moves, list) or len(moves) > 256 or any(not isinstance(m, str) or not MOVE.fullmatch(m) or m[:2] == m[2:] for m in moves):
        raise ValueError('reference-domain: at most256 well-formed complete actions required')
    if type(depth) is not int or not 0 <= depth <= 3:
        raise ValueError('reference-domain: perft depth must be integer0..3')
    return depth, request['root_fen'], moves


def bounded_run(command, data, timeout=20.0):
    """POSIX developer execution: bounded pipes, wall time and process cleanup."""
    if len(data) > MAX_REQUEST or not 0 < timeout <= 120:
        raise ValueError('request/time limit out of range')
    process = subprocess.Popen(command, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                               stderr=subprocess.PIPE, start_new_session=True)
    selector = selectors.DefaultSelector()
    outputs = {'stdout': bytearray(), 'stderr': bytearray()}
    deadline = time.monotonic() + timeout
    try:
        process.stdin.write(data)
        process.stdin.close()
        for stream, name in [(process.stdout, 'stdout'), (process.stderr, 'stderr')]:
            os.set_blocking(stream.fileno(), False)
            selector.register(stream, selectors.EVENT_READ, name)
        while selector.get_map():
            remaining = deadline - time.monotonic()
            if remaining <= 0:
                raise ProbeFailure('timeout', f'no complete response within {timeout:g}s')
            for key, _ in selector.select(min(remaining, 0.1)):
                chunk = os.read(key.fileobj.fileno(), 4096)
                if not chunk:
                    selector.unregister(key.fileobj)
                    continue
                name = key.data
                limit = MAX_RESPONSE if name == 'stdout' else MAX_DIAGNOSTIC
                if len(outputs[name]) + len(chunk) > limit:
                    raise ProbeFailure('output_limit', f'{name} exceeds {limit}bytes')
                outputs[name].extend(chunk)
        try:
            code = process.wait(timeout=max(0.001, deadline - time.monotonic()))
        except subprocess.TimeoutExpired as error:
            raise ProbeFailure('timeout', 'process did not terminate after response') from error
        diagnostic = outputs['stderr'].decode('utf-8', errors='replace')
        if code:
            raise ProbeFailure('process_failure', f'exit={code}; diagnostic={diagnostic}')
        if diagnostic:
            raise ProbeFailure('unexpected_diagnostic', diagnostic)
        return bytes(outputs['stdout'])
    finally:
        selector.close()
        # A leader may exit while a descendant retains the response pipes.
        # Retire the known process group even if that leader was already reaped.
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        process.wait()
        for stream in (process.stdin, process.stdout, process.stderr):
            stream.close()


def validate_response(response, depth):
    base = {'fen', 'legal_moves', 'in_check', 'undo_restored'}
    if not isinstance(response, dict) or set(response) != base | ({'perft', 'divide'} if depth else set()):
        raise ProbeFailure('malformed_response', 'wrong response fields')
    fen_domain(response['fen'], root=False)
    moves = response['legal_moves']
    if not isinstance(moves, list) or len(moves) > 128 or any(not isinstance(m, str) or not MOVE.fullmatch(m) for m in moves) or moves != sorted(set(moves)):
        raise ProbeFailure('malformed_response', 'complete move set must be unique, sorted, bounded coordinates')
    if type(response['in_check']) is not bool or response['undo_restored'] is not True:
        raise ProbeFailure('malformed_response', 'check/reversal facts absent or false')
    if depth:
        divide = response['divide']
        if not isinstance(divide, list) or len(divide) != len(moves):
            raise ProbeFailure('malformed_response', 'divide length differs from legal set')
        if any(not isinstance(entry, dict) or set(entry) != {'action', 'nodes'} or type(entry['nodes']) is not int or not 0 <= entry['nodes'] <= 128 ** depth for entry in divide):
            raise ProbeFailure('malformed_response', 'invalid bounded divide entry')
        if [entry['action'] for entry in divide] != moves or type(response['perft']) is not int or response['perft'] != sum(entry['nodes'] for entry in divide):
            raise ProbeFailure('malformed_response', 'divide actions/count disagreement')
    return response


def admitted_binary(manifest_path):
    manifest = strict_json(Path(manifest_path).read_text())
    if manifest.get('format') != 'mshogi.xiangqi.board-reference-admission.v1' or manifest.get('revision') != PIN:
        raise ValueError('unadmitted reference identity')
    if manifest.get('harness_sha256') != sha(Path(__file__).with_name('pikafish_board_probe.cpp')):
        raise ValueError('harness changed: rebuild and explicitly admit')
    binary = Path(manifest['binary_path'])
    if sha(binary) != manifest['binary_sha256']:
        raise ValueError('reference binary differs from recorded admission')
    return binary


def query(request, manifest=MANIFEST, timeout=20):
    depth, fen, moves = validate_request(request)
    data = f'{depth}\t{fen}\t{" ".join(moves)}\n'.encode('ascii')
    raw = bounded_run([str(admitted_binary(manifest))], data, timeout)
    try:
        result = strict_json(raw)
    except (ValueError, UnicodeError) as error:
        raise ProbeFailure('malformed_response', str(error)) from error
    return validate_response(result, depth)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--manifest', type=Path, default=MANIFEST)
    parser.add_argument('--timeout', type=float, default=20)
    args = parser.parse_args()
    raw = sys.stdin.buffer.read(MAX_REQUEST + 1)
    if len(raw) > MAX_REQUEST:
        raise ValueError('request byte limit')
    print(json.dumps(query(strict_json(raw), args.manifest, args.timeout), separators=(',', ':')))


if __name__ == '__main__':
    try:
        main()
    except (ValueError, OSError, KeyError, TypeError) as error:
        print('FAIL: ' + str(error), file=sys.stderr)
        sys.exit(1)
