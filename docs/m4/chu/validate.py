"""Offline fixture/provenance validation only. Implements no Chu rules."""
import collections
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
PROFILE = "mshogi.chu.jcsa2019.v1"
EDITION = "JCSA-2019-09-14+2004-06-06+MSH82-2026-11-02"
MAX_BYTES = 1024 * 1024  # Development data bound, not a product archive limit.
BASES = set("P GB L RC Cu Si G FL BT DE K Ki Ph SM VM R B DH DK Ln FK".split())
PROMOTABLE = BASES - {"K", "Ln", "FK"}
FACES = BASES | {"Pr", "WH", "W", "FS", "FO", "FB", "SE", "HF"}
FIXTURE_KEYS = {"id", "clauses", "sources", "scope", "given", "expected",
                "expectation_method", "requires_owner_decisions"}


def unique(pairs):
    value = {}
    for key, item in pairs:
        if key in value:
            raise ValueError("duplicate field: " + key)
        value[key] = item
    return value


def parse(raw):
    if len(raw) > MAX_BYTES:
        raise ValueError("development data exceeds 1MiB")
    return json.loads(raw, object_pairs_hook=unique)


def require(test, message):
    if not test:
        raise ValueError(message)


def square(value, where):
    require(isinstance(value, list) and len(value) == 2 and
            all(type(n) is int and 1 <= n <= 12 for n in value),
            where + ": expected [file,rank] in1..12")


def board_root(value, where, inventory):
    if isinstance(value, str):
        require(value == "standard_start", where + ": unsupported root")
        return
    require(isinstance(value, dict) and set(value) == {
        "origin", "declared_board_plies", "known_history_before_root", "turn", "board"
    }, where + ": missing/unknown root field")
    require(value["origin"] == "imported_diagram" and
            value["known_history_before_root"] is False, where + ": provenance")
    require(type(value["declared_board_plies"]) is int and
            0 <= value["declared_board_plies"] < 2**64, where + ": counter")
    require(value["turn"] in {"sente", "gote"}, where + ": side")
    require(isinstance(value["board"], list) and len(value["board"]) <= 92,
            where + ": board list")
    occupied = set()
    counts = collections.Counter()
    for p in value["board"]:
        require(isinstance(p, dict) and set(p) == {"side", "piece", "at"},
                where + ": piece fields")
        require(p["side"] in {"sente", "gote"}, where + ": piece side")
        code = p["piece"]
        require(isinstance(code, str) and
                (code in BASES or code.endswith("+") and code[:-1] in PROMOTABLE),
                where + ": original/promoted piece identity")
        square(p["at"], where)
        at = tuple(p["at"])
        require(at not in occupied, where + ": occupied twice")
        occupied.add(at)
        counts[p["side"], code.removesuffix("+")] += 1
    require(all(n <= inventory[k] for k, n in counts.items()),
            where + ": exceeds original native inventory (not a reachability proof)")


def rotate(value, key=None):
    if isinstance(value, dict):
        return {k: rotate(v, k) for k, v in value.items()}
    if isinstance(value, list):
        if (len(value) == 2 and all(type(n) is int for n in value) and
                key in {"at", "actor", "from", "route"}):
            return [13-value[0], 13-value[1]]
        return [rotate(v, key) for v in value]
    if value == "sente":
        return "gote"
    if value == "gote":
        return "sente"
    return value


