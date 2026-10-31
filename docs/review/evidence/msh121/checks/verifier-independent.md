# R6 verification-only delta independent review

Separate read-only reviewer `/root/r6_frontend_audit`,2026-10-31. Integrated base13510da; reviewed dirty verification-only additions (no product source changes). Files: gui-smoke.py, test_gui_smoke.py, gui_linux.py, gui_windows.py, package-smoke.py, new external verification/engine-gate.rs and test_gui_engine_gate.py. Native exact-evidence constructor assigned to separate native reviewer; this review checks its GUI usage, not independent rule correctness.

Initial tracked-diff SHA256736d5fd36d7870b5b1d8c91fd4902835bd1c1fdebb6d83008210a43bc65c3452; initial gate source SHA256d5a6ba8efc801de61e59ea6723d1f879351a3fb46ec17eff6e2039d5889ee257; gate test SHA2560b6592243fedf6ac42083121e7c9b25a9c771c7c9c35638dd2a8fc7dd44e0111. These identify the pre-correction reviewed inputs, not a final checkpoint.

## Useful coverage additions

31 actual normal/QA phases retain every original15 phase while adding current claim, concluded Full proof, Full bounded Unknown/cancel, real pending work all3games, and10Xiangqi QA cases. The new gate is compiled externally and passed only through an already QA-only engine override. It forwards original arguments and inherited pipe bytes to the unchanged real engine; it fabricates no moves/results/telemetry, modifies no product limits, and caps delay5seconds. Unix exec retains one cancellable PID. Adapter process inventory now explicitly includes gatepath, so cleanup cannot ignore that executable. Normal package smoke forbids new fixture/gate marker strings. All positive/negative/proof/Unknown presentation assertions use current actual native payloads or independently established native fixtures; GUI equality is used for lifecycle preservation, not claimed as a rule oracle.

Full proof uses already-mated native root and bounded real solver, then compares nativegame/conclusionunchanged. Zero-budget Full explicitly demandsUnknown then terminates a separate expensive request. Xq native warnings/countclaims/agreement/fourthoccurrence/fastproof/failure/retry/capacity/evidence are required distinctphases. Reopen demandsfreshdocument/stopped/noactiveanalysis and completecontext. Saved exact proof/witness/Unknown are presented as historical and not live outcomes; no new live exact UI is introduced.

## Independently executed

- `python3 -m unittest discover -s desktop/scripts -p 'test_gui*.py'`: **98PASS** on macOSarm64,0.742sec. Includes4new compiledexternalRustgatechecks for realargument/pipeforwarding, maxdelayrejection, cancellationbeforestart and afteractualchildstart (Unixexec exactPID). Log `/private/tmp/msh121-r6-delta-driver-review.log`.
- `python3 -m py_compile ...` changedPythonfiles:PASS.
- `git diff --check`:PASS.
- No actualWindows/LinuxGUI orWindowsFFIexecution claimed by this reviewer.

## Findings requiring correction/qualification before final acceptance

V01 **Concrete selector defect**: xiangqi-warning selects event8 using `.moves-events [data-event="8"]`. Actual native generated fixture has boardactions1–8 and warning9. NativeHistory deliberately excludes actions from Positionevents. Event8 is in `.moves-table button[data-event="8"]`, while event9 is in.moves-events. Currentprobe must fail; fixselectorwithoutchangingnativeexpectation. Demonstrated independently by reading authoritative shared-xiangqi-warning.json entries/events andNativeHistoryfilter.

V02 **Replacement acknowledgement race**: pending() waits for `.shogi-analysis[data-phase="analyzing"]` after replacing an already-analyzing request. This condition may be true before newcommandadmission. Immediately readingdiagnosticsandassertingversionadvanced can produce falsefailure or inspectoldrequest. Wait for actual newer accepted nativeversion/requestcontextbeforeassertingreplacedwork; capture/requestIDs/processgeneration whereavailable. Mutationinput remains exactlyonce; pollingmustbereadonly.

V03 **Windows externalgate orphan interval**: command.spawn() currentlyprecedes AssignProcessToJobObject. Cancellationofgateinthatinterval can leavechildnotyetownedbykill-on-closejob. Existingafterstartmarker testdoesnotproveabsenceofthisinterval. Useatomicownershipbeforechildexecution/inheritance (e.g suspendedcreationthenassign/resume, or gatejoinedjobbeforechildspawn withhandlekepttillprocessdeath) and verifyWindowsactualexitcodesandchildcleanup. This is externalverificationcorrectness, notaproductenginebridgefinding.

