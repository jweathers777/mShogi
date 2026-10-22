# mShogi rewrite

This repository baseline describes the rewrite of John Weathers's mShogi. It contains design, working instructions, an initial Backlog.md task queue, and the existing Rust prototype preserved from `main_v2`. The prototype has not been verified against the new design or credited as completed milestone work.

The product goal is a reusable library with complete implementations of chess, shogi, tori shogi, chu shogi, tenjiku shogi, and xiangqi; capable mShogi engines; a standalone and interoperable console application; and a polished desktop application for Windows, macOS, and Linux.

## Start here

- Product and architectural contract: [docs/design.md](docs/design.md).
- Working instructions for every implementation session: [AGENTS.md](AGENTS.md).
- Importing this package into the real repository: [START-HERE.md](START-HERE.md).
- Current work and evidence: the native task records under `backlog/`.

Design and product decisions are developed with John in ChatGPT. Codex or Claude Code implements bounded work from this repository. A fresh session must be able to continue without access to an earlier conversation.

## Current objective

Bootstrap the repository workflow, then establish a chess rules kernel and a headless verification console. This first milestone establishes legal move generation, reversible state changes, useful diagnostics, and independent correctness evidence. It is not a claim of complete chess adjudication or competitive playing strength.

Search, protocol interoperability, the desktop application, complete variant implementations, and learned evaluation follow in the roadmap in `docs/design.md`.

## Repository and existing work

The rewrite integration branch is `codex/rewrite`, based on `main_v2` at `fd280dff02fa7ef73809e20b28bdec83fa9c955f`, including its local WIP commit. The legacy `master` branch remains at `941cd4dc37e6e6210d4f993c96a553753e228b19`; neither existing branch was changed during bootstrap. The original `README` and `BSD-LICENSE` are retained.

`Cargo.toml`, `Cargo.lock`, `src/`, and `variants/` are the pre-existing experiment, preserved without source changes. Their presence does not satisfy the new rules profile, action contracts, workspace checks, or conformance criteria. MSH-2 and MSH-3 must inspect and reconcile that work within their scopes. MSH-1 records the import and integration evidence.

## Backlog commands

The starter was generated and validated with Backlog.md 1.55.1. Use that version for this project. If a different global CLI is installed, invoke the pinned version with `npm exec --yes --package backlog.md@1.55.1 -- backlog <command>`; a global upgrade is optional.

```bash
npm install -g backlog.md@1.55.1
backlog --version
backlog instructions overview
backlog task list --status Ready --ready --plain
backlog task view MSH-1 --plain
backlog browser
```

`--ready` checks dependencies. Use it together with `--status Ready` when choosing new implementation work. The tool's dependency check does not establish user authorization, correctness, review completion, or compliance with acceptance criteria.

Supported build and test commands will be established when MSH-3 reconciles the existing Cargo prototype with the planned Rust workspace. Do not invent successful commands or badges before that implementation exists.

## Source of truth

The design states requirements and decisions. Backlog.md records work, status, verification, and handoff. Git records changes. There is no parallel GitHub Issues queue or mandatory chronological handoff diary.
