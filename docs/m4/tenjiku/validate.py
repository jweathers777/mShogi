"""Offline two-profile fixture admission. No move generator, effect resolver or rules engine."""
import collections
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
IDS = ('mshogi.tenjiku.adams1999-modern.v1', 'mshogi.tenjiku.muller-completed.v1')
EDITION = ('ADAMS1999-MODERN+MSH83-2026-11-04', 'MULLER+ADAMS-COMPLETION+MSH83-2026-11-04')
MAX_BYTES = 2 * 1024 * 1024  # Development data, not a game/transport/document limit.


def require(test, message):
    if not test:
        raise ValueError(message)


def unique(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result, 'duplicate field: ' + key)
        result[key] = value
    return result


def parse(raw):
    require(len(raw) <= MAX_BYTES, 'development input exceeds2MiB')
    return json.loads(raw, object_pairs_hook=unique,
                      parse_constant=lambda value: (_ for _ in ()).throw(ValueError('nonfinite JSON')))


def coord(value):
    require(isinstance(value, list) and len(value) == 2 and
            all(type(n) is int and 1 <= n <= 16 for n in value), 'invalid16x16 coordinate')


def rotate(value, key=None):
    if isinstance(value, dict):
        return {k: rotate(v, k) for k, v in value.items()}
    if isinstance(value, list):
        if len(value) == 2 and all(type(x) is int for x in value) and key in {
                'actor', 'from', 'route', 'target', 'potential_igui_targets'}:
            return [17-value[0], 17-value[1]]
        return [rotate(x, key) for x in value]
    if value == 'sente':
        return 'gote'
    if value == 'gote':
        return 'sente'
    return value