def validate(profile, corpus):
    require(set(profile) == {"schema", "profile", "edition", "status", "board",
            "initial_board", "promotion", "pieces", "pending_owner_decisions",
            "selection_authority", "sources"}, "profile: missing/unknown field")
    require(profile["schema"] == "mshogi.chu.profile-data.v1" and
            profile["profile"] == PROFILE and profile["edition"] == EDITION,
            "profile identity/edition")
    require(profile["status"] in {"selected_pending_independent_review", "ratified"}
            and profile["pending_owner_decisions"] == [], "profile unresolved")
    require(profile["board"] == {"files": 12, "ranks": 12, "topology": "squares",
            "sente_forward_rank_delta": -1}, "board identity")
    require(len(profile["initial_board"]) == 92, "setup piece count")
    inventory = collections.Counter((p["side"], p["piece"]) for p in profile["initial_board"])
    require(sum(n for (s, _), n in inventory.items() if s == "sente") == 46 and
            sum(n for (s, _), n in inventory.items() if s == "gote") == 46,
            "setup side count")
    require(set(k for s, k in inventory) == BASES and
            set(profile["promotion"]) == PROMOTABLE and
            len(profile["pieces"]) == 29 and
            {p["face"] for p in profile["pieces"]} == FACES, "piece/promotion coverage")
    require(set(corpus) == {"schema", "revision", "profile", "profile_status",
            "native_conformance_executed", "source_ids", "notation", "fixtures"},
            "corpus: missing/unknown field")
    require(corpus["schema"] == "mshogi.chu.fixtures.v1" and
            corpus["profile"] == PROFILE and
            corpus["profile_status"] == profile["status"] and
            corpus["revision"] == "MSH82-2026-11-02", "corpus identity")
    require(corpus["native_conformance_executed"] is False, "no native execution exists")
    rows = corpus["fixtures"]
    require(isinstance(rows, list), "fixtures list")
    ids = [f.get("id") for f in rows]
    require(all(isinstance(i, str) for i in ids) and len(set(ids)) == len(ids),
            "missing/duplicate fixture ID")
    by_id = {f["id"]: f for f in rows}
    clauses = set()
    for f in rows:
        where = f["id"]
        require(set(f) in (FIXTURE_KEYS, FIXTURE_KEYS | {"rotation_of"}),
                where + ": missing/unknown fixture field")
        require(f["scope"] in {"native", "consumer"} and f["requires_owner_decisions"] == [],
                where + ": unresolved/unknown scope")
        require(isinstance(f["expected"], dict) and f["expected"] and
                isinstance(f["given"], dict) and f["sources"] and f["expectation_method"],
                where + ": expected/given/provenance")
        require(all(s.split(":", 1)[0] in corpus["source_ids"] for s in f["sources"]),
                where + ": unknown source")
        clauses.update(f["clauses"])
        if "root" in f["given"]:
            board_root(f["given"]["root"], where, inventory)
        for field in {"fixture_ref", "native_fixture_ref"} & f["given"].keys():
            require(f["given"][field] in by_id, where + ": dangling fixture reference")
        if "rotation_of" in f:
            require(f["rotation_of"] in by_id and
                    f["given"] == rotate(by_id[f["rotation_of"]]["given"]),
                    where + ": wrong rotation/owner exchange")
    require(clauses == {"C%02d" % n for n in range(1, 12)}, "missing/unknown clause")
    require({f["id"][9:] for f in rows if f["id"].startswith("CHU-MOVE-") and
             not f["id"].endswith("-ROT")} == FACES, "movement coverage")
    for code in PROMOTABLE:
        require(all("CHU-PROMOTE-"+code+suffix in by_id
                    for suffix in ["-YES", "-NO", "-NOCHAIN"]), "promotion choice gap")
    return {"data_validation": "PASS", "fixtures": len(rows),
            "authored_rows": sum("rotation_of" not in f for f in rows),
            "rotated_rows": sum("rotation_of" in f for f in rows),
            "consumer_rows": sum(f["scope"] == "consumer" for f in rows),
            "expected_top_level_fields": sum(len(f["expected"]) for f in rows),
            "native_executed": 0, "native_passed": 0,
            "pending_native_owner": "MSH-86", "pending_consumer_owners": ["MSH-78", "MSH-43"]}


def main():
    profile = parse((ROOT / "profile.json").read_bytes())
    corpus = parse((ROOT / "fixtures.json").read_bytes())
    result = validate(profile, corpus)
    lock = parse((ROOT / "audit-lock.json").read_bytes())
    for name, expected in lock["sha256"].items():
        actual = hashlib.sha256((ROOT / name).read_bytes()).hexdigest()
        require(actual == expected, name+": differs from independently reviewed audit anchor")
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
