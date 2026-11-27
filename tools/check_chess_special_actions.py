#!/usr/bin/env python3
"""Recheck castling/en passant fixtures using pinned external chess, never mShogi."""
from chess_check_helpers import check_case, checked_board, load_fixture, require, run_check


def check():
    chess, data = load_fixture("special-actions")
    ids = set()

    def common(case, fen):
        cid = check_case(data, case, ids)
        return cid, checked_board(chess, fen, cid)

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
    run_check(check)
