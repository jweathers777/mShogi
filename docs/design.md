# mShogi design

This document holds the accepted product requirements (P), architectural
boundaries (A) and decisions (D). How agents work is in
[AGENTS.md](../AGENTS.md); engine-strength work follows the
[strength roadmap](m5/strength-roadmap.md). Earlier dated handoffs and
milestone narratives are in Git history.

`main` is the integration branch. `master`, `main_v2` and the annotated
`legacy-cpp` tag preserve the earlier implementations. Mini Shogi, legacy save
import and the old search and Force controls are outside current requirements.

## 1. Product requirements

| ID | Requirement |
| --- | --- |
| P-01 | Provide reusable, complete implementations of at least chess, shogi, tori shogi, chu shogi, tenjiku shogi, and xiangqi. |
| P-02 | Develop mShogi's own engines that play well, with measurable progress and game-specific strength criteria. |
| P-03 | Provide a console application for interactive use, control by other interfaces, self-play, and matches against other programs. |
| P-04 | Provide a polished desktop application supporting every implemented game on Windows, macOS, and Linux. |
| P-05 | Make engine behavior inspectable and failures reproducible. Verification, diagnostics, and performance/strength measurements are development infrastructure. |
| P-06 | John makes design and product decisions. Any coding agent that follows AGENTS.md (for example Claude Code or Codex) implements them. Durable instructions and Backlog.md work records live in Git. |

Deferred requirements remain requirements. A partial implementation must state
its coverage and must not present itself as complete.

The product runs locally. A hosted service, mobile application, public online
play, a user-defined variant language and a distributed training platform are
outside the delivery plan.

### Variant completeness

mShogi is a game-playing engine, not a historical reconstruction project.
Delivering a game means a clearly identified, coherent, complete playable rules
profile; independently verified rules and history; a functional mShogi engine;
and integration into the application. Historical research resolves material
gameplay uncertainty; acquiring every historical edition is not a requirement.
Distinguish sourced rules from necessary software conventions. A material
conflict in ordinary play or outcomes needs an explicit decision; a coherent
convention for a rare edge case may be selected with distinguishing fixtures
and honest attribution. Uncommon adjudication must have explicit semantics.
Expensive optional analysis or exact proofs do not become delivery gates by
themselves.

Selected profiles: Standard Shogi [CSA36](m4/shogi-profile.md) (D-13),
Xiangqi [WXF2018](m4/xiangqi/profile-proposal.md) (D-15), modern
[Tori](m4/tori/profile.md), modern JCSA [Chu](m4/chu/profile.md) (D-20), and
the two Tenjiku profiles [Adams 1999 and Muller](m4/tenjiku/profile.md)
(D-21). Chu uses capture-based admission, finite immediate royal-threat mate,
must-vary admission before the fourth repetition, and narrow material
exhaustion with actual replies or agreed draws; it adds no Chess self-check,
unrestricted forced-mate search, repetition forfeit or general bare-royal award
([decisions](m4/chu/decisions.md)).

## 2. Technology

| Layer | Direction |
| --- | --- |
| Rules, positions, search, console | Rust and Cargo |
| Desktop host | Tauri 2 |
| Desktop frontend | React, strict TypeScript, Vite |
| Board and surrounding interface | SVG board presentation with HTML/CSS controls |
| Local collections and metadata | SQLite when persistence requirements warrant it |
| Learning experiments | Python/PyTorch, with bindings to the same Rust rules implementation |
| Search and learning path | Conventional search first, then learned evaluation; NNUE-style evaluation is the initial candidate |
| Later search research | Policy/value models and MCTS stay a separate research path |
| Tracking | Repository-resident Backlog.md records |

Add a dependency when its component is implemented, not before.

## 3. Architectural boundaries

### A-01. One authoritative rules implementation

The rules library owns legal complete actions, action resolution, promotion
choices, turn progression, and rule-dependent outcomes. Search, console, GUI,
match runner, and training tools use it. The frontend manages selection and
presentation and must not contain a second implementation of legality.
Training code must not implement a separate approximation of the game.

#### Terminology

