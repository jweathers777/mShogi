---
id: MSH-11
title: Expose the rules and diagnostics through a headless console
status: Done
assignee:
  - '@claude'
created_date: '2026-10-22T13:27:00Z'
updated_date: '2026-10-23T19:35:00Z'
labels:
  - console
  - diagnostics
  - integration
milestone: m-1
dependencies:
  - MSH-9
  - MSH-10
documentation:
  - docs/design.md
  - AGENTS.md
priority: medium
ordinal: 11000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Allow a developer or user to reproduce rule behavior from commands without starting a GUI.

## Design references

P-03; P-05; A-01; M1 console

## Scope

Support loading/inspecting a chess position, listing legal actions, applying/undoing a sequence, reporting check/mate/stalemate, and running perft/divide. Define actual help, output, error, and exit behavior; add an interactive shell only as needed to satisfy the workflow.

## Verification expectations

Exercise real process invocations and a representative load/list/apply/undo/perft sequence. Check stdout/stderr and exit behavior as specified, including malformed input and a rejected action.

## Out of scope

Claimed UCI/USI compatibility, search, clocks, GUI, and a speculative universal machine protocol.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Documented console commands expose every promised M1 diagnostic through the real binary.
- [x] #2 A position can be loaded, moved, undone, and inspected with state matching the library contract.
- [x] #3 Malformed inputs and illegal actions produce clear errors while preserving relevant session state.
- [x] #4 The console runs without UI, database, or Python dependencies.
- [x] #5 README examples and a bounded process-level smoke check reproduce the implemented behavior.
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
1. Claim MSH-11 on claude/next-task at 143dfac (equal to origin/codex/rewrite), worktree .claude/worktrees/next-task. Review the current console scaffold, its CLI tests, the A-05 human/machine output distinction and the library APIs from MSH-4 to MSH-10.
2. Add one session mode, 'mshogi shell', a human-oriented line command session (not a machine protocol and not UCI) reading commands from stdin, interactively or piped. It starts at the standard position. Blank lines and '#' comments are ignored. Results go to stdout; diagnostics go to stderr as 'error: line N: ...'.
3. Commands:
   - position startpos | position fen <FEN>: load and clear history;
   - fen and show: canonical FEN, plus a board diagram, side, rights, raw target, counters, status and history depth;
   - legal: count and actions in divide order;
   - move <action>...: an atomic sequence, so all apply or none;
   - undo [n];
   - status: quiet/check/checkmate/stalemate;
   - perft <depth>; divide <depth>;
   - help; quit.
   Actions use the documented lowercase coordinate notation (e2e4, e7e8q, king-move castling).
4. Errors keep session state: malformed FEN, unknown command, bad arguments, illegal or overflowing actions (reported with the library error), undo beyond history and invalid UTF-8 lines. Exit status: 0 if every command succeeded, 1 if any failed, 2 for invalid program arguments. Existing help/version/profile stay.
5. Parsing and I/O live only in the console crate; rules come only from mshogi-core (A-01). No new dependencies.
6. Process-level tests: the full load/list/apply/undo/status/perft/divide sequence, malformed inputs, a rejected action with preserved state, exit codes, stdout/stderr separation, and README example reproduction. Document commands in README and development docs. Workspace checks, independent review, integration, native CI, close.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup (2026-10-23): owner @claude, branch claude/next-task in worktree .claude/worktrees/next-task at 143dfac, equal to origin/codex/rewrite; no other task In Progress. User explicitly selected MSH-11 (dependencies MSH-9 and MSH-10 Done). Pinned backlog.md 1.55.1, Rust 1.89.0.

Implementation candidate on parent 143dfac:
- New crates/mshogi-console/src/session.rs: a human-oriented line command session over mshogi-core. It is not UCI or a machine protocol (A-05). It only parses and formats; all rules come from the library (A-01).
- main.rs adds the shell mode, which reads stdin line by line, interactive or piped, starting from the standard position. Help, version and profile remain, with updated help and coverage text.
- Commands: position startpos/fen; fen; show (diagram plus every field, status and history depth); legal (count plus actions in divide order); move with one or more actions (atomic: applied to a copy, all or none); undo [n]; status; perft; divide; help; quit.
- Notation: lowercase coordinates, promotion suffix, castling as the king move. Blank and comment lines are ignored; CRLF and a missing final newline are accepted.
- Errors: printed as error: line N: reason on stderr, and the session is unchanged. Covered: malformed FEN, unknown commands or arguments, illegal or overflowing actions (library error plus failing index), undo beyond history, invalid depth, divide 0, invalid UTF-8 lines.
- Exit status: 0 if all commands succeeded, 1 if any failed, 2 for invalid program arguments.
- No new dependencies: the console depends only on the core; there is no UI, database or Python.
- The CI console smoke step now also pipes a shell session (load, move, show, undo, status, perft, divide); pipefail fails the step on a nonzero status.
- Docs: a README console section (command table, notation, errors, exit status, and an example with exact stdout/stderr between markers), the development guide, and the coverage ledger console row.

