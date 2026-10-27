#!/usr/bin/env python3
"""Collect bounded board-only evidence from frozen MSH-94 source fixtures.

Developer-only: no game/outcome/reference rules-judge, engine weights or search.
Never writes fixture expectations. The independently expected history outcomes
are tested by the native Game integration tests, not by this collector.
"""
import argparse
import hashlib
import json
import platform
import shlex
import subprocess
import sys
from pathlib import Path

FIXTURES = (
    'history-cycles.json',
    'natural-count.json',
    'history-mutual-check.json',
    'history-precedence.json',
)
PROFILE = 'mshogi.xiangqi.wxf2018.v1'


def digest(data):
    return hashlib.sha256(data).hexdigest()


def encoded(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':')).encode()


def git(repo, *arguments):
    return subprocess.run(['git', '-C', str(repo), *arguments], check=True,
                          capture_output=True, text=True).stdout.strip()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path, default=Path.cwd())
    parser.add_argument('--native-probe', type=Path)
    parser.add_argument('--manifest', type=Path)
    parser.add_argument('--native-only', action='store_true',
                        help='Do not run or claim admitted-reference comparisons.')
    parser.add_argument('--timeout', type=float, default=20.0)
    parser.add_argument('--report', required=True, type=Path,
                        help='Compact output summary; raw observations are hashed only.')
    args = parser.parse_args()
    repo = args.repo.resolve()
    sys.path.insert(0, str(repo / 'tools/xiangqi'))
    import reference

    native = (args.native_probe or repo / 'target/debug/examples/xiangqi_probe').resolve()
    manifest = (args.manifest or repo / 'docs/m4/xiangqi/reference/pikafish-admission.json').resolve()
    if not 0 < args.timeout <= 120:
        raise ValueError('bounded timeout must be greater than0 and at most120 seconds')
    native_sha = reference.sha(native)
    script = Path(__file__).resolve()
    source_paths = sorted((repo / 'crates/mshogi-core/src/xiangqi').glob('*.rs'))
    source_paths += [repo / 'crates/mshogi-core/examples/xiangqi_probe.rs', repo / 'Cargo.lock']
    source_hashes = {str(p.relative_to(repo)): reference.sha(p) for p in source_paths}
    corpus = []
    input_hashes = {}
    for name in FIXTURES:
        path = repo / 'tests/fixtures/xiangqi' / name
        raw = path.read_bytes()
        if len(raw) > 262144:
            raise ValueError(f'fixture file exceeds256KiB: {name}')
        parsed = reference.strict_json(raw)
        if parsed.get('profile') != PROFILE:
            raise ValueError(f'wrong fixture profile: {name}')
        input_hashes[str(path.relative_to(repo))] = digest(raw)
        members = parsed.get('fixtures', [parsed])
        if not isinstance(members, list) or not members or len(members) > 64:
            raise ValueError(f'invalid bounded fixture collection: {name}')
        for item in members:
            request = {'root_fen': item['root_fen'], 'moves': item['moves']}
            reference.validate_request(request)
            corpus.append((name, item['id'], request))
    if len({item[1] for item in corpus}) != len(corpus):
        raise ValueError('duplicate fixture identities')
    reference_info = None
    if not args.native_only:
        binary = reference.admitted_binary(manifest)
        reference_info = {
            'source_revision': reference.PIN,
            'manifest_path': str(manifest),
            'manifest_sha256': reference.sha(manifest),
            'binary_path': str(binary),
            'binary_sha256': reference.sha(binary),
            'wrapper_sha256': reference.sha(repo / 'tools/xiangqi/reference.py'),
            'harness_sha256': reference.sha(repo / 'tools/xiangqi/pikafish_board_probe.cpp'),
        }
    results = []
    all_observations = hashlib.sha256()
    for file_name, fixture_id, request in corpus:
        observations = hashlib.sha256()
        prefixes = len(request['moves']) + 1
        check_prefixes = []
        for count in range(prefixes):
            prefix = {'root_fen': request['root_fen'], 'moves': request['moves'][:count]}
            data = encoded(prefix)
            response = reference.strict_json(reference.bounded_run(
                [str(native)], data, args.timeout))
            reference.validate_response(response, 0)
            other = None
            if not args.native_only:
                other = reference.query(prefix, manifest, args.timeout)
                if response['legal_moves'] != other['legal_moves']:
                    raise ValueError(f'{fixture_id} prefix{count}: complete legal-action mismatch; '
                                     f'missing={sorted(set(other["legal_moves"])-set(response["legal_moves"]))}; '
                                     f'extra={sorted(set(response["legal_moves"])-set(other["legal_moves"]))}')
                if response['in_check'] != other['in_check']:
                    raise ValueError(f'{fixture_id} prefix{count}: check mismatch')
                left, right = response['fen'].split(), other['fen'].split()
                # Pikafish rule60/check10 halfcounter is not the native declared
                # counter or claimant-specific WXF Art8 history. Only exclude
                # that one field; retain both raw observations in the hash.
                if left[:4] + left[5:] != right[:4] + right[5:]:
                    raise ValueError(f'{fixture_id} prefix{count}: board/side/fullmove mismatch')
            if response['in_check']:
                check_prefixes.append(count)
            frame = {'fixture': fixture_id, 'prefix_plies': count,
                     'request': prefix, 'native': response, 'reference': other}
            line = encoded(frame) + b'\n'
            observations.update(line)
            all_observations.update(line)
        results.append({
            'fixture_file': file_name, 'fixture': fixture_id,
            'complete_actions': len(request['moves']), 'prefixes': prefixes,
            'native_prefixes_passed': prefixes,
            'admitted_reference_comparisons_passed': 0 if args.native_only else prefixes,
            'native_only_prefixes': prefixes if args.native_only else 0,
            'side_to_move_checked_prefixes': check_prefixes,
            'raw_observation_stream_sha256': observations.hexdigest(),
        })
    # Detect concurrent mutation rather than silently associating a report with
    # final files or executable bytes that were not actually observed.
    for name, expected in input_hashes.items():
        if reference.sha(repo / name) != expected:
            raise ValueError(f'fixture changed during collection: {name}')
    if reference.sha(native) != native_sha:
        raise ValueError('native probe changed during collection')
    for name, expected in source_hashes.items():
        if reference.sha(repo / name) != expected:
            raise ValueError(f'native source changed during collection: {name}')
    total = sum(item['prefixes'] for item in results)
    result = {
        'format': 'mshogi.xiangqi.history-board-observation-summary.v1',
        'scope': 'board-only observations; no history/adjudication/claim/outcome oracle',
        'expected_fixtures_modified': False,
        'all_requested_checks_passed': True,
        'source_revision': git(repo, 'rev-parse', 'HEAD'),
        'environment': {'system': platform.system(), 'release': platform.release(),
                        'machine': platform.machine(), 'python': platform.python_version()},
        'source_working_tree_dirty': bool(git(repo, 'status', '--porcelain')),
        'native_source_sha256': source_hashes,
        'collector_sha256': reference.sha(script),
        'command': shlex.join([sys.executable, str(script), *sys.argv[1:]]),
        'native_probe': {'path': str(native), 'sha256': native_sha},
        'admitted_reference': reference_info,
        'fixture_file_sha256': input_hashes,
        'fixture_histories': len(results), 'native_prefixes_passed': total,
        'admitted_reference_comparisons_passed': 0 if args.native_only else total,
        'native_only_prefixes': total if args.native_only else 0,
        'comparison': 'complete sorted legal actions, side-to-move check, exact undo, board/side/fullmove; foreign halfcounter excluded only',
        'raw_observation_stream_sha256': all_observations.hexdigest(),
        'raw_retention': 'not stored by collector; deterministic ordered frames hashed; rerun collector to reproduce',
        'fixtures': results,
    }
    args.report.parent.mkdir(parents=True, exist_ok=True)
    temporary = args.report.with_name(args.report.name + '.tmp')
    temporary.write_text(json.dumps(result, indent=2) + '\n')
    temporary.replace(args.report)
    print(f'PASS: {len(results)} histories, {total} native prefixes, '
          f'{result["admitted_reference_comparisons_passed"]} admitted-reference comparisons, '
          f'{result["native_only_prefixes"]} native-only prefixes; {args.report}')


if __name__ == '__main__':
    try:
        main()
    except (ValueError, OSError, KeyError, TypeError, subprocess.SubprocessError) as error:
        print(f'FAIL: {error}', file=sys.stderr)
        sys.exit(1)
