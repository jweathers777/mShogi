# Tori shared semantic presentation — MSH-132

Current MSH-42 native follow-up: [native playable checkpoint](native-playable-checkpoint.md)
now supplies real engine turns/analysis, native limits/observations and the shared
Rust action notation. The [desktop visual checkpoint](desktop-visual-checkpoint.md)
now renders the production board and traditional kanji. Owner visual approval,
normal product exposure and final acceptance remain pending.
The source52b0390 descriptions below preserve the accepted MSH-132 checkpoint,
including its then-unsupported execution and pending notation; those two facets
are superseded by the linked native implementation evidence.

Production source `52b0390cac09faf727a0c5ed74c1fd673f468a11` extends the
[accepted runtime](desktop-runtime.md), using the selected `mshogi.tori.modern.v1`
profile/edition `MSH81-MODERN-2026-11-01`. This is a real native consumer contract,
not complete playable Tori delivery. [Verification and handoff](shared-presentation-evidence.md).

## Native authority and concrete payload

`SharedSnapshot.tori` contains typed Tori root provenance and selected/live
history facts: event index, known board ply, check, known occurrences, native
repetition departure obligation, conclusion and administrative incomplete reason.
The declared root counter is a decimal string, preserving every native u64 value
through JavaScript without pretending it is known history. Diagrams and imported
roots do not acquire historical reachability or earlier occurrences by display.

The existing board descriptor supplies 7×7 squares, native coordinates and both
Sente/Gote orientations. The nine readable face labels preserve Phoenix, Falcon,
Eagle (promoted Falcon), Crane, Pheasant, Left Quail, Right Quail, Swallow and
Goose (promoted Swallow). Board ownership and qualified hand origins remain
native; captured and dropped quails never exchange handed identities.

Live complete choices come from `Game::legal_actions`, inspection from the exact
selected prefix's immutable local query. Comparison with the native source face
adds a completion label only when the admitted resulting face changes. There is
no optional-promotion menu, movement table, legality calculation or declaration
borrowed from Shogi. An already promoted piece moving again is not a new promotion.
Historical snapshots grant no board-action capability or actionable choices.

## One frontend and complete actions

The tagged TypeScript snapshot, native side/settings/operation and resource
vocabulary now includes Tori. `PlayableGameKind` deliberately excludes it.
The existing `ProductApp`, `useGameSession`, `ApplicationView`, `GameFrame`,
`NativeBoard`, `useMovementInspection` and `useNativeChoiceController` remain the
owners. There is no Tori page, separate hook, second choice controller, worker or
application lifecycle. Parsing the opaque JSON action only embeds that exact
native complete action in the native operation; display text grants no authority.

Pointer and keyboard board/hand choices, Escape, focus restoration and duplicate
submission protection use the existing interaction lifetime. Native capability
refusal cannot be overridden with `disabled=false`. Inspection has its separate
read-only permission, full native token, permanent reply retirement and distinct
non-actionable markers. The shared wording remains
`Inspecting {side} {piece} · Esc clears`; richer accessibility announcements stay.

## Exact history and cues

The Rust Moves projection consumes the admitted event ledger, emitting only Play
rows. Native event count and board ply stay separate. Stop and resignation remain
Position events, never fabricated moves or extra move numbers. Player columns
use the actual actor, including a Gote-first imported root and Play→Stop→Play.
The existing projection cache selects native completed geometry at the exact
prefix: board moves have origin and emphasized destination; drops have a qualified
hand origin and destination-only board cue. Historical root clears the cue.
No geometry or history is reconstructed from the displayed label.

Labels explicitly say **Technical** and **not final Tori notation**. This does
not satisfy the C-NOTATION assertion/consumer requirement, both owned by MSH-42.
Selected and live conclusions/obligations/provenance remain distinct. Take back
uses the existing native decision boundary and preserves abandoned lineage and
observations; selected future facts cannot leak into an earlier prefix.

## Truthful status, failures and limits

Typed native conclusions distinguish checkmate, no-legal-action loss, repetition
forfeit and resignation. A repetition obligation restricts native departures but does not itself conclude
the game.
Administrative Stop retains Cancelled/ResourceLimit/EngineFailure/WorkLimit as
unscored native reasons. Operational refusals preserve accepted content and do
not become illegal moves, native outcomes or fabricated search scores.

Tori advertises no engine turn, move analysis, Chess Full or Xiangqi exact
capability. Even a genuine verification transport request pending completion
cannot appear as actual implemented Tori thinking, analysis, work limits or
telemetry. Its real UnsupportedMode observation stays unscored technical evidence;
there is no Retry/Resume invitation to nonexistent engine work.

Resource UI consumes authoritative typed facts. Structural byte/event/control/
observation/replay limits and reserved endings remain unchanged. The shared
ValidationBudget explanation now states that accepted content was kept and that
Save/Load themselves require bounded validation; it does not promise a deadline-
refused Save will succeed or advise reopening to reset structural limits.
The accepted 15-second off-path deadline, whole-document charge and individual
bounded noninterruptible native replay remain unchanged. See the earlier actual
[measurements](desktop-runtime-evidence.md), not injected classifier fixtures,
for performance evidence. This is a concrete supported-workload qualification for
MSH-77 acceptance, not a new replay redesign or a change of capacity constants.

## Product and verification exposure

Normal New Game still lists only Chess, Shogi and Xiangqi. Normal host New Game,
Replace and Load explicitly refuse Tori. Actual native codec/runtime access is
not product playability. MSH-42 owns functional search, console, final notation,
Tori-specific artwork/board/hand presentation and complete product journeys.
The accepted common application shell and full-board-visible product requirement
remain binding; the diagnostic technical renderer is not a visual design target.

Two external QA selectors, `tori-standard` and `tori-contract`, use the existing
stable QA application identity and same ProductApp/controller. They alone permit
native Tori contract commands and Save/Load in that verification build. No extra
bundle identity, app page or rules implementation is introduced. Normal packages
exclude selector/bootstrap, native fixture activation and generated fixture data;
shared Tori semantic dispatch code legitimately remains in the shared frontend.
The fixture's technical lettering and diagnostic hands are deliberately unfinished.