V04 **Windows test gap**: Desktopworkflow runs `test_gui*.py` onLinux only. Fournewgatetests includeWindowsspecifickill-on-closebehavior; localmacOS/LinuxPASScannotestablishthatFFI. Run gate tests onWindows andretainactualresults beforeclaimingboundedcleanupthere.

Timing qualification:2secstartupdelayallowsactualnativequery/screenshotroundtripstoexhaustthewindowbeforethenextpendingmutation,especiallyWindows. AboutdiagnosticsitselfusesseveralrealWebDrivercalls. Keepassertionsstrictandrecordfailureifworkcompleted; useup-to5secboundedgatewithintheexisting15secwatchdogifnecessaryforreliablehardstateproof. Do not callmereclickdispatch evidenceofcancellation/replacement. Existingactivephase/identityassertionsarecorrectinintent.

No unrelatedscopeexpansion or fabricatedtelemetry found. FinalverificationdeltaPASSispendingresolutionofV01–V04 andactualcurrentplatformcampaigns; normal15oldpassesalonearenotR6completion.

## Follow-up source disposition

Re-reviewed corrections from the platform verifier/root before checkpointCI. Current gateSHA25654ad48a917aa34c78575849617787034459c1cf0004ce2e6bf4d1263be1548d2, gui-smokeSHA256c977fc6fb4989e1515606259f830226c3e408752b11bcdd087d4a6f3a8e4b94a (pre-final filename portability adjustment).

- V01 resolved: board event8 usesactualkeyboardactivationof.moves-table; warning9 independentlyuses.moves-events. Currentfixtureentries/actions andwarningevent remainunchanged.
- V02 resolved: replacement_analysis_accepted requiresnewernativeversion,samedocumentandcanonicalhistory,activeanalysis;Chessadditionallyrequiresnewexactrequesttupleandmove_analysispurpose. Readonlypollnowwaitsactualack;mutatinginputisnotrepeated. Newnegativetests rejectalreadythinkingoldrequest,engine-turnpurpose,changedhistory/document andcompletedphase.
- V03 source-resolved: Windowsgateassignsitselftojobbeforechildspawn. Childinheritanceisatomic;non-inheritablejobhandleisintentionallyhelduntilprocessdeath,sokillingthegatecloseshandleandterminatesremainingchild. No Dropclosesjobwhilegateattemptstoreturnrealexitcode. Unixexecunchanged. ActualWindowsnestedjobandexitbehaviorremainCIgate.
- V04 scheduledproperly: WindowsDesktopworkflowexplicitlyruns4test_gui_engine_gatechecksusingitsseparateGUIvenvandfailsonnonzeroexit. LocalMacPASSisnotcalledWindowsPASS.
- Startupdelaynowexplicitbounded5000msinsideexisting15secsupervisorwatchdog;stalenessprobeobserves5.2secafterTakeback. This remainsmeasurementinstrumentationonly,notproductlimitincrease.

Independentlyreranfull test_gui*.py discovery: **100PASS**,0.678sec,includingnewreplacementpreconditionnegativesand4realgatechecks. `/private/tmp/msh121-r6-delta-review-corrected.log`.

Additionalportabilitychecknotifiedroot: Windowsmissingengineretrycurrentlycopiesreal.exeintoano-extensiontargetmatchingQAhost'snoextensionliteral. Windowslaunchmayappend.exe. PreferexplicitsuffixinbothQAhostandexternaldriver,orverifyactualWindowsnofilenameextensionlaunchbehaviorbeforeclaimingRetryPASS. This isverificationfixtureportability,notproductionerrorsemantics.

Sourceverdictforcorrectedverificationdelta:**PASSsubjecttofilenameportabilityresolutionandactualrequiredplatformexecution**. No productarchitecture/semanticchangesorhiddenIPCareintroduced. Preserveallactualfailures;100unittestPASSisnot31packagedphasePASS.

Final bounded portability disposition: independently confirmed QA-onlyhostinitial_controller nowuses`std::env::consts::EXE_SUFFIX`; externaldriver usesmatching`.exe ifWindows else ''` suffixwhenrestoringrealengineforretry. BothresolveidenticaltargetonWindowsandUnix. Normalhostunchanged; branchisinsidecfgdesktop-verification. This resolves thelastsourceportabilityquestion. **Final verification-onlydelta sourceverdictPASS; no unresolvedmaterialsourcefinding.** Independent100testsaboveapply; no ceremonialrerunfortwofilenameexpressions. RequiredactualWindows/LinuxGUI/Windowsgatetestresultsremainexecutiongates,notinferredpasses.
