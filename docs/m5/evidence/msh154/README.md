# MSH-154 (Chess Adversarial Assessment): evidence

Frozen diagnostic source: d53f34413d808dc2b727a06a770b92f24191eabf. No production changes.

The initial architecture and gameplay reports retain their original text before
sharing. The methodology record includes initial design challenge, the rejected
exhaustive screen, intermediate challenges and final review. Measurement and final
gameplay appendices preserve subsequent evidence and unresolved explanations.

## Artifact paths and reproduction

Reports mention original `/tmp` paths as execution provenance. Their durable mapping is:

- `/tmp/msh-adversarial-architecture-inputs/*` → `architecture/*`.
- `/tmp/msh-gameplay-*` → `gameplay/msh-gameplay-*`.
- Initial/final reports → the Markdown files in this directory.

Files larger than 50,000 bytes are gzip-compressed without changing their contents.
`artifact-manifest.json` records both stored and original hashes. Decompress to a
scratch directory before using the original scripts. The scripts retain actual host
paths and process commands; adapt root/binary/output paths for another machine.
These are bounded one-off diagnostic drivers using the existing native example,
not a second experiment framework or product dependency. Python-chess was optional
independent legal replay/reference infrastructure, not a product rules oracle.

Build the existing `chess_frontier_profile` example at the recorded source, using
`architecture/provenance.json`. `architecture/run.py`, its JSON inputs and `runs.jsonl`
record exact commands, watchdogs and source/binary identities. `profile.py` records
separate CPU sampling; profile timings are not speed comparisons. All native roots
assert final FEN, unchanged caller state and native PV admission. The semantic live
repetition control intentionally differs from a fresh-FEN history.

The gameplay panel was frozen before usable reference queries. Initial reference
outputs omit bound flags and are exploratory only. Use `msh-gameplay-bound-safe*`
for the two corrected per-info, bound-aware comparisons, including raw UCI logs.
The original bound-safe script contains the invalid manual Ne4 mapping that caused
a caught assertion; the corrected `key03` script and separate receipt preserve the
resolution. The failed Simbelmyne screen is excluded, not omitted from the record.

No new matches, strength study, tablebase query, product modification, platform CI
or GUI campaign occurred. Source-level architecture alternatives are proposals;
performance samples and reference preferences do not establish Elo gains.
