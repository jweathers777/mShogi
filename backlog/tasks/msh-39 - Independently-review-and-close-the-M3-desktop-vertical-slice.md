---
id: MSH-39
title: Independently review and close the M3 desktop vertical slice
status: Done
assignee:
  - '@codex'
created_date: '2026-10-25T12:48:00Z'
updated_date: '2026-10-25T21:56:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-27
  - MSH-28
  - MSH-29
  - MSH-30
  - MSH-31
  - MSH-32
  - MSH-33
  - MSH-34
  - MSH-35
  - MSH-36
  - MSH-37
  - MSH-38
  - MSH-61
documentation:
  - docs/desktop/m3-design.md
ordinal: 39000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
M3 needs a focused integration verdict against its complete deliverables while reusing unchanged M1/M2 evidence.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 An independent integration review verifies all eleven M3 areas against actual application, authoritative session/engine integration, records, cancellation and three-platform packaged evidence.
- [x] #2 All required M3 tickets satisfy their acceptance criteria; material findings are resolved and limitations/follow-up obligations are accurately recorded without repeating M1/M2 audits.
- [x] #3 M3 closes only after its complete scope and normal integration/native CI pass; the milestone verdict and next eligible work are recorded with durable evidence.
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
Review exact integrated source475e3b1 against all eleven M3 work areas and fully checked tickets27-38 plus61, reusing unchanged M1/M2 evidence. Commission a fresh independent reviewer of native session/result-purpose/stale acceptance, bounded engine/Full/cancellation, complete archives/admission/reopen identities, shared board/selection and packaged GUI evidence on all3 systems. Produce a durable area-by-area closure matrix with exact revisions, focused/native/frontend/GUI distinctions, known limits and next M4 planning boundary. Resolve material findings without weakening criteria. Verify product-path equality with actual GUI-tested2f0e4cee and final integrated native CI, update current design/launch/status documentation without altering historical records or future ToDo scope, then integrate/push normally and close only when all gates pass.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner authorizes closure without another procedural approval only when all eleven M3 areas, persistence-capacity follow-up MSH-61, actual three-platform packaged GUI evidence, independent review and final-source/native CI are satisfied. Reuse unchanged M1/M2 and prior M3 evidence; preserve roadmap correction and leave M4–M6 future To Do. Required unavailable checks remain explicit blockers; do not weaken criteria.

Owner @codex on codex/m3-desktop (.claude/worktrees/next-task); MSH38 is Done and normally integrated/pushed475e3b1a2d81db81175891f07da61db7806d6589. All actual GUI/native product source equals tested2f0e4cee; MSH38 completion changed only task/docs/evidence. M2 remains closed; MSH40 roadmap correction and completed tickets preserved. MSH41-60 remain future ToDo; no variant implementation authorized in this batch. Fresh independent MSH39 integration review requested before milestone verdict.

Fresh independent m3_final_review PASS at475e3b1: all eleven work areas and dependency AC reviewed, no material finding. Independently passed46 desktop-session feature tests,22 protocol/real-process tests, strict TypeScript and78 frontend tests; recomputed Linux48/Windows49/Mac final3/capacity3/current7/status3 retained hashes, exact history-prefix/capacity archives and all12 native guard→host/engine absence sequences. Durable report docs/review/m3-independent-review.md; root eleven-area closure matrix docs/review/m3-closure.md. Proportionate current design/README/launch-document review PASS, roadmap/MSH26Done and futureToDo boundary intact. Integrated475 RustCI40235926930 all3PASS; Desktop40235927049 still running, not claimed passed. Closure documentation and evidence are ready for normal integration; final status remains In Review until applicable integrated CI completes. No product paths changed after GUI-tested2f0e4cee.

Final gate PASS: normally integrated/pushed source996568b161eef47655cb80dafb57e5b43b488d6c. Rust40236534250 and Desktop40236534235 completed SUCCESS on Ubuntu24.04, Windows2022 and macOS15; root independently confirmed exactSHA/all6 job verdicts. Independent final-CI artifact assessment below verifies all75 Linux and65 Windows original file hashes, all12 GUI phases with specific guard and host/engine absence BEFORE teardown, canonical history/capacity records, exact package/driver identities and normal instrumentation exclusion. Actual macOS GUI remains separately sourced27.0.1arm64; CI15 is package/protocol evidence. Prior reviewed475e3b1 Rust40235926930/Desktop40235927049 also completed all3SUCCESS; superseded1e Desktop40236378631 completed all3SUCCESS, not substituted for final gate. Fresh reviewer confirmed zero product/build-manifest diff2f0e4cee→475→1e→996 and preserved unrelated prototype cleanup; builder Cargo exclusion difference is explicitly qualified. Documentation links/diff checks and local normal bundle hashes pass. Backlog39 completion is the only remaining bookkeeping change and does not alter tested product or workflow. M3 CLOSED; M2 remains closed, roadmap correction preserved, all41-60 remain futureToDo, no next dependency-ready Ready task. Stop at milestone boundary.

