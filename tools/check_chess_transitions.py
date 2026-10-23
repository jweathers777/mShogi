#!/usr/bin/env python3
"""Recheck ordinary before/after fixtures using pinned external chess, never mShogi."""
import hashlib
import json
from pathlib import Path
import re
import sys


def require(condition, message):
    if not condition:
        raise ValueError(message)


def check():
    import chess
    path = Path(__file__).resolve().parents[1] / "tests/fixtures/chess/ordinary-transitions.json"
    data = json.loads(path.read_text(encoding="utf-8"))
    require(data["format"] == "mshogi.chess.ordinary-transitions.v1", "format")
    require(data["schema_version"] == 1, "schema version")
    require(data["rules_profile"] == "mshogi.chess.position.v1", "profile")
    require(chess.__version__ == "1.11.2", "install chess==1.11.2 in a separate environment")
    require(hashlib.sha256(Path(chess.__file__).read_bytes()).hexdigest() == data["sources"]["python-chess-1.11.2"]["sha256"], "oracle source hash")
    ids = set()
    steps = 0
    for case in data["cases"]:
        cid = case["id"]
        require(cid not in ids, f"{cid}: duplicate ID")
        ids.add(cid)
        require(case["rules_profile"] == data["rules_profile"], f"{cid}: profile")
        require(case["steps"], f"{cid}: empty sequence")
        require(case["provenance"]["rationale"], f"{cid}: missing rationale")
        require(all(source in data["sources"] for source in case["provenance"]["sources"]), f"{cid}: sources")
        board = chess.Board(case["before_fen"])
        require(board.is_valid(), f"{cid}: invalid reference start")
        for step in case["steps"]:
            uci = step["action"]
            require(re.fullmatch(r"[a-h][1-8][a-h][1-8][qrbn]?", uci), f"{cid}: bad action")
            move = chess.Move.from_uci(uci)
            require(move in board.legal_moves, f"{cid}: illegal reference action {uci}")
            require(not board.is_castling(move) and not board.is_en_passant(move), f"{cid}: unsupported action")
            board.push(move)
            require(board.fen(en_passant="fen") == step["after_fen"], f"{cid}: {uci} after-state mismatch")
            steps += 1
    print(f"PASS independent chess {chess.__version__}: {len(ids)} ordinary sequences, {steps} expected after-states.")


if __name__ == "__main__":
    try:
        check()
    except (AssertionError, ImportError, OSError, ValueError, KeyError, TypeError) as error:
        sys.exit(f"FAIL: {error}")
