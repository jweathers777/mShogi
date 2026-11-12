# Failed Linux validation: independent integrity and cleanup audit

2026-11-12, execution_review. **Evidence integrity passes; validation remains failed. No rerun was performed or authorized by this audit.** The machine-readable companion contains the single read-only GCP/local snapshot, raw responses, commands, hashes and resource calculations. No engine, build, cloud mutation or submission was executed.

All12 checks passed: controller download hashes; artifact and upload manifests; intended build and returned build equality; packaged helper hash; image digest across build/registry/task; submission, failed terminal response and notification UID; resource identity; native embedded revision and empty dirty hash; report inner hash; actual candidate/opponent binary hashes; and exact explicit_only configuration. The native source was `ab7ef8f10d9b9023dbe525fcbc47e54a40c423e0`, worker source `d9271a294cb92e1e87d366d49dfd409bbfd662b6`, and image digest `544b12cbefb4c78e75bd982fe2f62ec189869b5d2f10aae7d81ee1e28173215f`. Native runner and replay exited0; that establishes readable/replayable failure evidence, not healthy gameplay.

Both validation games stopped with failures and remained unscored. The first records Oxidation `bestmove 0000` as an invalid coordinate action. The second records Oxidation EOF before bestmove, with last output `info string servererror illegal move h8g7 from 7k/8/8/8/8/8/8/KR6 b - - 100 1`. The report audit counted two searched plies, not the intended eight. The initial root has a100 halfmove clock. Cause and compatibility implications belong to the separate source review; this audit does not reinterpret either failure as a draw, pass or completed study game.

Cleanup is reconciled within the observed scope. Native runner/replay and local wrapper receipts each report settled process groups. A single current process census found recorded local PIDs96307 and96309 absent. Cloud task0 is FAILED with exit1. The one observed C3 VM was deleted successfully at `2026-11-12T14:03:26.062062Z`; current filtered instance and disk inventories are empty. The inventory commands emitted warnings that their filter keys were absent, retained in the JSON. The successful delete audit event independently supports VM cleanup. This is not a general descendant-tree census or a claim that the persisted Batch job record was deleted. The failure callback was accepted successfully.

Resource accounting, without adding nested intervals:

- One failed task; task RUNNING interval9.282370026s; no retry observed.
- VM insert request to completed deletion63.461652s (0.0176282367 VM-hours), one worker/concurrency1.
- Helper wall5.102551624s and child CPU5.10351s.
- Validation controller wall93.762887s; preceding image build22.938538s; enclosing deployment wrapper116.931703s.
- Compute estimate$0.003554 using the previously recorded$0.201608/VM-hour rate. This is not a billing receipt or current price verification; disk, network, storage and local build resources are excluded. Retain this failed attempt in cumulative resource accounting.

The build/image outputs remain valid provenance, but this failed runtime gate does not admit batch03. Preserve the original validation prefix, artifacts and local receipts; any separately authorized corrected validation needs a fresh identity rather than overwriting or silently replaying this failure.
