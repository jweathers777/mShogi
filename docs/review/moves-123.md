# MSH-123: Moves notation, exact positions and completed-action cues

Implementation baseline `0b4218deaa8dbe705b882169d9e4cdab33333b0e`, primary
`codex/rewrite`. Root Codex is builder/integrator. This is the bounded product
follow-up to R5, not MSH-124/125/R6. Current status: source review passed;
packaged interaction and final CI acceptance pending. No whole-task completion
is inferred from unit tests.

## Native ownership and display contract

`mshogi-desktop-session::moves` builds a native projection over admitted actions,
positions and exact events. A transient document/version cache avoids repeated
notation reconstruction during polling. Selection chooses the latest genuine
action at that exact prefix. No canonical record, archive, profile, engine work
limit, stale-result admission or capacity policy changes. The only core addition
is immutable Xiangqi command-prefix range access; it performs no replay.

The projection supplies native pair number/column, standard notation, opaque
complete action, exact post-action event count and separate command start/end.
The shared Moves table consumes those facts. Imported counters never fabricate
moves: Chess uses its fullmove counter, Shogi shows known relative pairs with
its declared SFEN counter and unknown earlier history, and Xiangqi preserves
its supplied/unknown fullmove provenance. Second-player roots leave the first
cell empty. Starting position is an explicit native target.

Non-board commands and derived events stay in Position events. They retain
independent exact selection and associated move annotations, without becoming
move rows. In WXF F022, action-prefix8 and warning-prefix9 have the same board
but different history. The move selects8; native analysis there is explicitly
unsupported because it is inside an atomic command. Committed9 analysis retains
the complete warning history. Neither frontend nor projection normalizes8 to9.

Selected-prefix geometry carries native board/intersection/hand origin, ordered
steps, complete identity and secondary effects. Chess castling includes native
rook movement; drops have only a destination; synthetic routes retain repeated
endpoints and order. Renderers are read-only decoration. Destination corners
plus double underline differ from small dashed origin corners, legal targets,
partial routes, hollow inspection and keyboard focus. The single R5 controller,
query helper and R4 application composition remain unchanged.

## Sources, expectations and independent review

[Independent source research](evidence/msh123/notation-sources.md) directly read
PGN SAN and the official JSA guide; selected English/Chinese WXF PDFs were
hash-verified and visually inspected. SAN uses legal alternatives and native
post-action check/mate. JSA uses numeric file/rank, Japanese promoted identities,
known-history 同, contextual 打, 成/不成 and mover-relative ambiguity, preserving
the explicit no-直 exception for 竜/馬. WXF uses K/A/E/H/R/C/P, mover files,
advance/retreat/traverse, distance versus destination file and front/rear.

The owner explicitly approved ordinal+file for four/five soldiers and ambiguous
multiple paired files. This source-consistent display extension is documented
in [the product convention](../desktop/post-r4-ux.md#msh-123-selected-notation-completion-2026-10-31),
not misrepresented as literal WXF text. Independent FigureB expectations remain
`36=5`, `26=5`, `16=5`. A candidate rear-soldier fixture before the river was
corrected to a legal crossed-river position; native legality assertions were
preserved rather than changing rules to match a candidate expectation.

[Native independent review](evidence/msh123/native-independent-review.md) by a
separate source researcher/reviewer passed, including independently executed
18 filtered tests (16 new notation/projection and two existing persistence
matches). Another bounded worker added three actual session-consumer tests for
Save/Open/Take back/retired work and Xiangqi exact warning prefixes. The separate
frontend reviewer independently passed106 tests/eight files and strict TS,
found the Shogi capture/completed-corner visual ambiguity, and reviewed the
actual double-underline correction. Final packaged visual review is pending.

## Verification log and limits

Initial native-consumer test assumptions about Chess MatchStart index,
manual-engine cancellation and Shogi selection token changes failed. They were
corrected against the actual accepted contracts; the three consumer tests then
passed. Clippy found an eight-argument helper; removing its duplicated event
index makes entry identity derive directly from native geometry, and lint then
passed. These were verification findings, not silently hidden passes.

Commands and final source/package revisions, full native/frontend totals,
actual GUI evidence and CI disposition will be recorded below after execution.
No new Windows/Linux/macOS GUI success is claimed by the source checks above.

Pre-package checkpoint: full native desktop-verification suite198/199 passed;
new Chess cancellation test asserted one race winner incorrectly. Chess really
submits its missing-engine process despite the Shogi/Xiangqi manual fixture.
The corrected test accepts only successful cancellation or the exact already-
stopped rejection with retired execution, and asserts unchanged complete native
context, Moves and cue in both paths. Three focused feature-enabled runs passed
all3 consumer checks each. Earlier failed assertions remain recorded; no product
behavior was changed to satisfy them. Frontend264/23, strictTS, native feature
clippy and formatting pass. External GUI-driver75 unit checks pass. Actual
macOS normal package built and instrumentation-exclusion/real bundled-engine
smoke passed; native CUA Chess play/history/flip/Take back observed. Further
packaged GUI and CI checks remain pending.
