#!/usr/bin/env python3
"""Recheck classification fixtures using pinned external chess, never mShogi."""
import hashlib
import json
from pathlib import Path
import sys


def require(condition, message):
    if not condition:
        raise ValueError(message)


def check():
    import chess
    path = Path(__file__).resolve().parents[1] / "tests/fixtures/chess/classification.json"
    data = json.loads(path.read_text(encoding="utf-8"))
    require(data["format"] == "mshogi.chess.classification.v1", "format")
    require(data["schema_version"] == 1, "schema version")
    require(data["rules_profile"] == "mshogi.chess.position.v1", "profile")
    require(chess.__version__ == "1.11.2", "install chess==1.11.2 in a separate environment")
    require(hashlib.sha256(Path(chess.__file__).read_bytes()).hexdigest() == data["sources"]["python-chess-1.11.2"]["sha256"], "oracle source hash")
    names = {(False, True): "quiet", (True, True): "check", (True, False): "checkmate", (False, False): "stalemate"}
    ids = set()
    for case in data["cases"]:
        cid = case["id"]
        require(cid not in ids, f"{cid}: duplicate ID")
        ids.add(cid)
        require(case["rules_profile"] == data["rules_profile"], f"{cid}: profile")
        require(case["provenance"]["rationale"], f"{cid}: missing rationale")
        require(all(source in data["sources"] for source in case["provenance"]["sources"]), f"{cid}: sources")
        board = chess.Board(case["fen"])
        require(board.is_valid(), f"{cid}: invalid reference position")
        require(board.fen(en_passant="fen") == case["fen"], f"{cid}: non-canonical FEN")
        legal = sorted(m.uci() for m in board.legal_moves)
        require(case["legal"] == legal, f"{cid}: legal set")
        require(case["classification"] == names[(board.is_check(), bool(legal))], f"{cid}: classification")
        require(board.is_checkmate() == (case["classification"] == "checkmate"), f"{cid}: is_checkmate")
        require(board.is_stalemate() == (case["classification"] == "stalemate"), f"{cid}: is_stalemate")
        hand = case["hand_expectations"]
        require(hand["classification"] == case["classification"], f"{cid}: hand classification")
        require(hand["legal_count"] == len(legal), f"{cid}: hand legal count")
        require(set(hand["required"]) <= set(legal), f"{cid}: hand required action")
    print(f"PASS independent chess {chess.__version__}: {len(ids)} classified positions.")


if __name__ == "__main__":
    try:
        check()
    except (AssertionError, ImportError, OSError, ValueError, KeyError, TypeError) as error:
        sys.exit(f"FAIL: {error}")