Local verification PASS, macOS arm64, rustc 1.89.0:
- cargo test -p mshogi-console --locked (11/11):
  - help and version;
  - profile text;
  - argument errors including extra shell arguments;
  - non-Unicode argument;
  - full workflow with exact stdout: castling both sides, show, legal count consistency, undo, a stalemate position with legal 0, perft 0/2, empty divide, start perft 3 = 8902 and divide 2 = 20 x 20 / total 400 (independent known values);
  - 13 distinct failures with exact stderr and preserved state verified through later fen/undo output;
  - quit and EOF;
  - CRLF and a missing final newline;
  - invalid UTF-8;
  - help lists every command;
  - the README example reproduced byte for byte (stdout, stderr, exit 1).
- cargo fmt check; clippy -D warnings; cargo test --workspace --locked (all suites); build; local CI-equivalent smoke commands; cargo tree; whitespace diff check.
Native CI pending integration. An independent review is running.

Independent review: a fresh delegated reviewer (Claude subagent, no repository edits) found session.rs correct and atomic, the stream ordering and exit codes sound, every rule delegated to the core, no new dependencies and no doc overclaims. It ran about 20 adversarial piped sessions, including early-closed stdout, an 80,000-action move line with matching undo, tabs, extra arguments, uppercase input and overflow; it did not run Windows CI.

Findings and resolution:
- B1 (likely blocking): the README example test depended on LF endings, and a Windows checkout may give CRLF. Fixed by normalizing CRLF in the test; verified by temporarily converting README.md to CRLF locally (test passes) and restoring it byte for byte. Native Windows CI is to confirm.
- N1: legal reused the divide order via a copied key. Added a process test that legal order equals divide 1 order on a promotion position.
- N2: the divide total now uses checked addition and reports perft: node count exceeds u64.
- N7: moving into check reported only the FEN structural reason. ActionError::IllegalPosition now displays as 'action rejected: own king would be in check (<structural reason>)', with the variant doc updated. This is a small library message change for AC3 clarity, justified by the MSH-8 review's finding that from a validated position this error only means the moving king would be attacked. A test asserts the exact console message.
- Not changed (cosmetic or harmless, recorded): N3 raw input echoed in errors; N4 Unicode whitespace accepted as a separator; N5 a leading + accepted in counts and depths; N6 no limit on line length or perft depth (Ctrl-C), which suits a diagnostic console; N8 the CI smoke checks exit status only, while the process tests check exact output.
Reruns PASS: cargo test --workspace --locked (console 12/12 and all suites); fmt; clippy; whitespace diff check.

Integration and native verification (2026-10-23): implementation 8670c73aa380862ee66ac46bb9f8f11148242fd6 committed on claude/next-task and fast-forward pushed to origin codex/rewrite (143dfac..8670c73). No force push, PR, default-branch or settings change.

Actual native CI PASS for exact head 8670c73: https://github.com/jweathers777/mShogi/actions/runs/40012369679. All three jobs succeeded on rustc 1.89.0:
- Rust (ubuntu-24.04), job 120615711186: console tests 12 passed.
- Rust (macos-15), job 120615711131: console tests 12 passed.
- Rust (windows-2022), job 120615710876: console tests 11 passed; the Unix-only byte-argument test is not compiled there.
On every platform readme_console_example_reproduces passed, which resolves review B1 natively, including the Windows checkout. All core suites passed (perft with 1 deep test ignored as designed). The new console smoke step piped a shell session through the real binary under bash pipefail and printed perft 3 8902 and the divide total 20 on each runner.

Next eligible: MSH-12 (sequence/differential verification) and MSH-13 (compound-action boundary review); MSH-14 milestone closure still waits for them. No subsequent task started.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added 'mshogi shell', a human-oriented line command session over mshogi-core. It is a diagnostic console, not UCI or a machine protocol (A-05). It reads stdin interactively or from a pipe and starts at the standard position.
- Commands: position startpos/fen; fen; show (board plus every field, status and history); legal (divide order); move (atomic sequence); undo [n]; status (the limited M1 classifier); perft; divide; help; quit.
- Notation: lowercase coordinates with a promotion suffix and king-move castling.
- Output: results on stdout; failures as 'error: line N: <reason>' on stderr, and nothing in the session changes.
- Exit status: 0/1/2.
All rules come from the library (A-01), and there are no new dependencies and no UI, database or Python. Moving into check now reads 'own king would be in check (...)' via the library's IllegalPosition display, for clearer errors.

Verification: 12 process tests against the real binary cover the full load/show/legal/move/undo/status/perft/divide workflow with exact output and independent counts, 13 failure modes with exact stderr and preserved state, quit/EOF/CRLF/invalid UTF-8, help completeness, legal versus divide order, the clear check error, and the README example reproduced byte for byte. CI now pipes a shell session on every platform. Independent review found the session correct and atomic. Its likely-blocking Windows CRLF issue in the README test was fixed, verified locally, and confirmed on native Windows CI; its nits were resolved or recorded. Native Linux/macOS/Windows CI run 40012369679 passed for 8670c73 on codex/rewrite.

Limitations, recorded as nits: raw input is echoed in errors; Unicode whitespace and a leading + are accepted; line length and perft depth are unbounded. No interactive prompt or machine mode.
<!-- SECTION:FINAL_SUMMARY:END -->
