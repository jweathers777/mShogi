# Normal Tenjiku desktop continuation

Actual normal `org.mshogi.desktop` application on macOS 27.0.1, arm64. These are native screenshots and accessibility captures, not web mockups. The exact package hashes, tested source and final cross-platform receipts are reconciled in [product-verification.json](../../product-verification.json). Each image has an accompanying `.ax.txt` capture and an entry in [artifact-manifest.json](artifact-manifest.json).

The owner approved the traditional-kanji design. The hard application minimum remains 760×650; approximately 960×1000 is a recommended playing size when the display permits. The whole 16×16 board fits at the minimum, but some kanji and secondary route badges remain too small for comfortable reading. Friendly piece names help identification without resolving every small-window accessibility limitation. These normal application captures do not claim fresh minimum-size DOM measurements; the prior fixed-slot QA measurements and the final Linux/Windows native-frame checks are recorded separately.

## Normal activation and shared presentation

![Sixth game and explicit Adams profile](normal-sixth-game-adams-profile-chooser.png)

![Dense opening, friendly Great General name and above-piece capture outlines](normal-adams-great-general-captures.png)

![Muller, human Gote, friendly Great General and occupied destinations](normal-muller-gote-great-general-captures.png)

![Lion Hawk name persists during partial compound route](normal-adams-compound-partial-name.png)

![Escape clears the compound selection](normal-adams-compound-cancelled.png)

![Completed two-capture action and genuine engine reply](normal-adams-compound-completed-engine-reply.png)

## Genuine play and lifecycle

![Adams human move and real Gote engine reply](normal-adams-human-engine-reply.png)

![Adams read-only analysis](normal-adams-readonly-analysis.png)

![Adams ordinary Open restores a stopped game](normal-adams-reopened-stopped.png)

![Adams historical inspection](normal-adams-history.png)

![Adams Takeback retains the previous branch](normal-adams-takeback-retained.png)

![Muller human Gote and complete multi-stage Sente engine reply](normal-muller-human-gote-engine-reply.png)

![Muller read-only analysis](normal-muller-readonly-analysis.png)

![Muller ordinary Open preserves profile and stops play](normal-muller-reopened-stopped.png)

![Dirty Quit guard; Cancel preserves the game](normal-muller-dirty-quit-guard.png)

![Muller historical inspection](normal-muller-history.png)

![Muller Takeback returns to the prior human decision and retains the branch](normal-muller-takeback-retained.png)

## Complete tactical games

These explicitly imported diagrams have no claim of a complete standard-opening history or competitive strength. Independent fixture expectations and public native replay establish their outcomes. Both normal application games start stopped, Resume the actual supervised engine, execute `FiD8jx8g`, and conclude native RoyalCapture for Sente in one board action.

![Adams native royal-capture conclusion](normal-adams-engine-royal-capture.png)

![Muller native royal-capture conclusion](normal-muller-engine-royal-capture.png)

The earlier unextended diagram also allowed the legitimate winning move `FiD8j-1c`, burning Gote's last nonroyal Pawn and ending by BareRoyalLoss. The [superseded capture](superseded-adams-royal-probe-bare-loss.png) preserves that actual observation. The two derived royal-engine fixtures add a remote Gote Pawn at 1a, with explicit independent provenance, to isolate the intended royal-capture case. The frozen rules corpus and native rules are unchanged. The old observation is not counted as a passed RoyalCapture expectation.
