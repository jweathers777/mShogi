# Selected Xiangqi planning dossier — MSH-80

John selected **`mshogi.xiangqi.wxf2018.v1`**, XIANGQI-P1, on2026-10-27.
The governing publication is official **bilingual WXF2018**, with individually
recorded interpretations and no blanket Chinese precedence. This is selected
planning, not implemented Xiangqi support. MSH-41/MSH-75 and M2/M3 remain Done;
MSH-42–47 and M4 remain open.

- [Owner decision record](owner-decisions.md) and [selected profile](profile-proposal.md)
- [Current coverage and affected-work gates](coverage.md)
- [Source reconciliation/editions](sources.md) and [acquisition hashes](source-manifest.json)
- [Independent fixture expectations](fixtures.md), preserving the44-action set and F002
- [Consumer/task mapping and first queue](handoff.md)
- [Current resolution review and verification](resolution-review.md)
- [Audited comparator candidates](comparator-audit.md) and [source hashes](comparator-source-manifest.json)

MSH-80 closes only after reviewed planning criteria and normal integration are
supported;91/92 then become Ready. No native rules, comparator build/admission,
console/engine/UI work or training is implemented in this batch. Residual
G-CYCLE/G-NOWIN questions gate affected94/105 work rather than the settled first
board/state queue. Required exact3.2A delivery105 gates84/103/45, preserving full
M4 completeness. AXF/CXA acquisition remains scoped to concrete questions.

Historical unselected proposal and actual prior reviews are preserved:
[proposal](profile-proposal-before-xiangqi-p1.md),
[primary-source review](source-independent-review.md),
[first proposal review](proposal-source-review.md),
[first architecture review](architecture-plan-review.md),
[first integration/validation](planning-review.md).
Those records describe the earlier unfinished state at14b9643/f4e564b/a2cf50b;
the owner decisions above supersede their conflicting recommendations without
rewriting the historical evidence.

The separately approved future [M5 opening experiment](../../m5/opening-knowledge.md)
is MSH-104, including a first-class dedicated teacher-distilled opening policy.
No model/book adoption or Python product runtime follows from task creation.

Independent partial [Tori](../research/tori.md), [Chu](../research/chu.md) and
[Tenjiku](../research/tenjiku.md) research remains unfinished and independent.
Provisional delivery order stays Xiangqi → Tori → Chu → Tenjiku.
