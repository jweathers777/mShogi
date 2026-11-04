"""Negative/mutation checks for data admission; no native Tenjiku tests."""
import copy
import json
import unittest
from pathlib import Path
import validate as v


class Admission(unittest.TestCase):
    def setUp(self):
        self.p = v.parse((Path(__file__).parent/'profile.json').read_bytes())
        self.c = v.parse((Path(__file__).parent/'fixtures.json').read_bytes())

    def reject(self, change):
        change(self.p, self.c)
        with self.assertRaises((ValueError, KeyError, TypeError)):
            v.validate(self.p, self.c)

    def test_admits_review_input(self):
        self.assertEqual(v.validate(self.p, self.c)['native_passed'], 0)

    def test_duplicate_nested_rejected_before_map(self):
        with self.assertRaises(ValueError):
            v.parse(b'{"nested":{"profile":1,"profile":2}}')

    def test_escaped_duplicate_rejected(self):
        with self.assertRaises(ValueError):
            v.parse(b'{"profile":1,"profi\\u006ce":2}')

    def test_trailing_and_nonfinite_rejected(self):
        for raw in [b'{} {}', b'{"x":NaN}', b'{"x":Infinity}']:
            with self.assertRaises(ValueError):
                v.parse(raw)

    def test_size_bound(self):
        with self.assertRaises(ValueError):
            v.parse(b' '*(v.MAX_BYTES+1))

    def test_identity_and_edition(self):
        self.reject(lambda p,c: p['profiles'][1].update(edition=p['profiles'][0]['edition']))

    def test_one_profile_cannot_replace_two(self):
        self.reject(lambda p,c: c.update(profiles=[v.IDS[0]]))

    def test_native_execution_claim_rejects(self):
        self.reject(lambda p,c: c.update(native_conformance_executed=True))

    def test_unknown_schema_field_rejects(self):
        self.reject(lambda p,c: c.update(native_passed=348))

    def test_duplicate_fixture_id(self):
        self.reject(lambda p,c: c['fixtures'].append(copy.deepcopy(c['fixtures'][0])))

    def test_wrong_profile_expectation(self):
        self.reject(lambda p,c: c['fixtures'][0]['expected'].pop(v.IDS[1]))

    def test_coordinate_and_bool(self):
        for value in [[0,8], [17,8], [True,8]]:
            with self.assertRaises(ValueError):v.coord(value)

    def test_missing_movement(self):
        self.reject(lambda p,c: c['fixtures'].pop(0))

    def test_rotation_corruption(self):
        self.reject(lambda p,c: c['fixtures'][1]['given'].update(actor=[8,8]))

    def test_setup_overlap(self):
        self.reject(lambda p,c: p['initial_board'][1].update(at=p['initial_board'][0]['at']))

    def test_distinction_cannot_be_flattened(self):
        def change(p,c):
            f=next(x for x in c['fixtures'] if x['id']=='TEN-BURN-DYING')
            f['expected'][v.IDS[1]]=copy.deepcopy(f['expected'][v.IDS[0]])
        self.reject(change)

    def test_duplicate_root_ids(self):
        def change(p,c):
            root = next(f['given']['root'] for f in c['fixtures']
                        if isinstance(f['given'].get('root'), dict))
            root['board'][1]['id'] = root['board'][0]['id']
        self.reject(change)

    def test_unknown_root_right_reference(self):
        def change(p,c):
            root = next(f['given']['root'] for f in c['fixtures']
                        if isinstance(f['given'].get('root'), dict))
            root['promotion_rights'] = [{'piece_id':'missing', 'state':'first_declined_next_move'}]
        self.reject(change)

    def test_override_type(self):
        self.reject(lambda p,c: p['profiles'][1].update(overrides=False))

    def test_unknown_nested_piece_field(self):
        self.reject(lambda p,c: p['pieces'][0].update(unknown=1))

    def test_unknown_nested_profile_field(self):
        self.reject(lambda p,c: p['profiles'][0].update(unknown=1))

    def test_scope_erasure(self):
        self.reject(lambda p,c: c['fixtures'][0].update(scope='annotation'))

    def test_same_profile_is_not_mismatch(self):
        def change(p,c):
            f = next(x for x in c['fixtures'] if x['id']=='TEN-PROFILE-MIX-EDITION')
            f['given']['incoming_profile'] = 'selected_fixture_profile'
        self.reject(change)

    def test_empty_sequence(self):
        def change(p,c):
            f = next(x for x in c['fixtures'] if 'commands' in x['given'])
            f['given']['commands'] = []
        self.reject(change)

    def test_bad_sequence_coordinate(self):
        def change(p,c):
            f = next(x for x in c['fixtures'] if 'commands' in x['given'])
            f['given']['commands'][0]['command']['action']['route'][0] = [17,1]
        self.reject(change)


if __name__ == '__main__':
    unittest.main()
