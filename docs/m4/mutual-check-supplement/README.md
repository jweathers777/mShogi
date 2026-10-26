# Proposed MSH-67 mutual-check proof supplement

Prepared 2026-10-26 for John Weathers and the mShogi rewrite.

## Result and status

This package supplies a concrete exclusion of the mixed-radius, shared-endpoint
bishop configuration identified in the repository's research review. Two
independent AI mathematics reviews checked the new argument. Two separately
authored necessary-geometry enumerations agree on the revised candidate list.

The new argument uses knight hand balance and a monotone count of rooks on a
specified ray. It permits arbitrary intervening direct checks, captures, drops,
promotion and ownership changes. It does not assume that a side permanently
owns particular checking sources.

The package is a proposed supplement for integration and fresh review within
MSH-67. Its mathematical conclusion is conditional on the preceding proof
reductions. It is not a machine-checked formal proof, a complete search of shogi
states, a legal mutual-check example, or a new adjudication policy. No repository
files or ticket statuses were changed during this investigation.

## Files

- `mixed-radius-lemma.md` contains the complete new exclusion argument and its
  explicit assumptions, including the side-exchanged symmetry.
- `case-accounting.md` documents the candidate inventory, other exclusions,
  proof-integration checks and review scope.
- `directed_geometry.py` enumerates directed candidate walks and their cycles.
- `independent_geometry.py` independently enumerates necessary candidate edges.
- `verify_geometry.py` compares both outputs, checks the displayed case
  accounting, and checks the two mixed cases' symmetry.
- `geometry-result.json` is the retained execution result from this investigation.
- `SHA256SUMS` identifies the package's text and script bytes.

The scripts use only the Python standard library. From this directory, run

```sh
python3 verify_geometry.py
```

Expected result is agreement on 23 pairings after horizontal reflection, across
20 king offsets. Including the reflected placements gives 46 pairings. Every
retained directed cycle has two endpoints for each bishop.

Those counts establish a conditional finite geometry calculation. They do not
mean that 46 legal game positions or mutual-check cycles were found.

## How the missing case is excluded

Use the review's coordinates. Side T's king is K=(0,0), side S's king is
L=(1,-2), S's discovery bishop moves between a=(1,0) and c=(0,-1), and T's
discovery bishop moves between b=(-1,-2) and a. S moves forward in positive y.

1. T's return from a to b must capture a checking S knight. The square b lies on
   no sliding line through K, and only an unpromoted knight checks K from b.
2. This capture increases T's knight hand count. A closed cycle restores both
   hands, so T must eventually drop a knight.
3. Every move checks. T's checking knight drop can only go to n=(2,0); its other
   possible checking square is occupied by T's own king.
4. That drop interposes an S rook or dragon's check. S must capture the knight,
   taking a rook or dragon off the ray R={(x,0) where x>2}.
5. No S checking evasion can ever arrive on R. Its squares cannot capture or
   interpose a checking non-bishop piece relative to L. The only possible long
   diagonal would require an active bishop or horse, while both bishops are
   already the persistent discovery blockers.

Thus the number of S rooks or dragons on R never increases and must decrease.
It cannot return to its original value. The proposed closed cycle is impossible.
The full lemma treats all alternative dragon origins and the required earlier
gaze-classification assumptions explicitly.

## Recommended next action

Resume the preserved implementation at
`f259bb23834d6156f21aeec53db9ee00ef7242e4` on `codex/shogi-history`, reconciling
the research handoff at `9325c4802467c83b5ebdb525bc194e9e4280f612` without
discarding existing work.

Incorporate the supplement and corrected case accounting into the repository's
research record. Have a fresh reviewer check the complete chain from the earlier
reductions through the new case, including drops, promotions, identical-piece
permutations, and the admitted live-root domain. Reuse the already completed
implementation and fixture reviews for unchanged code.

If that review accepts the argument, resolve HIST-INTERVAL, update the provisional
mutual-case diagnostic to an explicitly internal invariant-failure guard, and
complete MSH-67's normal verification, integration and evidence. No invented game
outcome is needed for a state excluded by the accepted invariant. Preserve
diagnostics if implementation corruption ever violates that invariant.

If a specific mathematical step is rejected, retain the counterargument and
continue that exact investigation. Do not replace the selected profile with a
tie-break or silently mark the unresolved case complete.

## Sources and provenance

The reviewed repository handoff is
[9325c48](https://github.com/jweathers777/mShogi/blob/9325c4802467c83b5ebdb525bc194e9e4280f612/docs/m4/mutual-check-proof-review.md).

The preceding proof is Nanka's third edition, dated 2024-03-09, available at
[the author's linked PDF](https://drive.google.com/file/d/112O0pL7sObJCkxaviVEEHNerLvTwpXCD/view).
The inspected PDF SHA-256 is
`cd5f987133d89a32bd6de8391015354b16d066a97cea7f719b67a5089275e065`.
The PDF is not redistributed in this package. The relevant sections are the
fixed-king and gaze reductions, the half-plane invariant, the necessity of
discovered checks on both sides, and section 5's piece/configuration reduction.

The [author announcement](https://yaneuraou.yaneu.com/2024/03/03/proof-of-the-impossibility-of-perpetual-check-in-shogi/)
is evidence of the proposed proof and its review history; it is not used as a
substitute for checking the argument.

The unchanged governing rules remain
[CSA WCSC36](https://www.computer-shogi.org/wcsc36/rule.pdf) and the selected
[JSA movement rules](https://www.shogi.or.jp/match/taikyoku_rules/taikyoku_rules.pdf).
This supplement reasons about the impossibility of a board/history pattern. It
does not supply an additional competition outcome.
