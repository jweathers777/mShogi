# Tenjiku shared application contract

MSH-79 extends the existing native dispatch, desktop document, controller, worker,
frontend session and choice controller. The rules remain in `mshogi_core::tenjiku`.
The structured verification record is [shared-verification.json](shared-verification.json).
Native rule conformance remains the separately recorded MSH-87 evidence.

New Game takes an explicit `RulesProfile` with the requested game and human side.
The two Tenjiku profiles retain their exact native editions. Presentation side
keys include the selected profile. A different profile requires a fresh document
replacement; current records, observations and branches cannot change profiles.
The normal playable chooser and product Open continue to exclude Tenjiku.
The separately labeled `tenjiku-contract` verification route uses the same
controller and permits explicit profile New and Open only in its QA host.

The existing `mshogi.desktop.document.v1` envelope carries an immutable validated
Tenjiku payload. Strict raw parsing preserves duplicate rejection, and native
replay derives and compares actions, phases, identities, promotion rights,
repetition and conclusions. The payload retains initial/current records,
administrative controls, exact historical selection, observations, retired Take
back branches, settings and retirement floor. A pending request is never saved
as accepted history. Reopen installs fresh authority and remains stopped.

Every mutation uses complete candidate admission on the existing worker before
the sole installation path accepts it. Failed/cancelled replacement leaves the
accepted document, dirty state, runtime authority and prior file intact. Native
one-event Undo remains an API operation; the player control remains Take back.
Close consumes an admitted actual encoding reserve and retires pending work.

Whole-document work is bounded at **160,000,000 units**, independently of the
native 512M record limit and the cooperative **15-second worker deadline**.
Outer bounds remain 16MiB, 4096 retained events, 8192 controls and 256 observations.
Accounting includes retained contexts, prefix reconstruction and installation.
Actual Stop→Resign→Stop paths for all eight actor combinations, with early and
final Close, establish ending reserves including maximal retirement-floor width.
Native-limit records may receive typed application capacity refusal. Elapsed
deadline refusal remains distinct from deterministic work exhaustion and never
concludes a game. Measurements and the rejected provisional 600M policy are
retained in the verification record with their actual scope.

`PresentedChoice` retains an opaque native action. Its narrow Tenjiku tag
distinguishes real travel routes from a stationary capture target. The existing
choice controller uses target steps to select igui without displaying a movement
route. The completed action keeps actual movement steps separate from native
capture, simultaneous burn batch, mover death and promotion phases. Off-route
victims come from native effects. Administrative events have no Moves geometry.
Technical native labels and abbreviations are used until MSH-44.

Snapshots expose both profile and edition, original/promoted piece identities,
surviving royals, selected/live history, native repetition keys, bare-reply state,
draw offers, conclusions and incomplete annotations. Read-only movement queries
bind the exact runtime selection and selected profile. Historical inspection
does not authorize a live action. The frontend rejects same-epoch snapshots or
inspection responses from another profile, while intentional New has a fresh
document epoch.

Execution uses the existing request/context/observation family. Both Tenjiku
profiles advertise no engine-turn, move-analysis or full-reachability capability.
Actual requests produce generic unscored `UnsupportedMode`; there is no Tenjiku
search result, fabricated move, score or principal variation. Saved failures can
be inspected but cannot revive obsolete request authority.

MSH-44 retains functional evaluation/search and supervised complete games,
ordinary product profile choices and Load activation, console/final notation,
licensed kanji/final board presentation, and normal/minimum packaged gameplay on
three platforms. MSH-79 contract fixtures do not establish those obligations.
