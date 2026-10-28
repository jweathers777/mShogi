# Actual Pikafish playing-opponent admission (MSH-100)

This supplements, and does not rewrite, the planning-only
[MSH-80 source audit](../comparator-audit.md) and MSH-91 board-harness evidence.
A network-free move probe is not a playing engine. This task builds and runs
unmodified upstream **Pikafish** with its actual NNUE and restricts admission to
a bounded UCI0-9 opponent, not a WXF2018 outcome oracle.

## Source, build and network

- Upstream: <https://github.com/official-pikafish/Pikafish>.
- Source commit `9d7833aa113626b3ea63ad83f49fb9b456bd5839`, tree
  `93fb0853d2d60dcfbb455b033b94907f4d572f61`; tracked source clean, no patch.
- Actual local build: `make -j4 build ARCH=apple-silicon COMP=clang` in
  `/private/tmp/msh80-research/xiangqi/pikafish/src` on macOS27 arm64, Apple
  clang21 (`clang-2100.3.34.2`). Full actual flags/output: [build log](pikafish-build.txt).
- Binary SHA-256 `78fe97765da9d4c3613bd93a6a4d241e9828fe26669686e3d4a153141822f84c`.
- Required external `pikafish.nnue`, 50,241,164 bytes, SHA-256
  `cd396741e9e0ffd22043216c5f8aed5d8960be6422ddefc4a5bc09a5344fec14`.
  Downloaded by pinned upstream `scripts/net.sh` from
  <https://github.com/official-pikafish/Networks/releases/download/master-net/pikafish.nnue>.
  Actual API provenance: [release metadata](pikafish-network-release.json),
  release111112960, asset684051879, updated2026-10-28T06:21:09Z; upstream asset
  digest matches local SHA-256. The URL is mutable: verify exact hash, never
  assume that a future file with the same name is this network. Large binaries,
  weights and build caches stay outside Git. A compatible weight is required
  for reproduction, and the retained reports identify the exact one used.
- GPLv3-or-later engine notices remain upstream, no opponent/source/weights are
  bundled with mShogi. README130–131 attributes training data to Pika Xiangqi
  Zero under ODbL. That is not a separately established license for this weight
  artifact: no weight redistribution is performed or authorized by this local
  test. Release licensing/distribution remains the existing product boundary.

Primary source locators at the pinned commit:
[`src/uci.cpp`](https://github.com/official-pikafish/Pikafish/blob/9d7833aa113626b3ea63ad83f49fb9b456bd5839/src/uci.cpp)
for actual loop/options/coordinates,
[`src/position.cpp`](https://github.com/official-pikafish/Pikafish/blob/9d7833aa113626b3ea63ad83f49fb9b456bd5839/src/position.cpp)
for input domain and rule policy, and
[`scripts/net.sh`](https://github.com/official-pikafish/Pikafish/blob/9d7833aa113626b3ea63ad83f49fb9b456bd5839/scripts/net.sh)
for network acquisition. The [upstream UCI documentation](https://github.com/official-pikafish/Pikafish/wiki/UCI-%26-Commands)
supports the handshake and complete root+move suffix; source pin and actual
advertisements govern differences from mutable documentation.

## Policy and domain limitations

Pikafish uses a120-ply rule60/check10 policy and search-root-sensitive repetition/
chase outcomes, not selected native WXF claimant-specific100-ply count and
third-warning/fourth-continuation semantics. It exposes no selectable WXF2018
`RepetitionRule` option at this pin. It also has its own material draw helpers.
Do not import any of those engine scores/outcomes as mShogi conclusions.
Native match authority reconstructs all known actions, warnings and claims,
auto-exercises valid claims, and stops before a native concluded/pending root.
A legal engine suggestion is only a suggestion; native admission and cleanup
must succeed before acceptance. Heuristic positive/zero scores do not prove a
natural win/no-win. An external null/missing/invalid reply stays unscored.

The admitted FEN domain is native validated live diagrams with known counters,
root no-capture0..119 and fullmove1..100000. The runner rejects other roots
explicitly without truncating history or normalizing counters. The selected
wire is a–i/0–9, canonical n/b letters, uppercase Red/lowercase Black, w=Red.
No Fairy UCI1–10 or UCCI conversion was implemented or admitted. Unmodified
Pikafish's own repetition policy remains an opponent limitation even though the
complete known board prefix is sent. Native selected-profile records are the
match/replay authority.

Runtime admissions, exact match commands/options/work, source/binary/model
identities and replay results are recorded alongside the reports. Neither
source inspection, the existing board harness, nor these tiny functional
matches establish measured playing strength or universal opponent compatibility.