def validate(profile, corpus):
    require(set(profile) == {'schema', 'status', 'revision', 'selection_authority', 'profiles',
                            'board', 'initial_board', 'originals', 'promotion', 'pieces',
                            'shared_contract', 'pending_owner_decisions', 'sources'}, 'profile fields')
    require(profile['schema'] == 'mshogi.tenjiku.profile-data.v1', 'profile schema')
    require(profile['revision'] == 'MSH83-2026-11-04' and profile['pending_owner_decisions'] == [], 'profile unresolved')
    require(profile['status'] in {'selected_pending_independent_review', 'ratified'}, 'profile status')
    require([p['id'] for p in profile['profiles']] == list(IDS) and
            [p['edition'] for p in profile['profiles']] == list(EDITION), 'two profile identities/editions')
    override_keys = {'FEg', 'HT', 'FiD', 'ranked_destination'}
    for index, p in enumerate(profile['profiles']):
        require(set(p) == {'id', 'edition', 'basis', 'overrides'} and
                isinstance(p['basis'], str) and isinstance(p['overrides'], dict), 'profile nested fields')
        require(set(p['overrides']) == (set() if index == 0 else override_keys) and
                all(isinstance(x, str) and x for x in p['overrides'].values()), 'override data')
    require(profile['board'] == {'files': 16, 'ranks': 16, 'topology': 'squares',
                                  'sente_forward_rank_delta': -1}, 'board geometry')
    originals = set(profile['originals'])
    require(len(originals) == len(profile['originals']) == 36 and len(profile['promotion']) == 30,
            'original/promotion counts')
    require(set(profile['promotion']) == originals - {'K', 'FiD', 'GGn', 'VGn', 'FEg', 'LHk'},
            'promotion originals')
    faces = {p['face'] for p in profile['pieces']}
    require(len(faces) == len(profile['pieces']) == 44 and
            faces == originals | set(profile['promotion'].values()), 'movement identities')
    for piece in profile['pieces']:
        require(set(piece) == {'face', 'name', 'adams_printed_page', 'steps', 'rays',
                               'jumps', 'limited_rays', 'special'} and
                isinstance(piece['name'], str), 'piece nested fields')
    placements = set()
    inventory = collections.Counter()
    for piece in profile['initial_board']:
        require(set(piece) == {'side', 'piece', 'at'} and piece['side'] in {'sente', 'gote'}
                and piece['piece'] in originals, 'setup piece')
        coord(piece['at'])
        square = tuple(piece['at'])
        require(square not in placements, 'setup overlap')
        placements.add(square)
        inventory[piece['side'], piece['piece']] += 1
    require(len(placements) == 156 and sum(n for (s, _), n in inventory.items() if s == 'sente') == 78,
            'setup inventory')
    require(all(inventory['sente', k] == inventory['gote', k] for k in originals), 'side inventory')
    lower = {(x['piece'], tuple(x['at'])) for x in profile['initial_board'] if x['side'] == 'sente'}
    upper = {(x['piece'], tuple(x['at'])) for x in profile['initial_board'] if x['side'] == 'gote'}
    require({(k, (17-f, 17-r)) for k, (f, r) in lower} == upper, 'setup owner rotation')
    require(('K', (9, 16)) in lower and ('DE', (8, 16)) in lower and
            ('LHk', (9, 14)) in lower and ('FEg', (8, 14)) in lower, 'central setup asymmetry')
    require(set(corpus) == {'schema', 'revision', 'profiles', 'profile_status',
                           'native_conformance_executed', 'source_ids', 'notation', 'fixtures'}, 'corpus fields')
    require(corpus['schema'] == 'mshogi.tenjiku.fixtures.v1' and corpus['revision'] == profile['revision']
            and corpus['profiles'] == list(IDS) and corpus['profile_status'] == profile['status'], 'corpus identity')
    require(corpus['native_conformance_executed'] is False, 'native execution cannot be claimed by loader')
    require(corpus['source_ids'] == ['A', 'M', 'S'], 'sources')
    rows = corpus['fixtures']
    ids = [f.get('id') for f in rows]
    require(all(isinstance(x, str) and x.startswith('TEN-') for x in ids) and len(set(ids)) == len(ids), 'fixture IDs')
    by_id = {f['id']: f for f in rows}
    scopes = parse((ROOT/'audit-lock.json').read_bytes())['fixture_scopes']
    require(set(ids) == set(scopes), 'missing/unknown reviewed fixture')
    clauses = set()
    for f in rows:
        require(set(f) in ({'id', 'clauses', 'sources', 'scope', 'given', 'expected', 'expectation_method'},
                              {'id', 'clauses', 'sources', 'scope', 'given', 'expected', 'expectation_method', 'rotation_of'}), 'fixture fields')
        require(f['scope'] in {'native', 'consumer', 'annotation'}, 'fixture scope')
        require(f['scope'] == scopes[f['id']], 'reviewed fixture scope changed')
        require(isinstance(f['given'], dict) and set(f['expected']) == set(IDS) and
                all(isinstance(e, dict) and e for e in f['expected'].values()) and f['expectation_method'], 'two-profile expectations')
        require(f['sources'] and set(f['sources']) <= {'A', 'M', 'S'}, 'fixture provenance')
        clauses.update(f['clauses'])
        if 'root' in f['given'] and isinstance(f['given']['root'], dict):
            r = f['given']['root']
            require(set(r) == {'origin', 'known_pre_root_history', 'turn', 'board', 'promotion_rights'} and
                    r['origin'] == 'imported_diagram' and r['known_pre_root_history'] is False and
                    r['turn'] in {'sente', 'gote'}, 'root provenance')
            used = set()
            piece_ids = set()
            for p in r['board']:
                require(set(p) == {'id', 'side', 'original', 'promoted', 'at'} and
                        p['original'] in originals and p['side'] in {'sente', 'gote'} and
                        type(p['promoted']) is bool and (not p['promoted'] or p['original'] in profile['promotion']), 'native root identity')
                coord(p['at']);at = tuple(p['at'])
                require(at not in used, 'root overlap');used.add(at)
                require(isinstance(p['id'], str) and p['id'] not in piece_ids, 'root piece ID')
                piece_ids.add(p['id'])
            require(isinstance(r['promotion_rights'], list), 'root rights list')
            rights_ids = set()
            for right in r['promotion_rights']:
                require(set(right) == {'piece_id', 'state'} and
                        right['piece_id'] in piece_ids and right['piece_id'] not in rights_ids and
                        right['state'] in {'unseen', 'first_declined_next_move', 'restriction_consumed'}, 'root rights reference')
                rights_ids.add(right['piece_id'])
        if 'commands' in f['given']:
            commands = f['given']['commands']
            require(isinstance(commands, list) and commands, 'nonempty command sequence')
            for command in commands:
                require(set(command) == {'actor', 'command'} and command['actor'] in {'sente', 'gote'}, 'command actor')
                cmd = command['command']
                require(set(cmd) == {'kind', 'action'} and cmd['kind'] == 'play', 'command shape')
                action = cmd['action']
                require(set(action) == {'kind', 'from', 'route', 'promote'} and
                        action['kind'] == 'travel' and type(action['promote']) is bool and
                        isinstance(action['route'], list) and 1 <= len(action['route']) <= 3, 'sequence action')
                coord(action['from'])
                for square in action['route']:
                    coord(square)
        if f['id'].startswith('TEN-PROFILE-MIX-'):
            require(f['given']['accepted_profile'] == 'selected_fixture_profile' and
                    f['given']['incoming_profile'] == 'the_other_profile', 'profile-mix directions')
        for e in f['expected'].values():
            if 'complete_geometry_actions' in e:
                actions = e['complete_geometry_actions']
                require(e['complete_geometry_count'] == len(actions), 'complete geometry count')
                keys = set()
                for a in actions:
                    require(set(a) == {'kind', 'from', 'route', 'promote'} and a['kind'] == 'travel' and
                            a['promote'] is False and isinstance(a['route'], list) and 1 <= len(a['route']) <= 3, 'geometry action shape')
                    coord(a['from'])
                    for square in a['route']:
                        coord(square)
                    key = json.dumps(a, sort_keys=True)
                    require(key not in keys, 'duplicate complete action');keys.add(key)
                for square in e['potential_igui_targets']:
                    coord(square)
        if 'rotation_of' in f:
            require(f['rotation_of'] in by_id, 'rotation reference')
            source = by_id[f['rotation_of']]
            require(f['given'] == rotate(source['given']) and f['expected'] == rotate(source['expected']), '16x16 rotation')
    require(clauses == {'T%02d' % i for i in range(1, 13)}, 'complete clause coverage')
    for code in faces:
        require('TEN-MOVE-'+code in by_id and 'TEN-MOVE-'+code+'-ROT' in by_id, 'movement/rotation gap')
    for code in profile['promotion']:
        for suffix in ('YES', 'NO'):
            require('TEN-PROMOTION-CONCRETE-'+code+'-'+suffix in by_id, 'concrete promotion gap')
    for id in ['TEN-BURN-DYING', 'TEN-FREE-EAGLE-ORTH-JUMP', 'TEN-TETRARCH-VERTICAL', 'TEN-GENERAL-VICTIM-VGn']:
        require(by_id[id]['expected'][IDS[0]] != by_id[id]['expected'][IDS[1]], 'missing distinguishing expectation')
    counts = collections.Counter(f['scope'] for f in rows)
    return {'data_validation': 'PASS', 'rows': len(rows), 'scope_rows': dict(counts),
            'rotated_rows': sum('rotation_of' in f for f in rows),
            'expected_top_level_fields_by_profile': {id: sum(len(f['expected'][id]) for f in rows) for id in IDS},
            'native_executed': 0, 'native_passed': 0, 'native_owner': 'MSH-87',
            'consumer_owners': ['MSH-79', 'MSH-44'], 'note': 'Counts include documentary annotations, not executable native assertions.'}


def main():
    profile_raw = (ROOT/'profile.json').read_bytes()
    corpus_raw = (ROOT/'fixtures.json').read_bytes()
    result = validate(parse(profile_raw), parse(corpus_raw))
    lock = parse((ROOT/'audit-lock.json').read_bytes())
    for name, raw in [('profile.json', profile_raw), ('fixtures.json', corpus_raw)]:
        require(lock['files'][name] == hashlib.sha256(raw).hexdigest(), 'reviewed data hash changed: '+name)
    corpus = parse(corpus_raw)
    for profile_id in IDS:
        projection = {'profile': profile_id, 'fixtures': [
            {'id': f['id'], 'given': f['given'], 'expected': f['expected'][profile_id], 'scope': f['scope']}
            for f in corpus['fixtures']]}
        digest = hashlib.sha256(json.dumps(projection, sort_keys=True, separators=(',', ':')).encode()).hexdigest()
        require(lock['profile_expectations'][profile_id] == digest, 'profile expectation digest changed')
    result['audit_hashes'] = 'PASS'
    print(json.dumps(result, indent=2))


if __name__ == '__main__':
    main()
