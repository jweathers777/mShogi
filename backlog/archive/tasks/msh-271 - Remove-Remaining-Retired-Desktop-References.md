---
id: MSH-271
title: Remove Remaining Retired Desktop References
status: To Do
assignee: []
created_date: '2026-11-25T17:50:00Z'
updated_date: '2026-11-26T11:47:00Z'
labels:
  - cleanup
  - model:small
  - effort:low
milestone: m-6
dependencies: []
priority: low
ordinal: 248000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MSH-248 (Remove Unused Console and Frontend Code) found leftovers outside its list (checked 2026-11-25; check them again before you change them). The verification surfaces that showed a 'DEVELOPMENT CONTRACT' label and a shared-contract-application class are gone from the source, but desktop/scripts/gui-smoke.py and desktop/src/product/ChuContract.test.tsx still assert that they are absent, and docs/m4/desktop-session.md and docs/m4/desktop-archive.md still describe the label. Two rules use custom properties that no style sheet defines: .document-dirty in desktop/src/style.css (var(--muted)) and .moves-events in desktop/src/product/shell.css (var(--shell-line)).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 No test or script asserts only that the retired contract class or label is absent; checks of kept behavior stay.
- [ ] #2 The desktop session and archive documents do not say that a current surface shows a 'DEVELOPMENT CONTRACT' label.
- [ ] #3 No style sheet uses a custom property that is not defined; the document state label stays readable.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John approved consolidation on 2026-11-26. Entire implementation scope, acceptance criteria (updated only for superseded requirements), references and relevant context are now owned by MSH-264. This record is archived as superseded, not Done. No implementation or acceptance check was completed by consolidation. Use the surviving task for future work.
<!-- SECTION:NOTES:END -->
