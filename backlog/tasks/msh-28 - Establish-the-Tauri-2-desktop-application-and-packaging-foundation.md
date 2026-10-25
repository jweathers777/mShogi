---
id: MSH-28
title: Establish the Tauri 2 desktop application and packaging foundation
status: Done
assignee:
  - '@codex'
created_date: '2026-10-25T12:48:00Z'
updated_date: '2026-10-25T13:15:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-27
documentation:
  - docs/desktop/m3-design.md
ordinal: 28000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The desktop slice needs a reproducible native application using the accepted stack and a bundled engine without exposing shell execution to the webview.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Tauri 2, React, strict TypeScript, Vite and SVG application sources are reproducibly locked and documented, with development and packaged launch commands.
- [x] #2 Normal native packages bundle the mShogi engine and web assets; test drivers and fixture executables are excluded, and frontend shell access is absent.
- [x] #3 Applicable native CI builds packages and exercises the packaged engine on Windows, macOS and Linux; local running desktop screenshots and package evidence are recorded.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Create a separately locked Tauri desktop project with React/strict TypeScript/Vite; bundle the target-native engine without frontend shell access; add native package CI and launch documentation, then verify packages and actual desktop launch.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner codex + /root/desktop_foundation; branch codex/m3-desktop; checkout .claude/worktrees/next-task. MSH-27 contract independently reviewed and integrated.

Local macOS arm64: npm run build (strict TypeScript + Vite), desktop Cargo1.90 fmt/clippy --locked --all-targets -D warnings/test --locked, and actual .app package build passed. Packaged engine UCI depth1 completed21nodes/bestmovea2a3; SHA d987f826ef87f7d004a1a9604f4b63e0bd8ac171fdff3fe862bb62a4e93352c0. Package inventory contains only plist, host, engine and icon. Actual native screenshot docs/desktop/evidence/foundation-macos.png; external smoke report /tmp/msh-desktop-foundation-smoke.json. Windows/Linux package CI and independent review pending.

Independent foundation review /root/m3_contract_review found no actionable issues across toolchains, manifests/locks, permissions, package preparation, CI and launch instructions. Native screenshot confirms rendered packaged window. Independent Python verifier syntax check passed.

Integrated source074fe515b9c9b9ab65dfc9a256dcbf0d07eb3132 on codex/rewrite. Desktop package CI https://github.com/jweathers777/mShogi/actions/runs/40196021909 passed all3 jobs: Windows2022 121176902007, macOS15 121176902044, Ubuntu24.04 121176902086. Actual .app, .deb extraction and NSIS install contents checked; their actual bundled engine completed a bounded UCI search. Artifacts desktop-<os> include package and package-smoke.json inventories/engine SHA. Full installed-app interactions remain MSH-38.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Established locked Tauri2/React/strictTS/Vite desktop with bundled engine, clear launch commands and isolated Rust1.90 host toolchain. Independent review, local actual native screenshot and all3 native package builds/packaged-engine smokes passed. No test instrumentation or fixture executables are bundled.
<!-- SECTION:FINAL_SUMMARY:END -->
