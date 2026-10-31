# MSH-121 final current-platform handoff

Exact tested production/QA source: `5a782bf78c19ebb31d89eed2c141c6715650f452`. Later `be718ee` contains documentation/evidence only. Desktop41240191481 and Native41240191521 each completed success on Windows, macOS and Linux. No unbounded or source-unchanged retry campaign was initiated for MSH-112; its original failure remains unresolved. Failed verifier campaigns were corrected and rerun at separately recorded source revisions.

Current index with verified existing exact original paths, package/ZIP/host/engine hashes, environments, CI logs and independent dispositions: `/private/tmp/msh121-5a-platform-index.json`. Full historical actual campaign index remains `/private/tmp/msh121-platform-status.json`; all source-specific reports/ZIPs retained.

Windows current artifact13057742169 `/private/tmp/msh121-windows-5a782bf.zip`:31/31 actual GUI phases,599directZIPhashes verified,31size-preservedinitialplacements,243boundforegroundnativecaptureframesfullycontained. Six fresh normal/minimum physical views independently PASS; six webview images identical to independently reviewed d725. Actual mode TEST0/APPLY0,1920x1080/workarea1920x1040. Current native QA-standard initial content1180x800/frame1196x859; normalized normal comparison content1180x820/frame1196x879. Distinct measurements, no cross-OS inference.

Linux current artifact13056899912 `/private/tmp/msh121-linux-5a782bf.zip`:31/31 actual GUI phases,700directZIPhashes verified. Twelve normal/minimum physical/webview images directly ZIP-identical to independently reviewed d725. Actual Linux QA-standard title content1180x820/frame1182x845logical, X11clientWIDTH1180/HEIGHT820. This does not establish Mac geometry.

Both current independent artifact verdicts PASS: `/private/tmp/msh121-windows-final-independent.md`, `/private/tmp/msh121-linux-final-independent.md`. Source/product review reused for unchanged contracts; reviewed QA selector/title metadata separately covered by actual current13hosttests all3, Mac11helpertests, Windows4realgateprocess+8displaytests, Linux114externaltests.

Current raw report paths per OS: `/private/tmp/msh121-<os>-5a782bf/gui-evidence/gui-report.json`, `/audit.json`, `/gui-packages/normal/manifest.json`, `/gui-packages/verification/manifest.json`, `/gui-packages/normal/package-smoke.json`. Windows driver provenance `/private/tmp/msh121-windows-5a782bf/webview-driver.json`; Linux `/private/tmp/msh121-linux-5a782bf/gui-driver-provenance.json`. Exact concrete paths are in index.

Raw CI transcripts `/private/tmp/msh121-5a-{desktop-windows,desktop-linux,desktop-macos,native-windows,native-linux,native-macos}.log`; ANSI-normalized `.txt` sidecars retain originals unchanged. All31passedphases actualnativeQuit precedes driverteardown and residualmatchingprocessesempty; forcedcleanupnotcountedasQuit. Normalpackage-smoke marker scans passed; externalgatesnotpackaged.

Original176/0b169verifierfailures,368Windowsnormalphysicalcrop,e77826/31cascade/modal-probefailures, and176LinuxMSH112faultclassification remain preserved. No additionalcampaigns,sourcechanges,worktreeorworkerprocessneeded. Root owns actualMacGUI evidence, final acceptance and task integration.
