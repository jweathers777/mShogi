#!/usr/bin/env python3
"""Bounded differential check of the real `mshogi shell` binary against pinned
python-chess 1.11.2, an independently implemented chess library.

Optional research tooling: python-chess (GPL-3.0-or-later) runs only in this
separate process and is never linked into, vendored in, or shipped with mShogi;
the two programs communicate only through mshogi's documented console text.

For every documented start position it compares perft 2, then plays seeded
oracle-chosen legal playouts and compares, at every ply, the full legal action
set, the position classification and the canonical after-FEN. Agreement is
bounded evidence, not proof of full chess rules.

Examples:
  cargo build --release --locked --bin mshogi
  python tools/differential_chess.py --mshogi target/release/mshogi
  python tools/differential_chess.py --mshogi target/release/mshogi --seed 7 --playouts 20 --plies 120
  python tools/differential_chess.py --save /tmp/new-regressions.json ...
  python tools/differential_chess.py --fill tests/fixtures/chess/sequence-regressions.json
"""
import argparse
import hashlib
import json
from pathlib import Path
import random
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "tests/fixtures/chess"
PROFILE = "mshogi.chess.position.v1"
ORACLE_SHA = "1fde6a8e932508d14b31f6a584497b49c2969fc3c7cdc675347f819f0d667a5b"
U32_MAX = 4294967295


def oracle():
    import chess
    if chess.__version__ != "1.11.2":
        sys.exit("FAIL: install chess==1.11.2 in a separate environment")
    if hashlib.sha256(Path(chess.__file__).read_bytes()).hexdigest() != ORACLE_SHA:
        sys.exit("FAIL: oracle source hash differs from the recorded python-chess 1.11.2")
    return chess


def classification(board):
    names = {(False, True): "quiet", (True, True): "check", (True, False): "checkmate", (False, False): "stalemate"}
    return names[(board.is_check(), any(True for _ in board.legal_moves))]


def perft(board, depth):
    if depth == 0:
        return 1
    moves = list(board.legal_moves)
    if depth == 1:
        return len(moves)
    total = 0
    for move in moves:
        board.push(move)
        total += perft(board, depth - 1)
        board.pop()
    return total


def start_positions():
    """Every start position of the documented fixture families, deduplicated.

    Families: the corpus position cases, attacks, special actions,
    classification and ordinary candidates. Counters near the u32 limit are
    replaced by 0 1: by the owner-approved profile decision mShogi rejects such
    actions as CounterOverflow, a representation limit python-chess lacks.
    """
    fens = []
    corpus = json.loads((FIXTURES / "corpus.json").read_text(encoding="utf-8"))
    fens += [c["input"]["fen"] for c in corpus["cases"] if c["operation"] == "position"]
    for name, key in [("attacks.json", "cases"), ("special-actions.json", "positions"), ("classification.json", "cases"), ("ordinary-candidates.json", "cases")]:
        data = json.loads((FIXTURES / name).read_text(encoding="utf-8"))
        fens += [c["fen"] for c in data[key]]
    out = []
    for fen in fens:
        fields = fen.split(" ")
        if int(fields[4]) > U32_MAX - 1000 or int(fields[5]) > U32_MAX - 1000:
            fen = " ".join(fields[:4] + ["0", "1"])
        if fen not in out:
            out.append(fen)
    return out


def run_shell(mshogi, commands, timeout=600):
    """Return (stdout lines, stderr text, exit code); a timeout is exit None."""
    try:
        result = subprocess.run([mshogi, "shell"], input="\n".join(commands) + "\n", capture_output=True, text=True, timeout=timeout)
    except subprocess.TimeoutExpired as error:
        out = error.stdout.decode() if isinstance(error.stdout, bytes) else (error.stdout or "")
        return out.splitlines(), f"timed out after {timeout} s", None
    return result.stdout.splitlines(), result.stderr, result.returncode


def failure_text(stderr, code):
    text = " ".join(stderr.split())
    if len(text) > 2000:
        text = text[:2000] + " ..."
    return f"exit {code}; stderr: {text or '(empty)'}"