| Term | Meaning |
| --- | --- |
| Rules library | `mshogi-core`, which owns the legal actions, history and outcomes for each game. The `game` module supplies shared dispatch for the six games. |
| Game action | One complete board choice. A game command can also request a non-move operation such as resignation or a draw claim. |
| Game record | A serialized initial state and command history under a named rules profile. A canonical record is the game-tagged envelope for a record. |
| Tagged side | A side paired with its game, for example Chess White or Xiangqi Red. |
| Game adapter | The per-game implementation of a shared consumer interface. A game session owns the current game and its runtime state. |
| Session snapshot | The current view of a desktop game session, including its token, selection, status and capabilities. |
| Engine result and observation | The result is the engine's response to an execution request. An observation pairs the result with the exact request context. |
| Internal execution | mShogi's structured search interface. Console interoperability adapters have their own protocol names. |
| Native | Use for platform concepts such as a native OS dialog, window or menu. |

### A-02. Explicit state and complete actions

An action is one complete player choice for one turn. It can require multiple
movement steps or distinguish routes with the same start and end squares.
Drops, promotions, capture choices and other rule-dependent options must be
representable.

The rules implementation determines captures and other effects from the
player's action; a client-provided victim list is not authoritative.
Resolution can contain ordered internal effects while appearing atomic to
callers. Rejecting an invalid external action leaves the state unchanged.
Partial UI selection is not a committed partial turn. A match command such as
resignation or a draw claim is not a board move.

Rust types, action encoding, board representation and trait structure are
implementation decisions. These requirements do not need a universal event
interpreter or one board representation for all games.

### A-03. Position, history, and match state

Specify the boundary between the current position, the history the rules need,
and match state such as clocks and player assignments. A board diagram alone
may not determine an outcome.

Saved-state identity, repetition identity and search-cache identity are
separate concepts. A result that depends on history must not be reused only
because a board key matches.

This rule governs the real game, its saved state and its adjudication. An
engine's private analysis can key its transposition table on the board state
alone and detect repetition and move-count rules on the search path, as
classical engines do. Such analysis can be inexact in rare history-dependent
cases. It never decides a real game result; the rules library does (2026-11-13).

An undo record restores every affected field and incremental cache. Public
saved-game records must not depend on the engine's private undo layout.

### A-04. Ruleset identity and honest coverage

Every rules profile has an explicit identity and version. Record authoritative
references, consequential interpretations and unresolved cases. Do not treat an
implementation shortcut as the rules of a game. Legality and outcomes belong to
the profile; do not assume one royal piece or chess check semantics for every
variant. Distinguish automatic outcomes from claimable outcomes.

### A-05. Search and interface isolation

Search uses private working state or equivalent isolation. A search request
and every result identify the request and its base position and version.
Superseded results cannot change the current game. Cancellation must not
corrupt the game or block behind unbounded progress messages.

The desktop application supervises the engine executable for
substantial search work. Quick rules queries may call the rules library in
process. Move generation, evaluation and search run in Rust without
frontend or Python round trips.

The console stays independently usable. Human interaction and machine protocol
modes have distinct output contracts. Diagnostics must not corrupt protocol
stdout.

### A-06. Interoperability and persistence

Use standard engine adapters where appropriate (UCI, USI, CECP/XBoard, and
UCCI when a selected opponent needs it). Keep a structured mShogi interface for
complete internal functionality.

The canonical game record preserves the rules profile, the initial state and
necessary history, and complete actions. Imported and exported notation is an
adapter. Do not force every game into a chess-specific notation.

External engines serve as independent opponents, reference implementations
where appropriate, and optional analysis providers. They do not replace P-01 or
P-02, and they are never rules oracles or ordinary build, test or CI
dependencies ([conformance policy](testing/conformance-policy.md)).

### A-07. Presentation

The desktop UI accommodates large boards, square and intersection geometry,
multi-stage action entry, promotions, different piece sets, readable labels,
keyboard play and useful textual game records. Engine progress and expensive
work stay off the UI event path. Validate the real packaged application on all
three target systems.

- **Movement inspection** (MSH-101): the visible status is exactly
  **`Inspecting {side} {piece} · Esc clears`**. The inspection label
  supplies game-specific side and piece terms; a shared formatter owns the
  surrounding words. Non-actionable inspection, distinct hollow markers,
  accessible read-only announcements and keyboard/Escape behavior stay
  required.
- **Pieces** (MSH-42): traditional, variant-appropriate kanji are the default
  for every shogi-family game, using the variant's own glyphs and
  bundled licensed glyph assets. Bird artwork may stay as an inactive
  alternative.
- **Console diagrams** follow the
  [compact convention](console-presentation.md): case-based ownership for
  Chess, Standard Shogi and Xiangqi, aligned Shogi promotion markers, and
  lowercase side prefixes for multi-character abbreviations.
