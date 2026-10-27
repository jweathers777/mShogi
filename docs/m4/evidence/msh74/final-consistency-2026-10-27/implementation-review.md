# Independent final consistency review

Reviewer: delegated `msh74_implementation_review`, separate from the sole builder.
Product change `95c7e00..abe66c5`; mechanical fixture correction `abe66c5..7df91d3`.

Method: read the bounded source/test diff and latest owner instructions; trace
both product adapters and legacy toolbar through the disk-label helper; compare
native Shogi submitted play limits with snapshot presentation; trace selected
canonical history to board ply and terminal-event behavior; inspect unchanged
native dirty, file guards, command admission and result acceptance. Review the
external driver label adaptations without weakening exact-path, settled-control
or inspection invariants. No source edits by reviewer.

Independently executed: strict TypeScript and34 frontend tests in App, ShogiApp,
ApplicationShell and Document files; one native budget/selected-ply regression;
12 general external-driver unit tests. All passed. Diff check passed.

No material findings. Initial source inspection observed a mid-edit struct-field
placement, corrected before the reviewed tests; it is not a remaining finding.
The fixture follow-up only moves/formats four JSON fields; independently parsed
before/after values are deeply equal, with no product source change. Reviewer
ran the native generator with desktop-verification enabled into a separate
directory:22 JSONs all byte-match. Builder's default-feature separate-directory
run generates20 JSONs (the capacity/synthetic verification fixtures are omitted),
also all byte-matching. CI's drift check is retained.

This is independent source/test review, not a claim of actual GUI appearance or
platform success. Those scopes are recorded separately. Prior bounded reviews
remain applicable to unchanged rules, archives, execution and inspection.

External-driver follow-up, independently reviewed after failed Linux assertions:
only gui-smoke.py and its tests change. The play-limit check retains the full
label, exact depth and exact integer node value while accepting locale digit
grouping; wrong/missing budgets remain rejected. The ply helper now matches1ply
and0/2plies in live and historical positions. No product source, native policy,
replay/admission, exact saved-path or inspection-state equality is relaxed.
Reviewer independently ran25 external-driver tests, pycompile and diff check;
all passed, no finding. Final source `1b10277436b94541235e15189a3af7fce867351c`
contains this verification-only correction; product code remains byte-identical
to the previously reviewed correction.