def expectations(chess, start_fen, actions):
    board = chess.Board(start_fen)
    after = []
    for uci in actions:
        board.push(chess.Move.from_uci(uci))
        after.append(board.fen(en_passant="fen"))
    return {"after_fens": after, "final_legal": sorted(m.uci() for m in board.legal_moves), "final_classification": classification(board)}


def compare_playout(chess, mshogi, start_fen, actions):
    """Return None on full agreement, else (failing prefix length, description).

    Every successful command prints exactly one stdout line: three for the
    start (position, legal, status) and three per ply (move, legal, status).
    The first missing or different line therefore locates the failing ply,
    whatever went wrong (a rejected action, a crash, a timeout).
    """
    commands = [f"position fen {start_fen}", "legal", "status"]
    expected = []
    board = chess.Board(start_fen)
    for ply in range(len(actions) + 1):
        if ply:
            commands += [f"move {actions[ply - 1]}", "legal", "status"]
            board.push(chess.Move.from_uci(actions[ply - 1]))
        legal = sorted(m.uci() for m in board.legal_moves)
        expected += [f"fen {board.fen(en_passant='fen')}", legal, f"status {classification(board)}"]
    stdout, stderr, code = run_shell(mshogi, commands)
    for index, want in enumerate(expected):
        ply = 0 if index < 3 else (index - 3) // 3 + 1
        if index >= len(stdout):
            return ply, f"no output for command {index + 1} ({commands[index]}): {failure_text(stderr, code)}"
        got = stdout[index]
        if isinstance(want, list):
            tokens = got.split(" ")
            if tokens[:1] != ["legal"] or sorted(tokens[2:]) != want or tokens[1:2] != [str(len(want))]:
                missing = sorted(set(want) - set(tokens[2:]))
                extra = sorted(set(tokens[2:]) - set(want))
                return ply, f"legal set at {expected[index - 1][4:]}: missing {missing}, extra {extra}"
        elif got != want:
            return ply, f"command {index + 1} ({commands[index]}): mshogi '{got}', oracle '{want}'"
    if stderr or code != 0:
        return len(actions), f"unexpected diagnostics: {failure_text(stderr, code)}"
    return None


def oracle_divide(board, depth):
    out = {}
    for move in board.legal_moves:
        board.push(move)
        out[move.uci()] = perft(board, depth - 1)
        board.pop()
    return out


def localize_perft(chess, mshogi, start_fen, depth):
    """Descend through divide to the shallowest disagreeing node.

    Returns (action prefix, description). Replaying the prefix in the Rust
    suite reaches a position whose legal set (or a child's) disagrees.
    """
    path = []
    board = chess.Board(start_fen)
    while True:
        commands = [f"position fen {start_fen}"] + ([f"move {' '.join(path)}"] if path else []) + [f"divide {depth}"]
        stdout, stderr, code = run_shell(mshogi, commands)
        rows = stdout[(2 if path else 1):]
        if code != 0 or stderr or not rows or not rows[-1].startswith("total "):
            return path, f"divide {depth} after {path}: {failure_text(stderr, code)}"
        ours = dict((row.split(" ")[0], int(row.split(" ")[1])) for row in rows[:-1])
        theirs = oracle_divide(board, depth)
        if set(ours) != set(theirs):
            return path, f"root actions at depth {depth} after {path}: missing {sorted(set(theirs) - set(ours))}, extra {sorted(set(ours) - set(theirs))}"
        differing = sorted(move for move in theirs if ours[move] != theirs[move])
        if not differing or depth == 1:
            return path, f"divide {depth} after {path}: totals differ without a localizable child"
        move = differing[0]
        path.append(move)
        board.push(chess.Move.from_uci(move))
        depth -= 1


def load_regressions(path):
    if path.exists():
        return json.loads(path.read_text(encoding="utf-8"))
    return {"format": "mshogi.chess.sequence-regressions.v1", "schema_version": 1, "rules_profile": PROFILE, "cases": []}


