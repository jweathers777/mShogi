#!/usr/bin/env python3
"""Recheck divide fixtures using pinned external chess, never mShogi."""
import hashlib
import json
from pathlib import Path
import sys


def require(condition, message):
    if not condition:
        raise ValueError(message)


def check():
    import chess

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

    def order(uci):
        promotion = {"": 0, "q": 1, "r": 2, "b": 3, "n": 4}[uci[4:]]
        return (chess.parse_square(uci[0:2]), chess.parse_square(uci[2:4]), promotion)

    path = Path(__file__).resolve().parents[1] / "tests/fixtures/chess/divide.json"
    data = json.loads(path.read_text(encoding="utf-8"))
    require(data["format"] == "mshogi.chess.divide.v1", "format")
    require(data["schema_version"] == 1, "schema version")
    require(data["rules_profile"] == "mshogi.chess.position.v1", "profile")
    require(chess.__version__ == "1.11.2", "install chess==1.11.2 in a separate environment")
    require(hashlib.sha256(Path(chess.__file__).read_bytes()).hexdigest() == data["sources"]["python-chess-1.11.2"]["sha256"], "oracle source hash")
    ids = set()
    nodes = 0
    for case in data["cases"]:
        cid = case["id"]
        require(cid not in ids, f"{cid}: duplicate ID")
        ids.add(cid)
        require(case["rules_profile"] == data["rules_profile"], f"{cid}: profile")
        require(case["provenance"]["rationale"], f"{cid}: missing rationale")
        require(all(source in data["sources"] for source in case["provenance"]["sources"]), f"{cid}: sources")
        require(isinstance(case["depth"], int) and case["depth"] >= 1, f"{cid}: divide depth")
        require(isinstance(case["deep"], bool), f"{cid}: deep flag")
        board = chess.Board(case["fen"])
        require(board.is_valid(), f"{cid}: invalid reference position")
        require(board.fen(en_passant="fen") == case["fen"], f"{cid}: non-canonical FEN")
        actions = [entry["action"] for entry in case["divide"]]
        require(actions == sorted(actions, key=order), f"{cid}: divide order")
        require(sorted(actions) == sorted(m.uci() for m in board.legal_moves), f"{cid}: root actions")
        for entry in case["divide"]:
            board.push(chess.Move.from_uci(entry["action"]))
            require(perft(board, case["depth"] - 1) == entry["nodes"], f"{cid}: {entry['action']} count")
            board.pop()
        require(sum(entry["nodes"] for entry in case["divide"]) == case["total"], f"{cid}: total")
        nodes += case["total"]
    print(f"PASS independent chess {chess.__version__}: {len(ids)} divides, {nodes} total leaf nodes.")


if __name__ == "__main__":
    try:
        check()
    except (AssertionError, ImportError, OSError, ValueError, KeyError, TypeError) as error:
        sys.exit(f"FAIL: {error}")
