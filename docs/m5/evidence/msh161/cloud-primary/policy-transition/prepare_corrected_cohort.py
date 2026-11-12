"""Create-only, offline preparation of the remaining 160 policy-amended games.

Never activates manifests, submits jobs, or runs engines. --package-only permits
an old image solely to construct a new image; those outputs are not admitted runs.
"""
import argparse
import copy
import hashlib
import json
from pathlib import Path
import re
import sys

REPO = Path(__file__).resolve().parents[6]
sys.path.insert(0, str(REPO))
from tools.experiments.core import seal, validate_manifest

PRIMARY = Path('docs/m5/evidence/msh161/cloud-primary')
STUDY = Path('docs/m5/evidence/msh161')


def read(path):
    return json.loads(path.read_text())


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def save(path, value):
    with path.open('x') as stream:
        json.dump(value, stream, indent=2, sort_keys=True, allow_nan=False)
        stream.write('\n')


def require(condition, message):
    if not condition:
        raise ValueError(message)


def transform(old, build, amendment_ref, source_files, source_revision, image, index, package_only):
    """Re-seal parent-first, preserving every non-amended job field verbatim."""
    validate_manifest(old)
    result = copy.deepcopy(old)
    result['experiment'] = f'msh161-primary-v2-batch-{index:02d}'
    result['policy_transition'] = {'claim_policy': 'explicit_only', 'study_amendment': amendment_ref,
                                   'package_only': package_only, 'predecessor_experiment': old['experiment']}
    result['primary_batch']['cohort'] = 'msh161-primary-v2'
    mapping, jobs = {}, []
    pending = list(old['jobs'])
    while pending:
        ready = [j for j in pending if all(r['job'] in mapping for r in j.get('inputs', {}).values())]
        require(ready, 'Unresolvable dependency order')
        for previous in ready:
            job = copy.deepcopy(previous)
            job.update(experiment=result['experiment'], build=copy.deepcopy(build),
                       source_revision=source_revision, source_files=copy.deepcopy(source_files),
                       claim_policy='explicit_only', study_amendment=copy.deepcopy(amendment_ref))
            job['environment']['cloud']['image'] = image
            # A package-only artifact must be unmistakable even if separated from its manifest.
            if package_only:
                job['package_only'] = True
            for ref in job.get('inputs', {}).values():
                ref['job'] = mapping[ref['job']]
            job = seal(job)
            mapping[previous['id']] = job['id']
            jobs.append(job)
            pending.remove(previous)
    # Retain original ordering, including root/screen adjacency.
    by_id = {j['id']: j for j in jobs}
    result['jobs'] = [by_id[mapping[j['id']]] for j in old['jobs']]
    result['pipelines'] = [mapping[j] for j in old['pipelines']]
    validate_manifest(result)
    return result


