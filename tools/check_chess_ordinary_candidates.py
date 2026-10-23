#!/usr/bin/env python3
"""Recheck ordinary candidate fixtures using pinned external chess, never mShogi."""
import hashlib
import json
from pathlib import Path
import sys


def require(condition, message):
    if not condition:
        raise ValueError(message)


def check():
    import chess
    path = Path(__file__).resolve().parents[1] / "tests/fixtures/chess/ordinary-candidates.json"
    data = json.loads(path.read_text(encoding="utf-8"))
    require(data["format"] == "mshogi.chess.ordinary-candidates.v1", "format")
    require(data["schema_version"] == 1, "schema version")
    require(data["rules_profile"] == "mshogi.chess.position.v1", "profile")
    require(chess.__version__ == "1.11.2", "install chess==1.11.2 in a separate environment")
    require(hashlib.sha256(Path(chess.__file__).read_bytes()).hexdigest() == data["sources"]["python-chess-1.11.2"]["sha256"], "oracle source hash")
    ids = set()
    candidates = legal = 0
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
        ordinary = lambda moves: sorted({m.uci() for m in moves if not board.is_castling(m) and not board.is_en_passant(m)})
        require(case["candidates"] == ordinary(board.generate_pseudo_legal_moves()), f"{cid}: candidate set")
        require(case["legal_ordinary"] == ordinary(board.legal_moves), f"{cid}: legal ordinary set")
        hand = case["hand_expectations"]
        require(len(case["candidates"]) == hand["candidate_count"], f"{cid}: hand candidate count")
        require(len(case["legal_ordinary"]) == hand["legal_ordinary_count"], f"{cid}: hand legal count")
        require(set(hand["required"]) <= set(case["candidates"]), f"{cid}: hand required action")
        require(not set(hand["excluded"]) & set(case["candidates"]), f"{cid}: hand excluded action")
        candidates += len(case["candidates"])
        legal += len(case["legal_ordinary"])
    print(f"PASS independent chess {chess.__version__}: {len(ids)} positions, {candidates} exact candidates, {legal} legal ordinary actions.")

if __name__ == "__main__":
    try:
        check()
    except (AssertionError, ImportError, OSError, ValueError, KeyError, TypeError) as error:
        sys.exit(f"FAIL: {error}")
