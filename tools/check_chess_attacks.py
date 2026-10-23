#!/usr/bin/env python3
"""Recheck attack fixtures using pinned external chess, never mShogi."""
import hashlib
import json
from pathlib import Path
import sys


def require(condition, message):
    if not condition:
        raise ValueError(message)


def check():
    import chess
    path = Path(__file__).resolve().parents[1] / "tests/fixtures/chess/attacks.json"
    data = json.loads(path.read_text(encoding="utf-8"))
    require(data["format"] == "mshogi.chess.attacks.v1", "format")
    require(data["schema_version"] == 1, "schema version")
    require(data["rules_profile"] == "mshogi.chess.position.v1", "profile")
    require(chess.__version__ == "1.11.2", "install chess==1.11.2 in a separate environment")
    require(hashlib.sha256(Path(chess.__file__).read_bytes()).hexdigest() == data["sources"]["python-chess-1.11.2"]["sha256"], "oracle source hash")
    colors = {"white": chess.WHITE, "black": chess.BLACK}
    ids = set()
    queries = 0
    for case in data["cases"]:
        cid = case["id"]
        require(cid not in ids, f"{cid}: duplicate ID")
        ids.add(cid)
        require(case["rules_profile"] == data["rules_profile"], f"{cid}: profile")
        require(case["queries"], f"{cid}: no curated queries")
        require(case["provenance"]["rationale"], f"{cid}: missing rationale")
        require(all(source in data["sources"] for source in case["provenance"]["sources"]), f"{cid}: sources")
        board = chess.Board(case["fen"])
        require(board.is_valid(), f"{cid}: invalid reference position")
        require(board.fen(en_passant="fen") == case["fen"], f"{cid}: non-canonical FEN")
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
    try:
        check()
    except (AssertionError, ImportError, OSError, ValueError, KeyError, TypeError) as error:
        sys.exit(f"FAIL: {error}")