- **Xiangqi** shows intersections, river, palaces and its own pieces, with the
  whole board visible at supported desktop sizes without scrolling or zooming.

<a id="owner-acceptance-of-tenjiku-visuals-and-shared-identification-2026-11-05"></a>

- **Piece identification and capture marks** (2026-11-05): selecting a
  playable friendly piece shows its piece name in the board footer in
  all six games, including during compound selection. Legal capture indicators
  paint above occupied destination pieces as outlined overlays; movement
  inspection stays distinct.
- **Window size**: the application minimum is 760×650. The whole Tenjiku board
  fits there, though not every kanji or route badge is comfortably readable;
  about 960×1000 is the recommended playing size, not a minimum.

### A-08. Learning is a separately measured track

Establish correct rules, a search baseline and reproducible matches before
substantial training. Training and deployed inference agree on ruleset,
feature encoding and model identity. Start with separate game-specific
evaluators and shared experiment infrastructure; architectures, features,
quantization and compute budgets are empirical decisions. Python stays outside
the shipped application unless a later requirement justifies it; bindings and
data generation reuse the product rules.

### A-09. Pre-release internal compatibility

mShogi is an unreleased rewrite (MSH-115, 2026-10-30). Internal Rust APIs, IPC,
hooks, controllers, adapters, document schemas and their interface tests are
project-controlled. Breaking or deleting obsolete interfaces, archive readers
and compatibility layers is acceptable when it gives a cleaner architecture;
earlier existence or passing tests are not reasons to keep them. Re-express
behavioral and independently expected fixtures against the replacement; retire
tests whose only purpose is a removed interface.

Preserve the functional requirements: correct complete game semantics;
authoritative game histories, adjudication and atomic operations; safe
execution, cancellation and stale-result rejection; save and reopen of the game
(rules profile, start position, every command, sides and settings);
cross-game product consistency; required external protocol interoperability
(UCI, USI and others actually required); independent rules conformance. A
changed encoding never authorizes changing a selected rules profile or losing
the game's own history.

A saved document holds the game, not an audit trail (MSH-228, John
2026-11-22): no journal of desktop changes, no copies of earlier records, no
branches removed by Take back or Undo, no runtime identities, and no engine
results (they stay in memory while the game is open). Keep only data that the
product uses.

### A-10. Application architecture and shared product UI

One application/runtime controller, exactly one active game-tagged document,
one internal execution contract
([internal-execution-v4](desktop/internal-execution-v4.md)), one desktop
envelope with game payloads, one frontend session/operation owner and one
shared board interaction controller (2026-10-30). Rules, actions,
histories, adjudication, search and justified optimized structures stay
independent per game.

There is **one product interface**, not separate applications under common
chrome. The application owns layout and visual hierarchy, the dark responsive
composition, document and lifecycle controls, player/turn/status,
analysis/history/actions, technical details, dialogs, notices, errors and
recovery, common language and control placement, and shared keyboard, focus and
accessibility conventions. Common controls stay in the same slots for every
game; game-specific content (Chess claims, Shogi declarations, Xiangqi
warnings) composes into those sections without its own layout.

Game renderers supply topology, geometry, coordinates and decorations; piece
art, typography and orientation; hands and other genuine board-adjacent
content; and promotion, compound-route and specialized completion views. One
common choice interaction owns selection, cancellation, inspection and
completion. This does not require a universal board renderer, rules
model or frontend rules interpreter.

### A-11. Desktop UX

(2026-10-31)

- Sidebar sections in this order: **Analysis → Game Actions → Moves**.
  Diagnostics and required notices are reachable through a nonintrusive
  support/About entry, outside the game sidebar.
- New Game shows a compact dropdown of **implemented playable games only**,
  with the existing side buttons and White/Black, Sente/Gote, Red/Black
  and Random labels. The application brand uses the Shogi selector character
  (歩). No implementation narration; keep meaningful uncertainty, result,
  capacity and recovery information.
- Moves are complete board actions in sourced game-appropriate notation,
  in two player columns. Warnings, claims and other non-move events stay
  in the history, attached to the command that caused them, not as extra rows.
  History stops only at command boundaries (MSH-226). The last
  completed action highlights its origin and a stronger destination (drops:
  destination only), also during thinking and history browsing.
- Document actions are Save and Load; the first Save chooses a path; there is
  no ordinary Save As. New Game and Load replace the document without an
  unsaved-changes warning, **only on valid atomic success**; cancellation,
  failure and stale admission keep the current document. Quit with unsaved
  changes asks Save / Discard / Cancel ([macOS Quit](desktop/macos-quit.md)).
