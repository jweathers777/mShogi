#!/usr/bin/env python3
"""Recheck classification fixtures using pinned external chess, never mShogi."""
from chess_check_helpers import check_case, checked_board, classification, load_fixture, require, run_check


def check():
    chess, data = load_fixture("classification")
    ids = set()
    for case in data["cases"]:
        cid = check_case(data, case, ids)
        board = checked_board(chess, case["fen"], cid)
        legal = sorted(m.uci() for m in board.legal_moves)
        require(case["legal"] == legal, f"{cid}: legal set")
        require(case["classification"] == classification(board, legal), f"{cid}: classification")
        require(board.is_checkmate() == (case["classification"] == "checkmate"), f"{cid}: is_checkmate")
        require(board.is_stalemate() == (case["classification"] == "stalemate"), f"{cid}: is_stalemate")
        hand = case["hand_expectations"]
        require(hand["classification"] == case["classification"], f"{cid}: hand classification")
        require(hand["legal_count"] == len(legal), f"{cid}: hand legal count")
        require(set(hand["required"]) <= set(legal), f"{cid}: hand required action")
    print(f"PASS independent chess {chess.__version__}: {len(ids)} classified positions.")


if __name__ == "__main__":
    run_check(check)
