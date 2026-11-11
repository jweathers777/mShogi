# Study status snapshots

These standard-library Python scripts read saved progress once. They do not start,
stop, retry or modify computations, queue messages, or poll in the background.
Run from the repository root:

```sh
python3 tools/study_status/check_status.py
python3 tools/study_status/check_analysis_status.py 1
python3 tools/study_status/check_adjudication_status.py 1
```

| Command | Coverage |
| --- | --- |
| `check_status.py` | Generated games across all recorded batches; excludes later analysis/review |
| `check_analysis_status.py BATCH` | All-move screening, confirmation-decision progress, current pipeline step and exit-notification receipt |
| `check_adjudication_status.py BATCH` | Deeper adjudication, current query, recorded failures and exit-notification receipt |

Batch defaults to 1. Use `2` for batch 02, and so on. A paired-game screening job
covers two games; ten such jobs cover one 20-game batch. Confirmations start after
screening, so finishing the games alone does not trigger the review notification.

Default evidence is `docs/m5/evidence/msh161` relative to this checkout, regardless
of the shell's working directory. All commands accept `--root /path/to/evidence`.
They understand the current MSH-161 (Competitive Gameplay Diagnosis) receipt
layout; a future study with a different schema needs an explicit adaptation.
The game monitor's 20/200 denominators describe this study's design.

Saved `running` status does not prove process liveness. Read the timestamps and
errors; no ETA is inferred. Queue acceptance is separate from computation success
and does not prove the coordinator has reviewed the result. Independent review,
batch authorization and final diagnosis happen in the coordinator chat, not in
these scripts. Further targeted adjudication can use the adjudication monitor
when it retains the same plan/notification layout; otherwise inspect that run's
own receipt. No additional monitor is currently required for the frozen pipeline.

These monitors are separate from frozen experiment helpers. Editing them does not
change the experiment's execution, analysis policy or evidence.
