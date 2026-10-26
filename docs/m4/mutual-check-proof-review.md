# Mutual continuous-check research review

Reviewed 2026-10-26 for MSH-67 by the independent `mutual_check_review`
research session, with a separate check of the drop reasoning by
`history_fixture_research`. This is a bounded research result, **not an
impossibility certificate, a legal mutual-check example, or an adjudication
policy**. The mutual-check fixture gate remains open.

## Question and required domain

The selected profile asks whether both players can give check on every own
move throughout the same first-to-fourth identical-position interval. An
identical position includes board, ownership, promotion, both hands and turn.
Such a history contains a nonempty closed legal sequence; proving that no such
closed sequence exists would suffice independently of the number of repeats.

The required domain includes every locally admitted live root with standard
piece-stock bounds, one king each, and an unattacked nonmoving king. The moving
king may initially be checked. Reduced material is admitted. Historical
reachability from the normal initial placement is not required. A proof only
about recorded professional games, only about roots reachable from the normal
start, or only about a capture-free sequence would not discharge this gate.

Any proof must handle promoted pieces, captures, demotion into hands, drops,
intervening direct counterchecks, and sources of a discovered check that can
themselves change between different parts of a cycle. Engine repetition
heuristics are not a rules authority for this question.

## Sources inspected

1. YaneuraOu, [2024-01-29 article](https://yaneuraou.yaneu.com/2024/01/29/sennichite-involving-a-reverse-check/),
   final problem paragraph: the author's earlier impossibility work was
   unfinished. Its examples establish occasional reverse checks during one
   player's continuous-check repetition, not both players checking throughout.
2. YaneuraOu, [2024-03-03 announcement](https://yaneuraou.yaneu.com/2024/03/03/proof-of-the-impossibility-of-perpetual-check-in-shogi/):
   links Nanka's proposed proof and requests review. A 2024-10-02 reader comment
   questions the half-plane invariant and handling of drops; the author's
   response directs the reader to the proof author rather than resolving the
   objection. The headline is therefore not treated as a reviewed theorem.
3. Nanka, [linked proof PDF](https://drive.google.com/file/d/112O0pL7sObJCkxaviVEEHNerLvTwpXCD/view),
   *将棋およびチェスにおいて双方連続王手の千日手が成立しないことの証明*,
   third edition dated **2024-03-09**, 16 pages. Download inspected locally:
   `/tmp/mshogi-mutual-check-proof.pdf`.
   SHA-256: `cd5f987133d89a32bd6de8391015354b16d066a97cea7f719b67a5089275e065`.
   PDF pp.2–11 (sections 2–5) were rendered and visually inspected. Japanese
   text extraction was incomplete because of font encoding, so the rendered
   pages governed the review. Page 16 identifies this as edition 3 and lists
   earlier editions; no later edition was found in the targeted search.

The downloaded PDF is research material in a temporary directory, not a
committed project artifact. Its URL and checksum identify the inspected bytes.
The separate chess section is not used as standard-shogi evidence. No inspected
JSA/CSA source supplied a special adjudication for a proven both-checking cycle.
Absence from this bounded search is not proof that none exists elsewhere.

## Reasoning checked and the drop objection

The following are useful intermediate observations, not a complete proof.

- In a closed all-checking sequence the kings cannot move (PDF proposition
  2.2). A king move giving check must discover a ray check: before it, the kings
  lie on that ray. An evading king cannot remain on the ray and still uncover
  it; moving off it leaves the kings neither on a common permitted ray nor
  adjacent. No subsequent checking king move can restore the original king
  positions. Nonmoving pieces can consequently be removed for the paper's
  definition of a piece that "gazes" at the opponent king.
- With fixed kings, a piece that gazes can only make direct checks; moving a
  nongazing piece must discover a check. The paper's closed-sequence arguments
  classify moving pieces into persistent nongazing discovery blockers and
  gazing pieces that may enter hands. Persistent discovery blockers cannot be
  captured. A capture that evades check with a fixed king must capture the
  active checker; capturing an unrelated enemy piece cannot interpose because
  that enemy piece would already have blocked the incoming ray.
- For an interposition drop, the old checker and the new dropped checker lie
  in the same open half-plane bounded by the line through the two fixed kings.
  The old piece ceases checking and the new one starts. The paper's `disc H`
  counts discovery blockers plus **currently checking** pieces in that
  half-plane. Thus the comment's proposed increase by one does not by itself
  refute proposition 3.8: the previous checking contribution must also be
  removed. A drop is not an additional persistent discovery blocker.
- Section 4 does not explicitly define an origin for a drop. It can be
  extended by assigning its virtual origin `D = C` (the destination), taking
  the dropped checker as `P = R`. Then its direct-check term `dang` is zero;
  its interposition term `iang` is strictly negative because the drop lies
  strictly between the old checker and the fixed own king. Capture responses
  give zero for this latter term. The sum of checker-angle changes telescopes
  across a closed cycle. A side using captures only cannot balance its hand
  count; hence the paper's requirement that each side make a discovery check
  remains plausible with this explicit drop extension. The second researcher
  independently checked this local extension.

These observations address the stated drop objection locally. They do not
establish the completeness of the later piece-specific case split. Promotion
also needs the paper's actual piece movement cases: merely showing the angle
formula for an unpromoted rook is insufficient for a dragon or horse.

## Exact remaining proof gap

Section 5 eliminates discovery moves by silver/rook/dragon and then uses the
two-bishop stock limit to reduce the remaining discovery blockers to the two
bishops/horses. Its Table 1 lists signed king/bishop configurations and ends
with a half-plane-invariant contradiction. This review did **not** establish
that this list covers all configurations with intervening direct counterchecks.

A concrete additional candidate from the necessary geometry is:

```text
Enemy king K = (0, 0); own king L = (1, -2).
Own discovery bishop: A = (1, 0), A' = (0, -1).
Enemy discovery bishop: B = (-1, -2), B' = (1, 0) = A.
```

The bishops' possible destinations share a square. They cannot both occupy it
at once and cannot capture a persistent discovery blocker. This fact alone
does not exclude a schedule with direct checks between bishop moves. Table 1
instead pairs the own radius-one bishop with the enemy radius-one bishop, and
the own radius-two bishop with the enemy radius-two bishop. This mixed-radius,
shared-endpoint candidate needs an explicit exclusion argument.

For the half-plane `H = { (x,y): 2*x + y > 0 }`, the three noncolliding bishop
states can all have `disc H = 1`: in the state with both bishops outside `H`,
the active checker can be inside `H`; when one bishop is inside, the active
checker can be outside. Thus the invariant alone does not immediately exclude
this candidate. This is **not** a legal counterexample: no complete closed
legal move history has been established.

Two attempted repairs were not accepted as proofs:

1. Counting one permanently allocated horizontal rook per side overlooks that
   gazing pieces may be captured, dropped, or change ownership between phases.
   Source inventory must be counted at a specified simultaneous position.
2. A shared endpoint can pin a prepositioned enemy vertical source, but that
   source may previously have been captured while checking, or another gazing
   piece may interpose on its ray. Its presence and the absence of another
   blocker must be established, not assumed.

The later invariant contradictions for the listed cases do not themselves
resolve this missing completeness argument. This review therefore does not
claim the PDF is false, nor that its theorem has been independently verified.

## Reproducible necessary-geometry check

The following small Python enumeration was run during review. It is a filter
for potential bishop destination geometry, **not** a legal move generator or an
exhaustive proof. Once both bishops are persistent discovery blockers, other
gazing pieces can attack along orthogonals, adjacent diagonals, or the forward
knight displacement. Promotion of small pieces is included by the adjacent
movement union; a dragon's adjacent diagonals are also included.

```python
def may_attack(a, king, forward):
    dx, dy = king[0] - a[0], king[1] - a[1]
    return (
        ((dx == 0 or dy == 0) and (dx != 0 or dy != 0))
        or (abs(dx) == 1 and abs(dy) == 1)
        or (abs(dx) == 1 and dy == 2 * forward)
    )

K, L = (0, 0), (1, -2)
for radius in range(1, 9):
    for sign in (-1, 1):
        B = (L[0] + sign * radius, L[1])
        B_prime = (L[0], L[1] + radius)
        if any(p[0] * L[1] == p[1] * L[0] for p in (B, B_prime)):
            continue
        if may_attack(B, K, 1) and may_attack(B_prime, K, 1):
            print(B, B_prime)
```

Output:

```text
(0, -2) (1, -1)
(-1, -2) (1, 0)
```

The second line is the shared-endpoint candidate above. Occupancy, actual
checking sources, king safety, hands, and history still need to be resolved.
The finite 9x9 displacement range was used only for this necessary-condition
check. It was not reported as exhaustive state-space search.

## Handoff and implementation consequence

No sourced adjudication for a concrete legal mutual-check cycle and no fully
checked impossibility argument were obtained. The remaining obligation is
specific: establish a sound exclusion of the shared-endpoint discovery-bishop
configurations with all intervening direct-check, capture, drop and promotion
possibilities, or supply a complete legal closed history and an authoritative
adjudication applicable to the selected CSA profile.

Do not choose draw, mover loss, first checker loss, double loss, or rejection as
an inferred game rule. A defensive internal error for a claimed invariant
failure does not resolve the public live-play contract. Preserve unaffected
implementation and ordinary repetition evidence, leave the mutual-check
acceptance gate unfinished, and bring the concrete unresolved scope conflict
to the owner. Neither this document nor passing synthetic history tests closes
MSH-67 or establishes full standard-shogi adjudication conformance.
