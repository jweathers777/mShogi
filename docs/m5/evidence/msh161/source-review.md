# Independent source review — MSH-161 (Competitive Gameplay Diagnosis)

Reviewer: execution_review agent, independent of source selector author. Date:
2026-11-10. Scope: candidate-source provenance, selection and coverage; no engines,
production modifications or playing-strength judgments. Initial source snapshot is
**suitable for candidate preparation, not yet cleared for main-study selection**.

## Evidence examined

Initial candidate gzip SHA256:
`9ce186924ee8326a9b8aea97b15e5683b26a996b6efda37a5c82ea2ecbebf62d`.
Selector SHA256:
`a0c56d99d3fa70c735f155725979c5af369018cb61b87f196470e5e42fc8d8ae`.
Manifest SHA256:
`8eed962f8f7a5e492837c51449cb43ab6e86bbba3eaf14d8ee7e841ec48076e8`.

Read selector, validator, README, manifest, source-validation and pinned-validation
records. Independently read the [PGN Mentor download page](https://www.pgnmentor.com/files.html)
and the [pinned Lichess source README](https://raw.githubusercontent.com/lichess-org/chess-openings/3ba6b20d31b2e6125cb0e6a2e792f400717d9d39/README.md).
The former offers free downloads, without the CC0 declaration found in the latter.
The source README correctly keeps those permission scopes distinct and retains
only factual selected moves/identification, not annotations or whole downloaded
archives. This review does not assert an open-data license for PGN Mentor.

Offline checks used `/private/tmp/msh139/venv/bin/python`, python-chess1.11.2,
macOS arm64. Independent scripts loaded the gzip and raw archives read-only:

- All330 complete source-game move lists replay legally; every16-ply prefix
  matches its stored FEN and independently recomputed board/side/rights/legal-EP
  key. All330 keys are distinct.
- All22 raw file hashes in the manifest match local downloaded bytes.
- Reparsed all six source archives and matched all330 selected records against
  their exact archive ordinal, headers and complete move lists.
- Independently counted structural coverage: first four candidates per family
  gives88 roots but only **one** exactly mirror-symmetric pawn structure; the
  entire330-candidate reserve contains only three.

No source game result, mShogi choice or reference judgment enters the selector.
Deterministic SHA ordering and explicit structure buckets are inspectable.
Board-based deduplication deliberately chooses one history from transpositions;
it does not erase the selected full history. Archive-occurrence counts are
correctly disclaimed as popularity estimates. The six elite repertoires, rating
filters, fixed eight-move prefix and structural oversampling make this a purposive
coverage survey, not a random population sample.

## Findings requiring resolution before main-sample freeze

1. **Symmetry coverage is too thin in the proposed first-four allocation.** The
   existing closure/castling buckets do not directly protect symmetry, pawn
   tension, genuinely open versus closed centers, or tactical/quiet coverage.
   Define structural targets and deterministic replacement/gap-fill rules before
   looking at mShogi results. A single locked c–f pawn pair does not by itself
   establish a closed center. Root-level structural descriptions need human
   verification where indicators are ambiguous.
2. **The Sicilian group name omits included Richter–Rauzer roots.** B62/B63
   examples are in `sicilian-najdorf-dragon-scheveningen`. Rename or remap before
   family-level reporting; the preserved recognized opening names themselves
   correctly identify Richter–Rauzer.
3. **Soundness is unestablished.** Legal master-game histories and ECO membership
   do not establish balanced positions. Apply the frozen independent balance
   screen and preserve ordered rejection/replacement reasons. Native admission
   remains required before gameplay; Python replay is diagnostic validation.

The parent has authorized a source-only revision addressing findings1–2 while
preserving the initial snapshot. Resolution and revised identities remain pending
at this initial review. These findings do not block the separate unaided-start
opponent qualification batch. No main-study gameplay conclusion is approved.

## V2 resolution and independent recheck

Source-only revision completed before main-sample freeze and without engine-result
selection. Initial v1 candidate/manifest/selector/validation artifacts remain in
`sources/v1-*`. Revised candidate SHA256:
`6781434f8f8f1202c6984499d5f2ab6e40a1cbc2f481a119672d42db9b9b462d`;
manifest `ed7f3b36dcd3038024651d9b924da5767c2736ba3d03f4f36ce97b83ed2703b0`;
selector `1d13899fb091082e367178fe5c0d29d680d0ef6f9662a48b4c8b9fb9b85f9084`.

Independently rechecked all330 revised source archive ordinals, exact headers and
complete move lists (skipping unselected PGNs using `chess.pgn.read_headers`),
all330 legal full histories and prefix FENs, and recomputed pawn symmetry and
center indicators directly from boards. All pass. The initial88 roots now have
18 mirror-symmetric structures,10 opposite-wing castling arrangements,17 closed,
28 open and43 mixed centers. The center labels match the documented operational
board indicators on all330 candidates. Quotas are source-based, deterministic and
fixed before reference screening; greedy selection meets these chosen minima in
the first four candidates per family. Richter–Rauzer is now included in the
Sicilian group name.

Findings1–2 are resolved for candidate preparation. Final balance-cleared selection
must preserve coverage minima when replacing rejected roots and independently
review tension, development and tactical/quiet variety; these are not proved by
castling/center counts. Finding3 remains an intentional main-execution gate.
**No source-preparation blocker remains to the unattended reference screening.**
