# Tori comparator audit — source only

Independent worker `tori_comparator_audit`,2026-11-01; root sole integrator.
Inspected actual [Fairy-Stockfish source53c1f2bd14a2d3b685904ccf8ec9dec8401800ff](https://github.com/fairy-stockfish/Fairy-Stockfish/tree/53c1f2bd14a2d3b685904ccf8ec9dec8401800ff),
commit dated2026-10-30, isolated public checkout under `/private/tmp`.
GPLv3-or-later according to source headers/Copying.txt. No source copied into
product, no engine build/execution, and no comparator admission in this task.

| Actual source boundary | Finding / proposed future scope |
| --- | --- |
| src/variant.cpp945–982 |7×7 initial setup, distinct quails, jumpers and limited Eagle rays agree. |
| variant.cpp966–972; movegen.cpp306–332; position.cpp1357–1364 | Mandatory promotion when origin OR destination in farthest2ranks; compatible with zone-drop departure. |
| position.cpp1643–1657 | Capture changes ownership, demotes, preserves handed base kind. |
| position.h708–723; position.cpp1111–1113 | Own unpromoted Swallow file count/drop cap2 and immobile drop exclusion. |
| position.cpp1264–1299; variant.cpp977; search.cpp196–199 | Royal safety; mate/stalemate loss. These are source observations, not executed conformance. |
| variant.cpp978–981; position.cpp2652–2720 | **Mismatch:** nFoldRule4, nFoldValue=VALUE_MATE/perpetualCheckIllegal. No selected third-occurrence departure obligation. Exclude history/outcomes. |
| movegen.cpp515–526; position.h960–970; pyffish.cpp221 | **Mismatch:** raw Legal move list includes forbidden Swallow-drop mates; engine handles them by reversed terminal scoring. It is not an authoritative complete legal-action set for this profile. |
| position.cpp650–667 | Hash includes board, turn, hands, but no selected obligation/history. |
| tests/perft.sh143 | Upstream Tori depth4=103857 assertion exists; neither executed nor independently established here. Do not convert it into an expected profile result. |

[PyChess FAQ](https://www.pychess.org/faq) states Fairy-Stockfish move validation;
its [Tori page](https://www.pychess.org/variants/torishogi) says third occurrence,
so documentation/runtime convention differs. It is not an independent second
reference. H.G.Muller's Shokidoki description distinguishes another repetition
interpretation/check priority; no executable Shokidoki version audited/admitted.

A later native fixture owner may build this pinned candidate and admit **scoped
board comparisons only**, after mappings and independent sentinels for quails,
blockers/leaps, both owners, promotion, demotion, file limits, drop-mate defenses
and pins. Exclude history/terminal behavior; explicitly normalize/filter drop
mates only with independently tested adapter or exclude those cases. A reference
mismatch is investigated against the selected rule and hand-reviewed position,
not resolved by editing expected data to match either engine.

Actual audited file SHA256:

```text
variant.cpp  0e835a90bb2c2d27b03e55ef164092927d68084087cdad19ccd29ba9981058d4
movegen.cpp  0ec1dd919ee6778f690030dc78dce9b18fded040089904b55ea37f9fb781b748
position.cpp 16f28fecb31f9d68fcd6010e3d231aea9c08182db10edf66ed232964ecc684fc
position.h   afa87108128a1562a02878cb6dbd8ae3667af338c46d207ef774ae5bad3da43e
piece.cpp    6695601cb82b230aab5bd3f8f7422ff8be1ab05163f80bc374ec8137e23399ef
```
