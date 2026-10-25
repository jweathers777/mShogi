---
id: MSH-58
title: Resolve release policy and complete three-platform product distribution
status: To Do
assignee: []
created_date: '2026-10-25T15:39:00Z'
labels: []
milestone: m-6
dependencies:
  - MSH-47
documentation:
  - docs/design.md
ordinal: 58000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The accepted polished desktop product needs product-quality distribution beyond M3/M4 packaging evidence. This task owns remaining release/distribution gaps and D-10 decisions, preserving existing notices. It does not assume a licensing, minimum-OS, signing or updater choice has already been made.

Open D-10 questions: release licensing and redistribution permissions, supported minimum OS policy, signing and update arrangements, and what constitutes a distributable supported release. Record applicable decisions before shipping; use SQLite only if concrete local persistence requirements warrant it, not as an automatic new subsystem.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Release licensing, third-party assets/dependencies/models and redistribution obligations are reviewed with provenance/notices preserved; minimum OS support and signing/update arrangements have recorded decisions. Unresolved applicable policy blocks the affected release claim.
- [ ] #2 Required build, packaging, installation/launch, bundled engine/model/resource discovery and agreed update/uninstall/recovery workflows are documented and reproducible on Windows, macOS and Linux for the supported six-game product. No private configuration or development-only fixture is required by a normal installation.
- [ ] #3 Product-quality/distribution gaps discovered in release verification are tracked and resolved, and user-facing supported coverage/limitations and installation/recovery guidance match actual behavior. Native installed evidence and independent review identify the exact release revision and environment rather than inferring cross-platform success.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
