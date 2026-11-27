#!/usr/bin/env python3
"""Recheck ordinary candidate fixtures using pinned external chess, never mShogi."""
from chess_check_helpers import check_case, checked_board, load_fixture, require, run_check


def check():
    chess, data = load_fixture("ordinary-candidates")
    ids = set()
    candidates = legal = 0
    for case in data["cases"]:
        cid = check_case(data, case, ids)
        board = checked_board(chess, case["fen"], cid)
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
    run_check(check)