def prepare(repo, build_path, amendment_path, image, source_revision, output, package_only=False):
    repo, output = Path(repo).resolve(), Path(output)
    require(not output.exists(), 'Output must not already exist')
    require(re.fullmatch(r'[0-9a-f]{40}', source_revision), 'Worker revision must be full Git SHA')
    require(re.fullmatch(r'.+@sha256:[0-9a-f]{64}', image), 'Image must use registry digest')
    build_path = Path(build_path).resolve()
    build_relative = build_path.relative_to(repo).as_posix()
    amendment_path = Path(amendment_path).resolve()
    amendment_relative = amendment_path.relative_to(repo).as_posix()
    amendment, build = read(amendment_path), read(Path(build_path))
    freeze_path, cohort_path = repo / STUDY / 'main-freeze.json', repo / PRIMARY / 'cohort-freeze.json'
    freeze, cohort = read(freeze_path), read(cohort_path)
    require(cohort['original_study_freeze_sha256'] == sha(freeze_path), 'Original study freeze changed')
    require(cohort['config_sha256'] == sha(repo / PRIMARY / 'cloud-config.json'), 'Cloud configuration changed')
    require(amendment.get('schema') == 'msh161-automatic-thresholds-v1'
            and amendment.get('claim_policy') == 'explicit_only'
            and amendment.get('uci_optional_claims_available') is False
            and amendment.get('original_study_freeze_sha256') == sha(freeze_path), 'Invalid policy amendment')
    require(re.fullmatch(r'[0-9a-f]{40}', amendment.get('native_source', '')), 'Invalid native revision')
    require(build['source'] == amendment['native_source']
            and all(build['engines'][name]['source'] == build['source'] for name in ('candidate', 'runner')),
            'Build differs from amended native source')
    require(build.get('architecture') == 'x86_64' and build.get('system') == 'Linux', 'Expected Linux x86_64 build')
    amendment_ref = {'path': amendment_relative, 'sha256': sha(amendment_path)}
    prepared, entries, originals, seen = [], [], [], []
    schedules = {e['index']: e for e in freeze['schedules']}
    cohort_entries = {e['index']: e for e in cohort['batches']}
    require(set(cohort_entries) == set(range(1, 11)), 'Original cohort must cover all ten batches')
    for index in range(3, 11):
        entry = cohort_entries[index]
        path = repo / entry['manifest']
        require(sha(path) == entry['sha256'], f'Original manifest changed: batch {index}')
        old = read(path)
        validate_manifest(old)
        schedule = repo / STUDY / 'main' / Path(schedules[index]['path']).name
        require(sha(schedule) == schedules[index]['sha256'], 'Frozen schedule changed')
        roots = [j for j in old['jobs'] if j['kind'] == 'game_pair']
        require(len(old['jobs']) == 20 and len(roots) == 10
                and [j['spec'] for j in roots] == read(schedule)['pairs'], 'Original schedule/spec mismatch')
        require(old['experiment'] == f'msh161-primary-v1-batch-{index:02d}', 'Wrong predecessor experiment')
        baseline = old['jobs'][0]
        require(build['source'] != baseline['build']['source'], 'Amendment requires a new native source')
        require(package_only or image != baseline['environment']['cloud']['image'], 'New execution requires a new image')
        for job in old['jobs']:
            require(job['source_files'] == baseline['source_files'], 'Mixed old source bundles')
            require(job['build'] == baseline['build'], 'Mixed old builds')
            require(job['study_freeze_sha256'] == sha(freeze_path), 'Old job freeze mismatch')
        for name in ('oxidation060', 'stockfish19'):
            require(build['engines'][name] == baseline['build']['engines'][name], f'Unchanged opponent/reference required: {name}')
        for name in ('candidate', 'runner'):
            require(build['engines'][name]['path'] == baseline['build']['engines'][name]['path'], 'Native executable path changed')
        sources = copy.deepcopy(baseline['source_files'])
        for name, expected in list(sources.items()):
            require((repo / name).is_file(), f'Missing source: {name}')
            if not name.startswith('tools/experiments/'):
                require(sha(repo / name) == expected, f'Frozen study source changed: {name}')
        for path in sorted((repo / 'tools/experiments').glob('*.py')):
            sources[path.relative_to(repo).as_posix()] = sha(path)
        for path in sorted((repo / PRIMARY / 'policy-transition').glob('*.py')):
            sources[path.relative_to(repo).as_posix()] = sha(path)
        sources[build_relative] = sha(build_path)
        sources[amendment_relative] = amendment_ref['sha256']
        new = transform(old, build, amendment_ref, sources, source_revision, image, index, package_only)
        name = f'batch-{index:02d}-manifest.json'
        prepared.append((name, new))
        originals.append((name, (repo / entry['manifest']).read_bytes()))
        ids = [j['spec']['id'] for j in roots]
        seen.extend(ids)
        entries.append({'index': index, 'manifest': name, 'original_sha256': entry['sha256'], 'pair_ids': ids})
    require(len(seen) == len(set(seen)) == 80, 'Remaining cohort must contain exactly 80 distinct pairs')
    # No writes until every input has passed preflight.
    output.mkdir(parents=True)
    (output / 'originals').mkdir()
    for name, raw in originals:
        (output / 'originals' / name).write_bytes(raw)
    for name, manifest in prepared:
        save(output / name, manifest)
    for entry in entries:
        entry['sha256'] = sha(output / entry['manifest'])
    receipt = {'schema': 'msh161-corrected-cohort-v1', 'package_only': package_only,
               'activation': 'Not activated; independent admission and explicit controller activation required',
               'pairs': 80, 'games': 160, 'source_revision': source_revision,
               'native_source': build['source'], 'image': image, 'study_amendment': amendment_ref,
               'build_sha256': sha(Path(build_path)), 'original_cohort_sha256': sha(cohort_path),
               'original_study_freeze_sha256': sha(freeze_path), 'batches': entries}
    save(output / 'cohort-transition.json', receipt)
    return receipt


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('build', 'amendment', 'image', 'source-revision', 'output'):
        parser.add_argument('--' + name, required=True)
    parser.add_argument('--repo', type=Path, default=REPO)
    parser.add_argument('--package-only', action='store_true')
    args = parser.parse_args()
    prepare(args.repo, args.build, args.amendment, args.image, args.source_revision, args.output, args.package_only)
    print('Prepared 80 pairs / 160 games offline; no activation or submission.')


if __name__ == '__main__':
    main()
