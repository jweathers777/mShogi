# MSH-121 independent final macOS artifact and visual review

Reviewer: separate frontend/product reviewer `/root/r6_frontend_audit`. Date 2026-10-31. This review independently read the root-operated native CUA AX transcript and package/identity evidence and inspected screenshot pixels. It did not personally operate the application, run VoiceOver, mutate an approval store, or launch any application. I authored the isolated staging helper; its independent source/test review was performed by the native reviewer (11 focused tests and Python compilation passed), not counted as my own independent review here.

## Disposition

No material product or visual finding in the final macOS evidence. The required recorded native GUI journeys are supported at their actual scope. One evidence-attribution correction is required before closure: all 415 raw interaction records currently carry the recorder's original `source=176c46a9dca35f97452b011d29ad6a94ee2a2900`, including the final six geometry/capture records made after the QA host rebuild. Preserve these raw records and explicitly append a provenance mapping to the actual later QA builds; do not silently label the entire campaign as the latest source. Parent reports final frame build `5a782bf`; that commit changes QA geometry reporting only. The stable package JSON records exact executable hashes and strict signature results but does not itself contain that source revision.

## Method and actual environment

Read `docs/review/evidence/msh121/macos/interaction.json` (415 records), runtime/package/normal-smoke provenance, old/new/final stable QA identities, before/after-fixture signed bundle inventories, and saved-approval observation evidence. Personally inspected all six `final-{chess,shogi,xiangqi}-{1180,760}.jpg` pixels. Earlier independently reviewed 176-era same-size QA/normal images remain separately recorded in `/private/tmp/msh121-macos-visual-independent.md`.

The recorded system is macOS27.0.1 build26A434 arm64, Codex26.1013.40934 build20770 and WebKit22625.1.29.11.28. Normal packaged app source is176c46a, product behavior source13510da. Normal package smoke is actual engine/UCI/internal queries and is not GUI evidence. Normal GUI and QA GUI observations are explicitly separated below. Legacy `.png` names are raw JPEG captures; final six are correctly named JPEG. Do not infer image format from the older suffix.

The six final actual native QA titles in AX records388/391,398/401,408/411 report BOTH inner content and outer frame as1180×820 logical and760×650 logical respectively. Both observed values are reported here; neither is inferred from image pixels or a supposed fixed Cocoa titlebar difference. The displayed board/footer assessment uses the pixels.

## Same-size pixel comparison

All three games retain the same dark application theme, header, title/document state, New game/Save/Load positions, live heading/Flip hierarchy, board-versus-sidebar spatial relationship, and Analysis → Game Actions → Moves order. Chess uses its native board/pieces; Shogi retains warm board, pentagonal kanji, rotated Gote ownership and integrated hands; Xiangqi retains intersections, palaces/river and Red/Black content.

At1180×820 and760×650, the complete board, player/hand regions and common live footer remain visible in all three games without board scrolling/zooming. The minimum sidebar may scroll to lower Moves content; this does not clip the board or require moving common controls into a different hierarchy. No new layout/accessibility regression was observed in these captures. AX exposes native game-specific square/intersection and piece labels, controls, selected state, game status and focused elements. Keyboard/pointer actions and live announcements are evidenced by AX; this is not an assertion of a complete assistive-technology/VoiceOver campaign.

Final image SHA256:
- Chess1180 `88dfba2312071c1792ae3d2c8449520a3e73d8f3dfebd7c2212ec618f564b6b8`
- Chess760 `a3ea0c0e42d7628b899477fe1b4ee07ae7c5b269a8f1f43b50081acc37e595f4`
- Shogi1180 `7159ed6fcd9ee322716c783dade2398592deb7c146cea48f207d133326c28189`
- Shogi760 `ae0c4f0c2dab06c896d6ca7af5a49d158443805e588c0cc559a6be28e348ffea`
- Xiangqi1180 `5d0f5a593a15e057728251f3f4003864274db5dee16f2cb5a4a773690131886e`
- Xiangqi760 `9f6a6a92c15ac9643da592c88f546fab121bbea27143ed0623c8f6c6a2c85268`

## Journey assessment from actual native AX interaction

Normal distribution: Chess records1–35 show play with real native engine result, history/Return, bounded read-only analysis, Take back/stopped state, Save cancel/retry/success, Load cancel/success with UNMODIFIED/stopped document, Resume, truthful unresolved resignation, native quit Save/Cancel protection. Records220–273 additionally establish native opponent inspection, shared exact visible language, keyboard completed Shogi and Xiangqi board actions, real analysis, native Save/Load and stopped reopen for both games, About/diagnostics and clean native window close. New game chooser is the intended single replacement confirmation; a second destructive confirmation is not independently required merely because the new document is dirty.