- Resign submits directly and shows the game conclusion or unscored
  uncertainty. One-step Return to live restores play suspended only by
  browsing, with a fresh admission check; play stopped by Load, Take back,
  cancellation or failure stays explicitly stopped (Resume or Retry).
- [Take back](desktop/take-back.md) restores human decision points
  atomically by cutting the game's command list, and retires abandoned
  execution; restored play needs an explicit Resume.

## 4. Milestones

| Milestone | Outcome | State |
| --- | --- | --- |
| M0–M3 | Bootstrap; Chess rules kernel and console; search and match baseline with UCI; desktop vertical slice | Done |
| M4 | All six games with sourced profiles, conformance, mShogi engines, records and desktop integration (MSH-41–46, closure MSH-47) | Done |
| M5 | Per-game classical engine strength (MSH-48–54), initial learned evaluator (MSH-55), per-game opening-knowledge experiment (MSH-104); closure MSH-56 | In progress ([roadmap](m5/strength-roadmap.md)) |
| M6 | Required: analysis, accessibility and product polish (MSH-57), release policy and three-platform distribution (MSH-58); closure MSH-60. Optional: policy/value search and learning research (MSH-59) | Not started |

A milestone closes only when its complete promised scope, required task
criteria and integrated verification have an explicit evidence-based verdict
in its closure task. A Backlog percentage is not that verdict. Do not erase a
gap by renaming scope, archiving unfinished required work or moving it into
optional research. For MSH-104, adopting an opening mechanism is optional; an
evidence-supported per-game non-adoption is a valid result.

## 5. Engineering principles

- Generalize when several real consumers show the same semantics, not merely
  similar-looking code. Examine both duplication and premature abstraction.
- Measure before setting performance targets. A speedup is not stronger play;
  perft is not evidence of correct search, adjudication, strength or GUI
  behavior; generated tests must not be the only source of expected rule
  behavior.

| Evidence | What it establishes |
| --- | --- |
| Curated rules fixtures | A selected interpretation is implemented correctly in a specific case. |
| State properties | Transitions, reversibility, derived state and serialization stay consistent. |
| Perft and differential comparison | Move generation agrees with independent expectations over specified positions and depths. |
| Search oracle tests | Search returns the intended result on independently solvable positions or trees. |
| Performance benchmarks | The effect of a change on time, throughput or memory under recorded conditions. |
| Match experiments | Playing performance for a game, opponent pool, limits and starting positions. |
| Application integration | Commands, progress, cancellation, state changes, persistence and presentation stay coherent. |

- Search: test small independently solvable positions or synthetic trees before
  relying on pruning or transposition reuse; test score perspective, terminal
  scoring and returned-action legality.
- Matches: preserve executable and model identity, parameters, limits, openings,
  colors, results and failures. Pair openings with reversed colors.
- Desktop: test starting a search, changing the position, cancelling and
  receiving a delayed result, plus persistence and compound action entry, in
  the real packaged application.

