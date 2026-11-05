# Modern Tenjiku Shogi profile family

MSH-83 ratifies two owner-selected whole profiles. MSH-87 supplies the native
implementation and executable conformance described below. MSH-44 subsequently
adds the engine, console and desktop consumer delivery; the earlier native
handoff remains evidence for its own bounded scope.

- [Complete rules and immutable identities](profile.md)
- [Owner decisions and profile differences](decisions.md)
- [Source reconciliation and historical qualifications](sources.md)
- [156-piece setup/44-face data](profile.json)
- [Independent two-profile corpus](fixtures.json) and[accounting](fixtures.md)
- [Native API, records, limits and consumer contract](native-v1.md)
- [Shared application contract](shared-v1.md) and [shared verification](shared-verification.json)
- [Direct native/shared/product handoff](handoff.md)
- [Validation and independent review evidence](verification.json)

Profile1 follows Adams1999 Modern; profile2 follows Muller's stated differences
with explicit Adams general-game supplements and rare-edge software conventions.
Earlier[historical research](../research/tenjiku.md) remains evidence, not an
acquisition gate or proof of agreement among historical and modern witnesses.

Native source/acceptance evidence is recorded in `native-verification.json`;
MSH-79 consumes the actual handoff;
MSH-44 now implements normal sixth-game selection with two explicit whole-profile
choices and ordinary profile-preserving Load: see
[engine](engine-v1.md), [notation/console](notation.md), [artwork](desktop-art.md),
[original-criteria coverage](product-coverage.md), the single
[product evidence record](product-verification.json) and
[normal macOS gallery](evidence/msh44-final-macos/gallery.md). The
[earlier QA gallery](evidence/msh44-macos/gallery.md) retains its checkpoint scope.
On 5 November the owner approved the traditional-kanji artwork and authorized
normal activation. The shared footer identifies the selected native piece across
all six games, including compound entry; capture outlines paint above occupied
pieces. The minimum remains 760×650: the board fits, but small kanji and secondary
route badges are not all comfortably readable. About 960×1000 is recommended,
not required. See the [recorded owner decision](../../design.md#owner-acceptance-of-tenjiku-visuals-and-shared-identification-2026-11-05).

The delivered normal product is verified at source
`eca9f9cfd43cf83ff4e3489292f6c25f8951bb30`: all six required Rust/Desktop CI jobs
passed on Linux, macOS and Windows. Linux and Windows each passed all 47 actual
installed normal/QA GUI phases, including both Tenjiku profiles. Genuine local
normal macOS journeys include both profiles and native royal-capture complete
games; the final rebuilt package is byte-identical to the captured package.
The original eight criteria and applicable DoD have complete technical evidence;
the single product record carries exact receipts and final acceptance disposition.
The earlier QA gallery retains its original source and scope.

The next eligible task is MSH-88. Standard-root tests establish bounded smoke,
while limited native diagrams establish complete games; competitive playing
strength remains MSH-53/M5 work.
