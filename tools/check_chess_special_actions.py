#!/usr/bin/env python3
"""Recheck castling/en passant fixtures using pinned external chess, never mShogi."""
import hashlib
import json
from pathlib import Path
import sys


def require(condition, message):
    if not condition:
        raise ValueError(message)


def check():
    import chess
    path = Path(__file__).resolve().parents[1] / "tests/fixtures/chess/special-actions.json"
    data = json.loads(path.read_text(encoding="utf-8"))
    require(data["format"] == "mshogi.chess.special-actions.v1", "format")
    require(data["schema_version"] == 1, "schema version")
    require(data["rules_profile"] == "mshogi.chess.position.v1", "profile")
    require(chess.__version__ == "1.11.2", "install chess==1.11.2 in a separate environment")
    require(hashlib.sha256(Path(chess.__file__).read_bytes()).hexdigest() == data["sources"]["python-chess-1.11.2"]["sha256"], "oracle source hash")
    ids = set()

    def common(case, fen):
        cid = case["id"]
        require(cid not in ids, f"{cid}: duplicate ID")
        ids.add(cid)
        require(case["rules_profile"] == data["rules_profile"], f"{cid}: profile")
        require(case["provenance"]["rationale"], f"{cid}: missing rationale")
        require(all(source in data["sources"] for source in case["provenance"]["sources"]), f"{cid}: sources")
        board = chess.Board(fen)
        require(board.is_valid(), f"{cid}: invalid reference position")
        require(board.fen(en_passant="fen") == fen, f"{cid}: non-canonical FEN")
        return cid, board

    for case in data["positions"]:
        cid, board = common(case, case["fen"])
        special = sorted(m.uci() for m in board.legal_moves if board.is_castling(m) or board.is_en_passant(m))
        require(case["legal_special"] == special, f"{cid}: legal special set")
        require(set(special) <= set(case["special_candidates"]), f"{cid}: legal special outside candidates")
        require(case["legal"] == sorted(m.uci() for m in board.legal_moves), f"{cid}: legal set")
    steps = 0
    for case in data["sequences"]:
        cid, board = common(case, case["before_fen"])
        special = 0
        for step in case["steps"]:
            move = chess.Move.from_uci(step["action"])
            require(move in board.legal_moves, f"{cid}: illegal reference action {step['action']}")
            special += board.is_castling(move) or board.is_en_passant(move)
            board.push(move)
            require(board.fen(en_passant="fen") == step["after_fen"], f"{cid}: {step['action']} after-state")
            steps += 1
        require(special, f"{cid}: no special action")
    for case in data["rejections"]:
        cid, board = common(case, case["fen"])
        legal = chess.Move.from_uci(case["action"]) in board.legal_moves
        require(legal == case["oracle_legal"], f"{cid}: oracle legality of {case['action']}")
    print(f"PASS independent chess {chess.__version__}: {len(data['positions'])} positions, {len(data['sequences'])} sequences / {steps} steps, {len(data['rejections'])} rejections.")


if __name__ == "__main__":
    try:
        check()
    except (AssertionError, ImportError, OSError, ValueError, KeyError, TypeError) as error:
        sys.exit(f"FAIL: {error}")
