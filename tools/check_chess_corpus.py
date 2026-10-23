#!/usr/bin/env python3
"""Validate MSH-2 fixture data; optionally recheck it with an external oracle.

This tool never imports mShogi and does not implement its future FEN parser.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re
import sys

PROFILE = "mshogi.chess.position.v1"
CORPUS = Path(__file__).resolve().parents[1] / "tests/fixtures/chess/corpus.json"
MOVE = re.compile(r"[a-h][1-8][a-h][1-8][qrbn]?")


def require(condition, message):
    if not condition:
        raise ValueError(message)


def validate(data):
    require(set(data) == {"schema_version", "rules_profile", "sources", "cases"}, "corpus fields")
    require(type(data["schema_version"]) is int and data["schema_version"] == 1, "schema version")
    require(data["rules_profile"] == PROFILE, "corpus profile")
    require(isinstance(data["sources"], dict) and data["sources"], "source registry")
    for source, record in data["sources"].items():
        require(all(isinstance(record.get(k), str) and record[k] for k in ("url", "version", "retrieved")), f"source {source}")
    require(isinstance(data["cases"], list) and data["cases"], "nonempty cases")
    ids = set()
    for case in data["cases"]:
        cid = case.get("id", "<missing>")
        require(set(case) == {"id", "rules_profile", "tags", "input", "operation", "expected", "provenance"}, f"{cid}: fields")
        require(isinstance(cid, str) and re.fullmatch(r"[a-z][a-z0-9-]*", cid) and cid not in ids, f"{cid}: unique ID")
        ids.add(cid)
        require(case["rules_profile"] == PROFILE, f"{cid}: profile")
        require(isinstance(case["tags"], list) and case["tags"] and all(isinstance(t, str) and t for t in case["tags"]), f"{cid}: tags")
        require(set(case["input"]) == {"fen"} and isinstance(case["input"]["fen"], str), f"{cid}: input")
        provenance = case["provenance"]
        require(set(provenance) == {"sources", "locator", "method", "rationale"}, f"{cid}: provenance fields")
        require(isinstance(provenance["sources"], list) and provenance["sources"] and all(s in data["sources"] for s in provenance["sources"]), f"{cid}: sources")
        require(all(isinstance(provenance[k], str) and provenance[k] for k in ("locator", "method", "rationale")), f"{cid}: provenance text")
        expected = case["expected"]
        require(isinstance(expected, dict), f"{cid}: expected object")
        if case["operation"] == "position":
            require({"legal_moves", "classification"} <= set(expected) <= {"legal_moves", "classification", "perft", "attacks"}, f"{cid}: position expectations")
            moves = expected["legal_moves"]
            require(isinstance(moves, list) and all(isinstance(m, str) and MOVE.fullmatch(m) for m in moves), f"{cid}: move notation")
            require(moves == sorted(set(moves)), f"{cid}: sorted unique complete move set")
            require(expected["classification"] in ("quiet", "check", "checkmate", "stalemate"), f"{cid}: classification")
            if "perft" in expected:
                counts = expected["perft"]
                require(isinstance(counts, dict) and counts and set(counts) <= {"0", "1", "2", "3"}, f"{cid}: bounded perft depths 0..3")
                require(all(type(n) is int and n >= 0 for n in counts.values()), f"{cid}: perft counts")
            if "attacks" in expected:
                require(isinstance(expected["attacks"], list), f"{cid}: attacks")
                for attack in expected["attacks"]:
                    require(set(attack) == {"by", "square", "attacked"}, f"{cid}: attack fields")
                    require(attack["by"] in ("white", "black") and isinstance(attack["square"], str) and re.fullmatch(r"[a-h][1-8]", attack["square"]) and type(attack["attacked"]) is bool, f"{cid}: attack query")
        elif case["operation"] == "parse_fen":
            require(type(expected.get("accepted")) is bool, f"{cid}: acceptance flag")
            if expected["accepted"]:
                require(set(expected) == {"accepted", "canonical_fen"} and isinstance(expected["canonical_fen"], str), f"{cid}: canonical FEN")
            else:
                require(set(expected) == {"accepted", "error"} and expected["error"] in ("syntax", "range", "structure"), f"{cid}: rejection category")
        else:
            raise ValueError(f"{cid}: unknown operation")


def perft(board, depth):
    if depth == 0:
        return 1
    if depth == 1:
        return board.legal_moves.count()
    nodes = 0
    for move in list(board.legal_moves):
        board.push(move)
        nodes += perft(board, depth - 1)
        board.pop()
    return nodes


def verify_oracle(data):
    try:
        import chess
    except ImportError as error:
        raise ValueError("--oracle needs a separate environment with chess==1.11.2; see corpus README") from error
    require(chess.__version__ == "1.11.2", "oracle must be chess==1.11.2")
    digest = hashlib.sha256(Path(chess.__file__).read_bytes()).hexdigest()
    require(digest == data["sources"]["python-chess-1.11.2"]["sha256"], "oracle module hash mismatch")
    checked, rejected = 0, 0
    for case in data["cases"]:
        cid, expected = case["id"], case["expected"]
        if case["operation"] == "parse_fen" and not expected["accepted"]:
            rejected += 1
            continue  # Project parser contracts, not the external library's input policy.
        board = chess.Board(" ".join(case["input"]["fen"].split()))
        if case["operation"] == "parse_fen":
            require(board.fen(en_passant="fen") == expected["canonical_fen"], f"{cid}: canonical FEN mismatch")
        else:
            moves = sorted(move.uci() for move in board.legal_moves)
            require(moves == expected["legal_moves"], f"{cid}: move set mismatch; missing={sorted(set(expected['legal_moves']) - set(moves))}, extra={sorted(set(moves) - set(expected['legal_moves']))}")
            classification = "checkmate" if board.is_checkmate() else "stalemate" if board.is_stalemate() else "check" if board.is_check() else "quiet"
            require(classification == expected["classification"], f"{cid}: classification mismatch")
            for depth, nodes in expected.get("perft", {}).items():
                require(perft(board, int(depth)) == nodes, f"{cid}: perft depth {depth} mismatch")
            for attack in expected.get("attacks", []):
                actual = board.is_attacked_by(chess.WHITE if attack["by"] == "white" else chess.BLACK, chess.parse_square(attack["square"]))
                require(actual == attack["attacked"], f"{cid}: attack mismatch at {attack['square']}")
        checked += 1
    print(f"PASS oracle chess {chess.__version__}: {checked} position/accepted-FEN cases; {rejected} parser rejection contracts are skipped by this oracle (use the Rust FEN tests).")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--corpus", type=Path, default=CORPUS)
    parser.add_argument("--oracle", action="store_true", help="recheck external reference answers; requires chess==1.11.2")
    args = parser.parse_args()
    try:
        data = json.loads(args.corpus.read_text(encoding="utf-8"))
        validate(data)
        print(f"PASS structure: {len(data['cases'])} cases with profile, input, operation, expectation and provenance.")
        if args.oracle:
            verify_oracle(data)
    except (OSError, ValueError, KeyError, TypeError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
