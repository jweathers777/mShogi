"""Shared checks for pinned, developer-only Chess verification."""
import json
from pathlib import Path
import re
import sys

from rules_check_helpers import sha

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "tests/fixtures/chess"
PROFILE = "mshogi.chess.position.v1"
MOVE = re.compile(r"[a-h][1-8][a-h][1-8][qrbn]?")
ORACLE_VERSION = "1.11.2"


def require(condition, message):
    if not condition:
        raise ValueError(message)


def read_fixture(name):
    return json.loads((FIXTURES / name).read_text(encoding="utf-8"))


def load_oracle(sources):
    import chess

    require(chess.__version__ == ORACLE_VERSION,
            f"install chess=={ORACLE_VERSION} in a separate environment")
    require(sha(chess.__file__) == sources[f"python-chess-{ORACLE_VERSION}"]["sha256"],
            "oracle source hash")
    return chess


def load_fixture(name):
    data = read_fixture(f"{name}.json")
    require(data["format"] == f"mshogi.chess.{name}.v1", "format")
    require(data["schema_version"] == 1, "schema version")
    require(data["rules_profile"] == PROFILE, "profile")
    return load_oracle(data["sources"]), data


def check_case(data, case, ids):
    cid = case["id"]
    require(cid not in ids, f"{cid}: duplicate ID")
    ids.add(cid)
    require(case["rules_profile"] == data["rules_profile"], f"{cid}: profile")
    require(case["provenance"]["rationale"], f"{cid}: missing rationale")
    require(all(source in data["sources"] for source in case["provenance"]["sources"]),
            f"{cid}: sources")
    return cid


def checked_board(chess, fen, cid):
    board = chess.Board(fen)
    require(board.is_valid(), f"{cid}: invalid reference position")
    require(board.fen(en_passant="fen") == fen, f"{cid}: non-canonical FEN")
    return board


def classification(board, legal_moves):
    names = {(False, True): "quiet", (True, True): "check",
             (True, False): "checkmate", (False, False): "stalemate"}
    return names[board.is_check(), bool(legal_moves)]


def perft(board, depth):
    if depth == 0:
        return 1
    moves = list(board.legal_moves)
    if depth == 1:
        return len(moves)
    total = 0
    for move in moves:
        board.push(move)
        total += perft(board, depth - 1)
        board.pop()
    return total


def run_check(check):
    try:
        sys.exit(check())
    except (AssertionError, ImportError, OSError, ValueError, KeyError, TypeError) as error:
        sys.exit(f"FAIL: {error}")
