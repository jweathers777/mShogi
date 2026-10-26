# MSH-64 actual reference admission

On 2026-10-26 the exact upstream cshogi commit
`6985fe5b516c34072e27c63cf429847f03193522` was built and executed in an isolated
Python environment on macOS arm64. The 101 independently drawn, color-reversed
board fixtures passed, including promotion/nonpromotion, capture demotion and
hands, all drop types, empty dead ranks, nifu, pawn-drop mate and legal king or
piece defenses, pinned defenders, single/double check, check-evasion promotion,
king-origin x-rays, geometric pinned-piece influence, and nested capture/drop
reversal. The independent review is recorded in
[reference-review.md](../reference-review.md).

This is bounded board-reference admission, not complete shogi delivery or a
complete instantiated SH-F001–055 inventory. Sparse positions are analysis
diagrams; historical reachability is not certified. Match outcomes, clocks,
declarations and fourfold/perpetual-check adjudication remain MSH-67. No native
mShogi/reference comparison was run for MSH-64; the JSONL probe contract in the
[tool guide](../../../tools/shogi/README.md) enables MSH-66.

## Board-inventory coverage

| Inventory IDs | Instantiated evidence |
| --- | --- |
| SH-F001 | Independently named exact 30-choice initial set. |
| SH-F002–004 | Focused knight-jump/rook-blocker choices; promoted gold movement; initial blocked paths. Full per-type central/edge/blocker movement matrix remains MSH-66 coverage work. |
| SH-F005–009 | Both sides, optional/compulsory promotion, entering/inside/leaving camp, captures and check evasions. |
| SH-F010 | Capture of each promoted kind demotes into the capturer's hand. |
| SH-F011–014 | Every hand drop kind, repeated-piece uniqueness, dead-rank boundaries, nifu/promoted/enemy pawn contrasts, absent-resource/occupied/wrong-owner/same-square exclusions. F014 board exclusion is covered; native atomic request rejection is MSH-65/66 product evidence. |
| SH-F015–017 | Pawn-drop mate; king escape/capture, defender capture and pinned-defender contrasts; existing-pawn and gold-drop mate remain board actions. |
| SH-F018–022 | Pins, single/double-check defenses, defended-checker capture, origin x-ray and geometric pinned attacker, nonpromotion during check evasion. |
| SH-F023–024 | Exact reference snapshots around transitions and nested capture→demotion→drop history; final SFEN independently specified. Native exact undo is separate product evidence. |

These are reviewed concrete assertions within each family, not a claim that
every pending inventory subcase is instantiated or that reference agreement
alone completes MSH-66. SH-F025–042 history/outcomes remain MSH-67; later adapter,
engine and application rows retain their assigned work.

## Durable evidence

- [cshogi-admission.json](cshogi-admission.json): exact source revision, nine
  audited source/license byte hashes, installed package origin, Python/build
  dependency versions, actual compiler commands, successful build-log hash and
  extension hash. The extension SHA-256 is
  `903a5af71398c687fda7ed3bdddcdc2595e2c28461dac43f67c7b896b82c37ad`.
- [rule-source-manifest.json](rule-source-manifest.json): ten downloaded JSA,
  CSA-36 and USI primary sources with URLs, UTC retrieval dates, byte lengths,
  HTTP metadata and SHA-256. Raw bytes are retained in the isolated local
  `/private/tmp/msh64-rule-sources` archive; they are not redistributed here.
  Fingerprints identify the retrieved bytes and do not assert a future live
  URL will retain those bytes.
- [board-rules.json](../../../tests/fixtures/shogi/board-rules.json): independent
  expected facts, derivations and review provenance, separate from observations.
- [reference-run.json](reference-run.json): all actual fixture observations,
  four API-gap sentinels and four deliberate mismatch/failure demonstrations,
  bound to the input fixture and admission-manifest hashes.

The source's root LICENSE is GPLv3; inherited C++ headers state GPLv3-or-later.
The checkout retains those notices. cshogi credits Apery/Stockfish and includes
YaneuraOu adaptations, so agreement with another related engine is not a second
independent rule authority. This process is development-only and does not copy,
link or bundle that implementation into the product.

## Reproduce the source build