Separately identified QA distribution establishes difficult states using the public native UI/controller:
- Chess underpromotion chooser cancellation and Knight completion produces native proven-dead conclusion (118–125); current fifty-move claim concludes without another board move (130); intended-move claim remains unplayed/0plies (351–353). Full reachability proof is explicitly cooperative, not a forced win and not a new live outcome (40). Zero-budget work is Unknown rather than an inferred mating conclusion (321); explicit cancellation retains no accepted result and stopped play (326).
- Shogi native opponent-hand inspection is read-only/non-actionable with approved visible language and separate accessibility announcement (135); native drop completion/real engine reply(139), optional promotion cancel and accepted promoted pawn(330–336), and advisory declaration followed by committed entering-king conclusion(146–147).
- Xiangqi history prefix8 and warning-event prefix9 show the same8board plies, with no future warning at8 and an active Red perpetual-check warning at9(152,155). Native opponent inspection remains distinct(158). Natural-count claim is available but ongoing at100plies until user confirms, then native draw with100plies unchanged(344–347). Capacity refusal truthfully names control capacity, preserves saveability, and says reopening does not reset it(310–314).
- Xiangqi exact evidence shows sound fast proof separately from five historical exact reports: positive Red natural-board-win witness explicitly not forced; exact no-win proof; Unknown; cancelled Unknown; failure. Reports retain event0 versus event4 provenance, show unplayed route disclosure, survive native Save/Load, and do not replace the independently concluded live native game(274–286). This is historical exact-result presentation/restoration evidence, not an invented claim that the UI starts a new Xiangqi exact query.
- All three games have actual pending-engine Take back to0plies/stopped, analysis replacement/cancellation and no completed action invented(163–196). Deep native lineage, exact context identities and late-result assertions are supported by separate controller and all-platform driver evidence, not inferred solely from these pixels.
- Earlier missing-engine Retry attempts remain failures in their original records(197–217); they are not counted as recovery. Later controlled real bundled-engine restoration followed by actual Retry succeeds for Chess(360→362), Shogi(368→370/371), Xiangqi(377→379/380): game remained0plies/unscored during failure and becomes exactly1ply with Complete native engine observation after Retry. The bundle is restored and strict verified per identity evidence.
- Clearly labeled synthetic16×16 fixture is explicitly not a real variant. Two routes with the same endpoints differ via their full native staged geometry; partial selection/cancel accepted count stays0, complete first/alternate route accepted count becomes1. Keyboard arrows, cancellation, reset, orientation, and native close protection are exercised(290–309,384–387). Synthetic records cannot be saved as chess.

## Stable app identity and approvals

Earlier audit independently found20 app identities; per-fixture plist/bundle-ID/signature variation created unnecessary new application identities. The durable fixed QA and synthetic paths/legacy approved IDs eliminate per-scenario identity creation. Before/after external fixture selection inventories have identical Info.plist, host, engine, resources, code signature and CDHash; strict verify returns0. Actual old/new host hashes change from `2a9b4e30…` to`06072357…`, while fixed QA path and bundle ID remain constant; both strict verify0. Later geometry host hash `eff22c4747600590f4e2362fe53107805a36e9c50231e01a3be40793e63f11bc` and CDHash430d3d… is another explicitly recorded QA build.

`stable-rebuild-reuse-observation.json` records actual CUA launches of relocated old build, changed host at same fixed path/ID, unchanged bundle with another external fixture, standard fixture, and fixed synthetic app, with no observed approval interruption. Read-only saved approval-store SHA256 remains `401fe192e9a95516a387311ef66dc1828f897e8a27331a8eb01e76d21be7960d` before/after. Running selector refusal is actually exercised with exit1 and prior selector preserved. No approval/security policy was edited. This supports observed reuse on this installed runtime/session, not guaranteed approval persistence across future versions/accounts/policies. All signatures are ad-hoc, TeamIdentifier not set; code hashes can legitimately change on rebuild.

## Review limitations and follow-up

The only remaining review action is precise append-only source provenance reconciliation for later Mac captures. No new production correction or rule campaign is required by this artifact review. Existing native/reference rule correctness remains the original independent fixtures' responsibility. No personal remote GUI execution, no VoiceOver completion, and no new Windows/Linux inference is claimed. Their final actual-platform artifact reviews are separately documented.

## Final source-provenance finding resolution

Parent resolved the preceding documentation finding with explicit per-record annotations, manifest links and preserved original data. Independently verified `source-reconciliation.json`: decompressed `interaction-before-source-reconciliation.json.gz` SHA256 is exactly `7a0b1ff1aa56b188930cde1c94d18a87ef6a734507126f2e331250bc29bb340d`; its415records compare field-for-field with the reconciled log for all original keys except the59corrected source annotations. Each changed annotation retains `source_label_before_provenance_reconciliation`, an explicit reason and the exact package manifest. No original AX/state/action/capture field changed.

Final source distribution:344records176c46a,44records801e1a905c682c408fc293a88b131d873c9c960b,27records5a782bf78c19ebb31d89eed2c141c6715650f452. The final six geometry records and final six JPGs are explicitly source5a782bf and link stable-qa-frame-build; selector/recovery later phase source801e1a links stable-qa-new-build. `campaign-summary.json` independently verified all95capture SHA256s, true JPEG format and pixel dimensions against actual files. The historical initial inventory remains distinctly identified. Provenance finding resolved; no material remaining Mac artifact or visual finding.
