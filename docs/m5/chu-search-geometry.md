# Chu search movement and attack tests

The private search position uses a 144-square array, three-word occupancy sets
and complete moves with one or two landings. Movement descriptors supply
piece geometry. Rules remain the authority for root actions and played
moves. The search has no Chess self-check filter.

Generation emits ordinary steps, jumps and rays. A Lion also emits two adjacent
steps with evolving occupancy, including captures on either leg, capture-return
(igui), and quiet returns through an empty square. The Horned Falcon restricts
these compound routes to straight forward; the Soaring Eagle uses either forward
diagonal without turning between legs. These two pieces are not Lions under
the Lion-capture law. Their direct two-square jumps preserve
an intermediate piece. Promotion is a separate choice after the complete route.

Reach tests work from the target displacement in the mover's coordinates.
Steps, jumps and special-piece ranges can answer directly. A sliding attack must
align with one of the piece's ray directions; only the squares between source
and target need an occupancy scan. The target is excluded from that scan because
it can be a friendly piece whose defense is being tested. This same test serves
move validation, royal threats, Lion defense and exchange evaluation.

Geometry alone does not admit a Lion capture. The complete projection vacates
the source, checks revealed defenders, removes the ordered victims and installs
the final piece. A nonadjacent defended-Lion capture needs the qualifying earlier
victim specified by the profile. A captured Pawn or Go-between does not qualify
unless promoted. Active next-turn Lion protection is checked on the complete
route; non-Lion captures can create the next window. Failed admission restores
the board. Undo restores captures, promotion, rights, hash and evaluation caches.

A royal threat requires one admitted action that captures every surviving royal.
With one royal, a direct reach proves a legal single-landing capture: the victim
is not a Lion, no intermediate victim is required, and there is no self-check
filter. This case needs no scratch projection. With King and Prince, the search
still generates and admits a complete action capturing both. Separate attacks
on King and Prince do not qualify. Full generation and perft
retain every route. Search generation first applies game root restrictions,
then retains the first route for each exactly equal successor, including Lion
rights. It checks full state when hashes match.

The shared continuous-check df-pn solver stays disabled for Chu. It proves mate
when the checked defender has no legal reply and excludes positions decided by
`rule_score`. Chu admits replies that ignore a royal threat and handles royal
capture through `rule_score`, so that solver's terminal contract does not fit.
Alpha-beta handles those capture threats. Selectivity margins use one Pawn
(100 evaluation units); captured pieces do not also enter a hand.

For fixed-depth comparisons, run the [strength harness](strength-harness.md)
on frozen game records with fresh engine state. For cost attribution,
measure generation/admission, make/threat checks, unmake, rule/threat
checks and exchange evaluation separately. Compare the instrumented and
uninstrumented decisions and node counts. These coarse categories include
nested calls and timer overhead. Use uninstrumented runs for speed comparisons.