def write_regressions(path, data):
    path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    default = ROOT / "target/release" / ("mshogi.exe" if sys.platform == "win32" else "mshogi")
    parser.add_argument("--mshogi", default=str(default), help="path to the built mshogi binary")
    parser.add_argument("--seed", type=int, default=20261024)
    parser.add_argument("--playouts", type=int, default=3, help="playouts per start position")
    parser.add_argument("--plies", type=int, default=60, help="maximum plies per playout")
    parser.add_argument("--perft-depth", type=int, default=2)
    parser.add_argument("--save", type=Path, help="append disagreements as regression cases to this file")
    parser.add_argument("--fill", type=Path, help="add oracle expectations to cases lacking them, then exit")
    args = parser.parse_args()
    chess = oracle()

    if args.fill:
        data = load_regressions(args.fill)
        filled = 0
        for case in data["cases"]:
            if "expected" not in case:
                case["expected"] = expectations(chess, case["start_fen"], case["actions"])
                filled += 1
        write_regressions(args.fill, data)
        print(f"filled {filled} case(s) in {args.fill}")
        return 0

    rng = random.Random(args.seed)
    starts = start_positions()
    disagreements = []
    plies = 0
    for index, fen in enumerate(starts):
        stdout, stderr, code = run_shell(args.mshogi, [f"position fen {fen}", f"perft {args.perft_depth}"])
        want = f"perft {args.perft_depth} {perft(chess.Board(fen), args.perft_depth)}"
        if stderr or code != 0 or stdout[-1:] != [want]:
            path, detail = localize_perft(chess, args.mshogi, fen, args.perft_depth)
            disagreements.append({"start_fen": fen, "actions": path, "where": f"start {index} perft",
                                  "reason": f"perft: mshogi {stdout[-1:]} vs oracle '{want}'; {detail}"})
        for playout in range(args.playouts):
            board = chess.Board(fen)
            actions = []
            for _ in range(args.plies):
                moves = sorted(board.legal_moves, key=lambda m: m.uci())
                if not moves:
                    break
                move = rng.choice(moves)
                board.push(move)
                actions.append(move.uci())
            result = compare_playout(chess, args.mshogi, fen, actions)
            plies += len(actions)
            if result:
                prefix, reason = result
                disagreements.append({"start_fen": fen, "actions": actions[:prefix], "reason": reason, "where": f"start {index} playout {playout}"})
    for item in disagreements:
        print(f"DISAGREEMENT {item['where']}: {item['reason']}\n  start {item['start_fen']}\n  actions {' '.join(item['actions'])}")
    print(f"{'FAIL' if disagreements else 'PASS'} differential mshogi vs chess {chess.__version__}: seed {args.seed}, "
          f"{len(starts)} start positions (perft {args.perft_depth}), {len(starts) * args.playouts} playouts of up to {args.plies} plies, "
          f"{plies} plies compared, {len(disagreements)} disagreement(s).")
    if args.save and disagreements:
        data = load_regressions(args.save)
        seen = {(case["start_fen"], tuple(case["actions"])) for case in data["cases"]}
        ids = {case["id"] for case in data["cases"]}
        saved = 0
        for item in disagreements:
            key = (item["start_fen"], tuple(item["actions"]))
            if key in seen:
                continue
            seen.add(key)
            number = 1
            while f"differential-seed{args.seed}-{number}" in ids:
                number += 1
            ids.add(f"differential-seed{args.seed}-{number}")
            data["cases"].append({
                "id": f"differential-seed{args.seed}-{number}", "rules_profile": PROFILE,
                "start_fen": item["start_fen"], "actions": item["actions"],
                "expected": expectations(chess, item["start_fen"], item["actions"]),
                "provenance": {"source": f"tools/differential_chess.py seed {args.seed}", "reason": item["reason"]},
            })
            saved += 1
        write_regressions(args.save, data)
        print(f"saved {saved} new regression case(s) to {args.save}")
    return 1 if disagreements else 0


if __name__ == "__main__":
    sys.exit(main())
