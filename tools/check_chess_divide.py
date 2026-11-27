#!/usr/bin/env python3
"""Recheck divide fixtures using pinned external chess, never mShogi."""
from chess_check_helpers import check_case, checked_board, load_fixture, perft, require, run_check


def check():
    chess, data = load_fixture("divide")

    def order(uci):
        promotion = {"": 0, "q": 1, "r": 2, "b": 3, "n": 4}[uci[4:]]
        return (chess.parse_square(uci[0:2]), chess.parse_square(uci[2:4]), promotion)

    ids = set()
    nodes = 0
    for case in data["cases"]:
        cid = check_case(data, case, ids)
        require(isinstance(case["depth"], int) and case["depth"] >= 1, f"{cid}: divide depth")
        require(isinstance(case["deep"], bool), f"{cid}: deep flag")
        board = checked_board(chess, case["fen"], cid)
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
    run_check(check)