The existing source archive was cloned from
[TadaoYamaoka/cshogi](https://github.com/TadaoYamaoka/cshogi) and its clean tracked
HEAD checked against the immutable pin. Generated Cython/build files are
untracked; staged and unstaged tracked changes are both forbidden at admission.

```sh
git clone https://github.com/TadaoYamaoka/cshogi.git /private/tmp/msh62-cshogi-audit
git -C /private/tmp/msh62-cshogi-audit checkout 6985fe5b516c34072e27c63cf429847f03193522
python3 -m venv /private/tmp/msh64-reference-venv
/private/tmp/msh64-reference-venv/bin/python -m pip install \
  Cython==3.2.4 numpy==2.3.5 setuptools==80.9.0 wheel==0.45.1
```

The host Python was 3.11.0. Apple clang21 rejected upstream's constexpr enum
conversion at `src/square.hpp:290`; its upstream
`-Wno-enum-constexpr-conversion` option was unrecognized. A second attempt with
`-fno-strict-enums -Wno-invalid-constexpr` also failed. These are failed build
attempts, not reference admissions. No cshogi source was changed to fix them.

The successful compiler was Homebrew LLVM19.1.7, downloaded and extracted only
into temporary storage (no global compiler installation). The immutable bottle
is [the Homebrew OCI blob](https://ghcr.io/v2/homebrew/core/llvm/19/blobs/sha256:e7865a7b30dcc44fd36a2e61259de7cb8d92e1a042ad7ff3ed8928a82b6cb64f),
SHA-256 `e7865a7b30dcc44fd36a2e61259de7cb8d92e1a042ad7ff3ed8928a82b6cb64f`.
At execution, Homebrew's fetch command selected its arm64 Golden Gate bottle.

```sh
HOMEBREW_CACHE=/private/tmp/msh64-brew-cache HOMEBREW_NO_AUTO_UPDATE=1 \
  /opt/homebrew/bin/brew fetch llvm@19
mkdir -p /private/tmp/msh64-llvm19
tar -xzf /private/tmp/msh64-brew-cache/downloads/*--llvm@19--19.1.7.arm64_golden_gate.bottle.3.tar.gz \
  -C /private/tmp/msh64-llvm19
/usr/bin/install_name_tool \
  -change '@@HOMEBREW_PREFIX@@/opt/zstd/lib/libzstd.1.dylib' \
  /opt/homebrew/opt/zstd/lib/libzstd.1.dylib \
  /private/tmp/msh64-llvm19/llvm@19/19.1.7/lib/libLLVM.dylib
/usr/bin/codesign -s - -f /private/tmp/msh64-llvm19/llvm@19/19.1.7/lib/libLLVM.dylib
```

That install-name relocation fixes the extracted compiler's Homebrew placeholder;
it changes no reference source or rule logic. zstd1.5.7 was already installed.
The extracted `bin/clang-19` SHA-256 is
`0ed94991417a606632497fc94eef2bc3eca29b6e13365f92e555ade286ca3083`;
the relocated/re-signed `lib/libLLVM.dylib` SHA-256 is
`31782bf6f17811852b055665eb928d622e1661422493b0ba5df291497053af56`.
One intermediate LLVM attempt failed C/C++ header ordering because the host
Python's default flags inserted SDK C headers ahead of libc++; explicit flags
below fixed it. Another compiled every source but failed linker subprocess
loading until the compiler relocation above. Only the final clean build was
admitted.

```sh
CC=/private/tmp/msh64-llvm19/llvm@19/19.1.7/bin/clang \
CXX=/private/tmp/msh64-llvm19/llvm@19/19.1.7/bin/clang++ \
CXXFLAGS='-O3 -DNDEBUG -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk' \
CFLAGS='-O3 -DNDEBUG -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk' \
/private/tmp/msh64-reference-venv/bin/python -m pip install -v \
  --force-reinstall --no-build-isolation --no-deps /private/tmp/msh62-cshogi-audit \
  > /private/tmp/msh64-reference-build.log 2>&1
```

Before the successful final build, only this worker's generated temporary
`/private/tmp/msh62-cshogi-audit/build` directory was removed to force compilation
of every source. The final successful log hash is
`55a82fa1fb8414e2e0ebb6abc5e455154ed668c4cc7774728e2c26f553c9799a`.
It records `-std=c++17 -mcpu=apple-m1`, the upstream feature macros, and no
`BAN_BLACK_REPETITION` or `BAN_WHITE_REPETITION` definition. Their upstream
source branches in `src/ifdef.hpp` are disabled. Full actual compiler/linker
commands are preserved in the admission manifest. Build output and binaries
remain external to Git.

## Actual verification and intentional failures

From the repository worktree:

```sh
CXX=/private/tmp/msh64-llvm19/llvm@19/19.1.7/bin/clang++ \
/private/tmp/msh64-reference-venv/bin/python tools/shogi/reference.py \
  --admit-source /private/tmp/msh62-cshogi-audit \
  --build-log /private/tmp/msh64-reference-build.log \
  --diagnostics-demo --report docs/m4/reference/reference-run.json
```

The final run after independent review metadata/coverage updates omitted only
`--admit-source`/`--build-log`, preserving the already admitted binary manifest:
**101 fixtures passed, 0 failed; 4 API-gap sentinels; 4 diagnostic demonstrations**.
Each ordinary run requires all fixture and gap assertions to pass. The sentinels
observed permissive parser/`is_legal` acceptance of an illegal empty-final-rank
pawn drop and compulsory-promotion omission, while LegalAll excluded both.
After one king-shuffle cycle, `is_draw()` already returned REPETITION_DRAW at
only two known root occurrences. After three cycles LegalAll still enumerated
board choices. Those observations are explicitly excluded from outcome-oracle
use.

Diagnostic test doubles intentionally injected missing `1g1f`, extra `P*5e`,
exit17 and truncated JSON. Each was detected at ply2 of seed20261026 path
`2h1h 8b7b 2g2f 7b5b 1h5h`, reduced to prefix `2h1h 8b7b`. Root, original path,
seed, exact failing SFEN and distinct missing/extra/transport diagnostics are
retained. These expected injected failures are not native mShogi failures.

A separate actual staged-dirty-source admission sentinel cloned the same pin
into `/private/tmp/msh64-dirty-source`, appended a marker to README.rst, staged
it with `git add`, then invoked `--admit-source` with a temporary output manifest.
It exited1 with `FAIL: reference source must be the exact clean pinned tracked
revision`; no manifest was produced. This validates the index-aware clean-source
guard, rather than silently admitting staged edits.
