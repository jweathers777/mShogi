# Fresh MSH-75 normal macOS application journey

Actual external CUA pointer, accessibility, keyboard and native file dialogs on
2026-10-27; macOS27.0.1 arm64. Normal bundled application, not verification or
the MSH-72 development harness. Reviewed source80d6b5c is production-equivalent
to the accepted package's abe66c5/7df91d3 source. Fresh
[package protocol/inventory smoke](package-smoke.json) has the exact final host
`a2f5fa4f05745e12e14f4e80b0349b6e203e266b2a59c4cd68fc7986d8611dcc`
and engine `5d0fa23fdd7ca5683c848c7117379aabed033288c6d9114f697492153f2f5615`.
[Runtime and native archive facts](runtime.json) bind path/PID, roots, records and
the visible technical token change. [Artifact index](artifact-index.json) binds
unaltered JPEG/AX/record bytes before this README was added. Captures are1180×820
native including titlebar,2360×1640 Retina pixels. No new minimum-size claim is
inferred; unchanged current-platform minimum evidence is independently audited.

| Fresh normal interaction | Screenshot / full native AX evidence |
| --- | --- |
| New Shogi/Sente, full9×9; keyboard7g→7f, real engine4c4d | [Initial](sente-initial.jpg), [keyboard play](keyboard-play-engine-reply.jpg) |
| Bishop captures pawn into hand; engine1a1b | [Capture](capture-hand-engine-reply.jpg) |
| Optional promotion chooser, cancel without action, choose promotion; engine knight captures promoted bishop into unpromoted bishop hand | [Choice](promotion-choice.jpg), [cancelled](promotion-cancelled-no-action.jpg), [complete](promotion-complete-engine-reply.jpg) |
| Read-only opponent knight and bishop-in-hand destinations; Escape clears, still6plies | [Piece](opponent-piece-inspection.jpg), [hand](opponent-hand-inspection.jpg) |
| Native Save then Open complete six-event game; Unmodified, explicit stopped Resume | [Saved](saved-six-event-game.jpg), [reopened](reopened-six-event-game-stopped.jpg) |
| Open same archive twice establishes fresh document/version/revision/selection | [Before AX](reopened-native-before.ax.txt), [after AX](reopened-native-after.ax.txt): document2→3, version26→27, revision27→28, selection26→27. Visible native JSON is truncated by AX; no unobserved full-JSON claim. |
| Select event3: SentePawn1, no future GoteBishop; read-only Gote analysis −2.00, depth1/30nodes, no action | [Correct prefix](history-prefix-without-future-hand.jpg), [actual analysis](historical-read-only-analysis.jpg) |
| Return to live6plies restores hands and retires current historical result; Resume/Flip; human5g5f, real engine1c1d; Save8events | [Return](return-live-analysis-retired.jpg), [Resume/Flip](resumed-flipped-live.jpg), [engine reply](resumed-engine-play.jpg) |
| New Shogi/Gote: native opening1g1f, human3c3d, native6g6f; Save | [Opening](gote-engine-opening.jpg), [play](gote-human-and-engine-reply.jpg), [saved](gote-saved.jpg) |
| Open existing native **reduced-material imported fixture**; Resume, hand selection cancellation leaves2plies, completeP*5e then native1a1b | [Choices](fixture-native-drop-choices.jpg), [cancelled](fixture-drop-cancelled-no-action.jpg), [committed](fixture-complete-drop-engine-reply.jpg) |
| Save As separate QA file; cancel resignation preserves4plies, then conclude with distinct event5 at4boardplies and Save | [Cancel](fixture-resignation-cancelled.jpg), [terminal event](fixture-resignation-terminal-event.jpg), [saved](fixture-resignation-saved.jpg) |
| Leave standard-start eight-event game reopened, Unmodified, stopped until Resume | [Final normal application](final-normal-reopened-stopped.jpg) |

Each screenshot has its same-basename `.ax.txt`, except chooser captures whose
AX correctly exposes the active modal rather than inert background. The complete
native records are retained in `records/`: normal-sente has8events/5observations,
normal-gote3/2, imported-drop4/1, imported-resignation5/1. Standard Sente history
analysis contains **only three events**, with explicit historical target/purpose;
the future hand content is absent. Saved observations remain information, not
resumed pending work. The saved-observation `historical` flag is distinct from
an analysis request selecting a historical position in the current runtime.

The imported fixture is the unchanged MSH-74 native capture archive, with supplied
counter17 and unknown earlier history. It is not proof of standard-start
reachability. Its source file was never overwritten; Save As wrote `/private/tmp`
before the complete QA copies were retained. Mandatory promotion, entering-king,
capacity, cancellation/replacement, failures/retry, stale-result recovery, native
quit and minimum-size journeys reuse separately labeled current-source MSH-74
platform evidence rather than being claimed as fresh Mac interactions here.

The preexisting entry game was preserved separately in `/private/tmp` before
New Game. One automation locator initially matched both a board row and the
focused-element echo; it was corrected to actual indexed rows before acting.
Sandbox process listing was unavailable, then an authorized read obtained the
normal PID/path. These are verification-invocation limitations, not product
failures. No injected host command, test state or raw implementation call was
used to perform the GUI journeys.