Each game's conformance must be verifiable from its selected profile and
independently established expectations, without another engine
([conformance policy](testing/conformance-policy.md)). Where and how often
checks run is in [AGENTS.md](../AGENTS.md#checks).

## 6. Future direction: larger historical Shogi variants

Deferred; needs John's authorization before any work (D-22). Provisional order
and boards: Dai Shogi (15×15), Dai Dai Shogi (17×17), Maka Dai Dai Shogi
(19×19), Tai Shogi (25×25), and conditionally Taikyoku Shogi (36×36). Do not add
them to the game selector before a playable, verified implementation.

When authorized, start from the shared shell and interaction controller. Up to
about 16×16 keep the full-board view; 17×17 to 19×19 investigate a maximized
window, responsive layout and optional zoom and pan; 25×25 and larger
investigate a full-board overview with a magnified, navigable playing region,
accessible zoom/pan and keyboard controls, legal destinations outside
the detailed view, and preserved complete-action identities. Each game owns its
rules; shared infrastructure owns session, persistence, execution and
presentation; no universal variant-rules framework. Each such game needs a
named, versioned playing profile, independently expected fixtures,
rules and complete actions, a functional engine, desktop integration and
platform verification.

## 7. Decision register

| ID | State | Decision |
| --- | --- | --- |
| D-01 | Accepted | All six named games, own capable engines, console interoperability, and three desktop platforms. |
| D-02 | Accepted | Rust core and console; Tauri 2 with React/TypeScript/Vite desktop; rules reused by learning tools (2026-10-22). |
| D-03 | Accepted, revised 2026-11-21 | John decides design and product; any coding agent that follows AGENTS.md implements; Backlog.md in the repository. Replaces the 2026-10-22 role of ChatGPT for design. |
| D-04 | Superseded | The agent workflow is in AGENTS.md (revised 2026-11-20). |
| D-05 | Done | Rewrite in the existing repository, preserving legacy history (`master`, `main_v2`, `legacy-cpp`). |
| D-06 | Done | M0/M1 sequencing. |
| D-07 | Open per game | Disputed historical rules, versioned conventions and full conformance criteria are resolved per game before affected behavior is implemented or conformance is claimed. |
| D-08 | Implementer choice | Public API/ABI, board and action encodings, trait hierarchy and wire/save schemas: implement only the needed stable semantic contracts. |
| D-09 | Accepted, 2026-11-18 | Engine strength follows the per-game classical [strength program](m5/strength-roadmap.md), then learned evaluation (MSH-55, MSH-135). Fast predecessor game tests have standing authorization; long campaigns and rating studies need approval. |
| D-10 | Deferred | Release licensing, redistribution, minimum OS policy, signing and updates (MSH-58). Preserve existing notices; resolve before copying restricted dependencies or distributing releases. |
| D-11 | Accepted, 2026-10-24 | Chess match semantics: optional core claims with a recorded unattended auto-claim policy, FEN-limited known history, explicit unscored mating uncertainty, untimed and increment clocks, ordered replayable match events, evaluation adjudication disabled. See [match v1](rules/chess-match-v1.md). |
| D-12 | Done | Standard Shogi first, then Xiangqi, Tori, Chu and Tenjiku. |
| D-13 | Accepted, 2026-10-26 | Standard Shogi profile `mshogi.shogi.csa36.v1` (SHOGI-P1), with untimed local play, advisory/committed declarations, atomic rejection and declared-counter/bounded-history adaptations. JSA professional and other event conventions are unselected alternatives. See [profile](m4/shogi-profile.md). |
| D-14 | Accepted, 2026-10-27 | Every game uses the common application shell (A-10). |
| D-15 | Accepted, 2026-10-27 | Xiangqi profile `mshogi.xiangqi.wxf2018.v1` (XIANGQI-P1): official bilingual WXF2018 with per-case discrepancy reconciliation and no blanket language precedence; third-occurrence warning and fourth continued unilateral loss, or the applicable WXF draw; untimed claims; Article 3.2A by sound fast recognition only (John removed the exact analysis on 2026-11-25, MSH-269). See [profile](m4/xiangqi/profile-proposal.md). |
| D-16 | Accepted, 2026-10-27 | MSH-104 per-game opening-knowledge experiment is part of M5 closure; adoption is optional. A compact teacher-distilled opening policy, the learned evaluator (MSH-55) and full policy/value search (MSH-59) are distinct. No product Python and no mandatory opening mechanism. |
| D-17 | Accepted, 2026-10-30 | Pre-release compatibility policy: see A-09. |
| D-18 | Accepted, 2026-10-30 | Application architecture and shared product UI: see A-10. |
| D-19 | Accepted, 2026-10-31 | Desktop UX: see A-11. |
| D-20 | Accepted, 2026-11-02 | Chu profile `mshogi.chu.jcsa2019.v1` (modern JCSA): capture-based movement and finite immediate-threat mate; the responsible side must vary before the fourth repetition; narrow sourced material exhaustion, immediate response and agreement-based draw. No automatic forfeit, automatic draw or quiet-immobility loss. See [decisions](m4/chu/decisions.md). |
| D-21 | Accepted, 2026-11-04 | Tenjiku offers two immutable whole profiles, Adams 1999 Modern and Muller, chosen at New Game and preserved by records, replay and search. No independent rule toggles or in-place reinterpretation of a game. See [decisions](m4/tenjiku/decisions.md). |
| D-22 | Deferred direction, 2026-11-05 | Larger historical variants (section 6); nothing approved beyond recording the direction. |

Deferred questions block only the work that depends on them. A routine choice
within accepted requirements needs no new approval.

## 8. References

- Original project: https://github.com/jweathers777/mShogi
- Rule fixtures carry their own primary rule references. This design is not a
  substitute for the rules of any game.
