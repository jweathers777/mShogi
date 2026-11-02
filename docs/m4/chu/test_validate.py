"""Data-admission mutation tests, not native Chu rule tests."""
import copy
import importlib.util
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("chu_data", ROOT / "validate.py")
DATA = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(DATA)


class Admission(unittest.TestCase):
    def setUp(self):
        self.profile = DATA.parse((ROOT / "profile.json").read_bytes())
        self.corpus = DATA.parse((ROOT / "fixtures.json").read_bytes())

    def reject(self, mutate):
        mutate(self.profile, self.corpus)
        with self.assertRaises(ValueError):
            DATA.validate(self.profile, self.corpus)

    def test_valid_reports_zero_native_execution(self):
        report = DATA.validate(self.profile, self.corpus)
        self.assertEqual(report["native_executed"], 0)
        self.assertEqual(report["native_passed"], 0)

    def test_duplicate_outer_and_nested(self):
        for raw in [b'{"a":1,"a":2}', b'{"a":{"x":1,"x":2}}']:
            with self.assertRaisesRegex(ValueError, "duplicate field"):
                DATA.parse(raw)

    def test_unknown_outer(self):
        self.reject(lambda p, c: c.update(unknown=True))

    def test_missing_outer(self):
        self.reject(lambda p, c: c.pop("native_conformance_executed"))

    def test_wrong_profile(self):
        self.reject(lambda p, c: p.update(profile="mshogi.tori.modern.v1"))

    def test_wrong_edition(self):
        self.reject(lambda p, c: p.update(edition="unreviewed"))

    def test_fabricated_native_pass(self):
        self.reject(lambda p, c: c.update(native_conformance_executed=True))

    def test_duplicate_id(self):
        self.reject(lambda p, c: c["fixtures"].append(copy.deepcopy(c["fixtures"][0])))

    def test_fixture_unknown_field(self):
        self.reject(lambda p, c: c["fixtures"][0].update(unknown=True))

    def test_unresolved_choice(self):
        self.reject(lambda p, c: c["fixtures"][0].update(requires_owner_decisions=["new"]))

    def test_dangling_reference(self):
        self.reject(lambda p, c: c["fixtures"][-1]["given"].update(fixture_ref="missing"))

    def test_wrong_source(self):
        self.reject(lambda p, c: c["fixtures"][0].update(sources=["engine:answer"]))

    def test_missing_promotion_category(self):
        self.reject(lambda p, c: c.update(fixtures=[f for f in c["fixtures"]
                    if f["id"] != "CHU-PROMOTE-Ki-NOCHAIN"]))

    def test_wrong_rotation(self):
        def mutate(p, c):
            f = next(f for f in c["fixtures"] if "rotation_of" in f)
            f["given"]["root"]["turn"] = "sente"
        self.reject(mutate)

    def test_bad_piece_square_and_unknown_root(self):
        for change in [lambda r: r["board"][0].update(at=[0, 1]),
                       lambda r: r["board"][0].update(piece="K+"),
                       lambda r: r.update(unknown=True),
                       lambda r: r["board"].append(copy.deepcopy(r["board"][0]))]:
            p, c = copy.deepcopy(self.profile), copy.deepcopy(self.corpus)
            root = next(f["given"]["root"] for f in c["fixtures"]
                        if isinstance(f["given"].get("root"), dict))
            change(root)
            with self.assertRaises(ValueError):
                DATA.validate(p, c)

    def test_bounded_input(self):
        with self.assertRaisesRegex(ValueError, "1MiB"):
            DATA.parse(b" " * (DATA.MAX_BYTES + 1))

    def test_all_144_squares_rotation_and_explicit_exchange(self):
        for f in range(1, 13):
            for r in range(1, 13):
                point = {"side": "sente", "at": [f, r]}
                rotated = DATA.rotate(point)
                self.assertEqual(rotated, {"side": "gote", "at": [13-f, 13-r]})
                self.assertEqual(DATA.rotate(rotated), point)


if __name__ == "__main__":
    unittest.main()
