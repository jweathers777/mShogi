#!/usr/bin/env python3
"""Recheck attack fixtures using pinned external chess, never mShogi."""
from chess_check_helpers import check_case, checked_board, load_fixture, require, run_check


def check():
    chess, data = load_fixture("attacks")
    colors = {"white": chess.WHITE, "black": chess.BLACK}
    ids = set()
    queries = 0
    for case in data["cases"]:
        cid = check_case(data, case, ids)
        require(case["queries"], f"{cid}: no curated queries")
        board = checked_board(chess, case["fen"], cid)
        for query in case["queries"]:
            square = chess.parse_square(query["square"])
            actual = [chess.square_name(s) for s in board.attackers(colors[query["by"]], square)]
            require(actual == query["attackers"], f"{cid}: {query['by']} attackers of {query['square']}")
            queries += 1
        require(set(case["attacked_squares"]) == set(colors), f"{cid}: map colors")
        for name, color in colors.items():
            actual = [chess.square_name(s) for s in chess.SQUARES if board.is_attacked_by(color, s)]
            require(actual == case["attacked_squares"][name], f"{cid}: {name} attacked-square map")
    print(f"PASS independent chess {chess.__version__}: {len(ids)} attack positions, {queries} exact attacker sets, {2 * 64 * len(ids)} map queries.")


if __name__ == "__main__":
    run_check(check)
