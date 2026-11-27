#!/usr/bin/env python3
"""Recheck ordinary before/after fixtures using pinned external chess, never mShogi."""
from chess_check_helpers import MOVE, check_case, load_fixture, require, run_check


def check():
    chess, data = load_fixture("ordinary-transitions")
    ids = set()
    steps = 0
    for case in data["cases"]:
        cid = check_case(data, case, ids)
        require(case["steps"], f"{cid}: empty sequence")
        board = chess.Board(case["before_fen"])
        require(board.is_valid(), f"{cid}: invalid reference start")
        for step in case["steps"]:
            uci = step["action"]
            require(MOVE.fullmatch(uci), f"{cid}: bad action")
            move = chess.Move.from_uci(uci)
            require(move in board.legal_moves, f"{cid}: illegal reference action {uci}")
            require(not board.is_castling(move) and not board.is_en_passant(move), f"{cid}: unsupported action")
            board.push(move)
            require(board.fen(en_passant="fen") == step["after_fen"], f"{cid}: {uci} after-state mismatch")
            steps += 1
    print(f"PASS independent chess {chess.__version__}: {len(ids)} ordinary sequences, {steps} expected after-states.")


if __name__ == "__main__":
    run_check(check)
