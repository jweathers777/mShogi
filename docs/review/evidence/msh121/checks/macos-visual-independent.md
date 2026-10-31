# R6 macOS current artifact visual assessment

Separate read-only reviewer `/root/r6_frontend_audit`,2026-10-31. Actual artifact review, not personal GUI operation. Inspected pixels of all six `docs/review/evidence/msh121/macos/qa-{chess,shogi,xiangqi}-{1180,760}.png`, plus normal-chess-1180, qa-shogi-history, qa-xiangqi-play and qa-chess-full-proof. Read current118-entry interaction.json AX transcript. All identify source176c46a; QA is explicitly labeled and is not claimed as a normal package. QA title preserves initialfixtureconcluded while actualnativeNewGame switches its livegame; do not interpret thatlabelasthecurrentconclusion.

## Pixel comparison PASS

Same common header/branding/theme/documentstate/title/Newgame/Save/Load placement, gameheading/Flip region, board/player/sidebar grammar and Analysis→GameActions→Moves order. NormalChess matches QAChess commoncomposition. Game-specific warm Shogi pentagons/kanji/rotatedopponentpieces and hands remain; Xiangqi correctlyusesintersections, river, palace diagonals and distinct Red/Black discs. All completeboards visible at1180x820and760x650logicalcontent withoutboardscroll/zoom. Common lowerplayer, keyboardhelp and livepositionfooter remain visible atminimum, including Shogihandempty-statecontent. Minimumsidebar Moves continuesbelowviewport: intentionalindependentsidebarscroll, notboardclipping orcontrolsrearrangement. No new visualregression found; accepted design is not reopened.

PlayedXiangqi screenshothasnativeorigin/destinationcue, standardnative two-columnnotations `P9+1` and `C2+7`, realBlackperspectivesearchobservations, andBoard-winassessmentUnknown separatefromgameoutcome. HistoricalShogi screenshotclearlylabels HISTORY/READONLY and Returntolive, showsrootselectedandnativeMoves7６歩/4４歩; originalLastengineturn remains explicitly labeled ratherthanmasqueradingasselected-positionanalysis. No misleadingcurrentgameconclusionfound.

## Accessibility and interaction qualification

AXactualtranscript establishesnamedboardbuttons/nativecoordinates/sidepieceidentities, nativeNewgameselect and sidecontrols, namedAnalysis/Moves/gameactions, actualhistoryReturn, stoppedResume, and concise SaveGame withSave/Discard/Cancel focusedonCancel. NativeFullproofAXtext explicitlysayscooperativerouteexists, notforced-win/liveoutcome, andgameconcludedstaysunchanged. ScreenshotFullproofshowsadvancedcontrolsabovefold; theactualresultisAX-evidencedbelowfold, notvisibleinthatPNG. Captionmustnotclaimvisibleproofresultwithoutanadditionalreveal/capture.

No opponentinspectionentryappearsinthis118-recordMaclog, so theseartifactsalone do notestablishfreshMacopponentinspection orscreenreaderoperation. Source/independenttests andactualotherplatformjourneysareseparateevidence. Staticpixelscannotprovekeyboardfocustrapping/controlreachability; useactualdriver/transcriptcoverage.

Owner's NewGameintentionalreplacementpolicy requires nosecondunsavedguard. CurrentnativeNewgamechooser+explicitStart complies; absenceofanextra warningparagraph/dialogisnotanewdefect. No unapprovedlayoutredesignrequested.

Verdict: **PASSforcurrentmacOSsame-sizepixelcomparisonandboundedAXartifactassessment**, withtheaboveevidencequalifications. FinalR6platform/accessibilityacceptance stillrequiresexactcampaigncoverage/provenanceandremainingactualMacnativecapabilities.