Final independent CI artifact assessment (retained here to preserve exact current-run provenance):
```json
{
  "reviewer": "capacity_review",
  "checked_at_utc": "2026-10-25T21:55:08.954398+00:00",
  "source_revision": "996568b161eef47655cb80dafb57e5b43b488d6c",
  "run_id": 40236534235,
  "run_url": "https://github.com/jweathers777/mShogi/actions/runs/40236534235",
  "status": "passed",
  "scope": "Final Desktop CI and original artifact report verification. No repository files edited or binary artifacts retained.",
  "jobs": [
    {
      "os": "ubuntu-24.04",
      "job_id": 121299212620,
      "job_url": "https://github.com/jweathers777/mShogi/actions/runs/40236534235/job/121299212620",
      "status": "passed",
      "artifact": {
        "id": 12615082638,
        "name": "desktop-ubuntu-24.04",
        "url": "https://github.com/jweathers777/mShogi/actions/runs/40236534235/artifacts/12615082638",
        "sha256": "d049289c2671cd0d15bf3a9acba386f906a6e33494c83ad15bfee7eae8305319",
        "github_digest_matched": true
      },
      "normal_package": {
        "report_sha256": "39ad0aeaaac2d31042f3ee34682e9a575055b07998103d12829dfad545903ad3",
        "manifest_source": "996568b161eef47655cb80dafb57e5b43b488d6c",
        "host_sha256": "f68157bb18ffba2610882f913d30019ec284989d884ba8576728e8362a247e07",
        "engine_sha256": "129a6af24ffaf5833d763caf7589c1c651b9628719f70defd8a8bd9f0ec11799",
        "inventory_excludes_instrumentation_names": true,
        "frontend_exclusion_log_occurrences": 1,
        "host_marker_scan": "Successful package-smoke report generation follows actual native-host verification-marker assertions.",
        "bundled_protocol_queries": "UCI, engine-turn, move-analysis and bounded Full reports present and successful."
      },
      "gui": {
        "report_sha256": "499285fcac6fbe9ae44746af1057f0495fc1e2da1adc0cde18d4f96d5f3a2ff1",
        "original_file_hashes_verified": 75,
        "phases": [
          {
            "phase": "normal",
            "status": "passed",
            "specific_guard_dispatched": true,
            "host_absent_before_teardown": true,
            "engine_absent_before_teardown": true,
            "failure_cleanup_substituted": false
          },
          {
            "phase": "verification-standard",
            "status": "passed",
            "specific_guard_dispatched": true,
            "host_absent_before_teardown": true,
            "engine_absent_before_teardown": true,
            "failure_cleanup_substituted": false
          },
          {
            "phase": "verification-promotion",
            "status": "passed",
            "specific_guard_dispatched": true,
            "host_absent_before_teardown": true,
            "engine_absent_before_teardown": true,
            "failure_cleanup_substituted": false
          },
          {
            "phase": "verification-claim",
            "status": "passed",
            "specific_guard_dispatched": true,
            "host_absent_before_teardown": true,
            "engine_absent_before_teardown": true,
            "failure_cleanup_substituted": false
          },
          {
            "phase": "verification-capacity",
            "status": "passed",
            "specific_guard_dispatched": true,
            "host_absent_before_teardown": true,
            "engine_absent_before_teardown": true,
            "failure_cleanup_substituted": false
          },
          {
            "phase": "verification-synthetic",
            "status": "passed",
            "specific_guard_dispatched": true,
            "host_absent_before_teardown": true,
            "engine_absent_before_teardown": true,
            "failure_cleanup_substituted": false
          }
        ],
        "environment": {
          "platform": "Linux-6.17.0-1022-azure-x86_64-with-glibc2.39",
          "python": "3.12.3",
          "runner_image": "20261023.334.3",
          "runner_os": "Linux",
          "renderer_environment": {
            "GDK_BACKEND": "x11",
            "WEBKIT_DISABLE_COMPOSITING_MODE": "1",
            "WEBKIT_DISABLE_DMABUF_RENDERER": null,
            "LIBGL_ALWAYS_SOFTWARE": null
          }
        },
        "canonical_archives": "One-event historical analysis prefix and all 256 capacity observations retained."
      }
    },
    {
      "os": "windows-2022",
      "job_id": 121299212338,
      "job_url": "https://github.com/jweathers777/mShogi/actions/runs/40236534235/job/121299212338",
      "status": "passed",
      "artifact": {
        "id": 12614978391,
        "name": "desktop-windows-2022",
        "url": "https://github.com/jweathers777/mShogi/actions/runs/40236534235/artifacts/12614978391",
        "sha256": "d5030e5fa81443de5ba3e6c65035f0e220122f8158f3ad6832226417718285bb",
        "github_digest_matched": true
      },
      "normal_package": {
        "report_sha256": "ea03bfa9d732df6e892bb9a3fd24d8f1b2ff2087c48a36568a4b1e66c86efc01",
        "manifest_source": "996568b161eef47655cb80dafb57e5b43b488d6c",
        "host_sha256": "61554be9139b0a9c16f561f42cc7a54f0c83c16609fcc813c3db26f77cacd80b",
        "engine_sha256": "3477df67e1ff3b8bb966cd4dca5fd616f9ede4d5c780c38f0496a3dc05b19ef9",
        "inventory_excludes_instrumentation_names": true,
        "frontend_exclusion_log_occurrences": 1,
        "host_marker_scan": "Successful package-smoke report generation follows actual native-host verification-marker assertions.",
        "bundled_protocol_queries": "UCI, engine-turn, move-analysis and bounded Full reports present and successful."
      },
      "gui": {
        "report_sha256": "05a597e318e063f47cc27a30061a2a72ad46685522ce8108c0c2dde05e46bfb9",
        "original_file_hashes_verified": 65,
        "phases": [
          {
            "phase": "normal",
            "status": "passed",
            "specific_guard_dispatched": true,
            "host_absent_before_teardown": true,
            "engine_absent_before_teardown": true,
            "failure_cleanup_substituted": false
          },
          {
            "phase": "verification-standard",
            "status": "passed",
            "specific_guard_dispatched": true,
            "host_absent_before_teardown": true,
            "engine_absent_before_teardown": true,
            "failure_cleanup_substituted": false
          },
          {
            "phase": "verification-promotion",
            "status": "passed",
            "specific_guard_dispatched": true,
            "host_absent_before_teardown": true,
            "engine_absent_before_teardown": true,
            "failure_cleanup_substituted": false
          },
          {
            "phase": "verification-claim",
            "status": "passed",
            "specific_guard_dispatched": true,
            "host_absent_before_teardown": true,
            "engine_absent_before_teardown": true,
            "failure_cleanup_substituted": false
          },
          {
            "phase": "verification-capacity",
            "status": "passed",
            "specific_guard_dispatched": true,
            "host_absent_before_teardown": true,
            "engine_absent_before_teardown": true,
            "failure_cleanup_substituted": false
          },
          {
            "phase": "verification-synthetic",
            "status": "passed",
            "specific_guard_dispatched": true,
            "host_absent_before_teardown": true,
            "engine_absent_before_teardown": true,
            "failure_cleanup_substituted": false
          }
        ],
        "environment": {
          "platform": "Windows-2022Server-10.0.20348-SP0",
          "python": "3.12.10",
          "runner_image": "20261022.344.1",
          "runner_os": "Windows",
          "renderer_environment": {
            "GDK_BACKEND": null,
            "WEBKIT_DISABLE_COMPOSITING_MODE": null,
            "WEBKIT_DISABLE_DMABUF_RENDERER": null,
            "LIBGL_ALWAYS_SOFTWARE": null
          }
        },
        "canonical_archives": "One-event historical analysis prefix and all 256 capacity observations retained.",
        "native_menu_quit_targets": 6,
        "native_captures_without_engine_console_or_focus_error": 21,
        "webview2_runtime": "131.0.2903.86",
        "native_driver": "131.0.2903.86"
      }
    },
    {
      "os": "macos-15",
      "job_id": 121299212570,
      "job_url": "https://github.com/jweathers777/mShogi/actions/runs/40236534235/job/121299212570",
      "status": "passed",
      "artifact": {
        "id": 12615441834,
        "name": "desktop-macos-15",
        "url": "https://github.com/jweathers777/mShogi/actions/runs/40236534235/artifacts/12615441834",
        "sha256": "6a44f6f59b70ad874f33c1af7b1f342abd4d21460f1861f409582771f2a22ea2",
        "github_digest_matched": true
      },
      "normal_package": {
        "report_sha256": "8c4495e4abe2976d2aa22c9b0afe76c62c87427661d56a97fe9bd4338b029b37",
        "manifest_source": "996568b161eef47655cb80dafb57e5b43b488d6c",
        "host_sha256": "20ba59637b5552c3ed76959504fa2a9e2fba710d8ca1dedd487f195907d40cf6",
        "engine_sha256": "0c97c32df1441a85a51f8a338bde4bfcc9b6cd6dd4abb74cfe998454d6cbc990",
        "inventory_excludes_instrumentation_names": true,
        "frontend_exclusion_log_occurrences": 1,
        "host_marker_scan": "Successful package-smoke report generation follows actual native-host verification-marker assertions.",
        "bundled_protocol_queries": "UCI, engine-turn, move-analysis and bounded Full reports present and successful."
      },
      "gui": {
        "scope": "macOS CI package/protocol checks only; separately sourced local macOS GUI evidence is unchanged through the scoped product diff."
      }
    }
  ],
  "superseded_run": {
    "run_id": 40236378631,
    "source_revision": "1e138926d4e35acdbd9912f6aff2fc7dc64aadf5",
    "status": "completed",
    "conclusion": "success",
    "role": "Historical only; not final gate.",
    "jobs": [
      {
        "name": "Desktop (windows-2022)",
        "status": "completed",
        "conclusion": "success"
      },
      {
        "name": "Desktop (ubuntu-24.04)",
        "status": "completed",
        "conclusion": "success"
      },
      {
        "name": "Desktop (macos-15)",
        "status": "completed",
        "conclusion": "success"
      }
    ]
  },
  "unchanged_product_check": {
    "reference_source": "2f0e4cee3e1cc05df99ff51178eb4be810fbcb44",
    "paths": [
      "crates",
      "desktop/src",
      "desktop/src-tauri",
      "desktop/scripts",
      ".github/workflows/desktop.yml"
    ],
    "diff_name_only": [],
    "scope": "Scoped code/harness/workflow check; no broader builder/integration manifest-equality claim."
  },
  "limits": [
    "No new macOS GUI session by this reviewer.",
    "Windows Quit proof uses actual mouse-menu selection; prior Ctrl+Q automation success is not claimed.",
    "Original artifact ZIPs were read in memory; only this concise JSON is retained, with existing repository evidence unchanged."
  ]
}
```
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
M3 CLOSED. All eleven desktop work areas, MSH27-38 and focused archive-capacity follow-up MSH61 satisfy their complete acceptance criteria; fresh independent MSH39 review passes with no material findings. Final integrated source996568b161eef47655cb80dafb57e5b43b488d6c passed Rust40236534250 and Desktop40236534235 on all three native systems. Actual Linux/Windows normal and labeled QA applications pass all six GUI phases and native Quit/guard/host+engine absence before teardown; current normal packages exclude instrumentation. Actual macOS27.0.1 normal/QA journeys, native termination regression/Tao provenance and current regressions are retained with honest source reuse. Fresh independent verification passed46 session feature tests,22 protocol/real-process tests, strict TypeScript and78 frontend tests, plus platform hashes and canonical archives.

The application plays untimed chess with either human color, native complete choices, claims/outcomes/recovery, bounded read-only move analysis and explicit controlled Full analysis, complete Save/Open, history/Return to live/Resume and protected replacement/quit. All byte/event/control/observation limits are admitted before commit; the 257th observation is refused while all256 remain saveable. The shared synthetic16x16 routes are verification-only and distinct from real games.

See docs/review/m3-closure.md, m3-independent-review.md and docs/desktop/packaged-gui-verification.md for the eleven-area matrix, exact package/environment evidence, screenshots, launch instructions and limits. UntimedM2 live mode, bounded Full uncertainty, explicit archive ceilings, unsupported timed/Full-live restoration, no branching/crash recovery, limited assistive/compositor evidence and development signing policy remain explicit; no chess outcome or strength is invented.

Normal integration/push is on codex/rewrite; final completion changes only this task record after tested996568b. M2 stays closed, completed M3 history/MSH40 roadmap correction are preserved. MSH41-60 remain future ToDo. Next work is M4 scope/rules refinement for41-46 before implementation; no future-game work began in this batch.
<!-- SECTION:FINAL_SUMMARY:END -->
