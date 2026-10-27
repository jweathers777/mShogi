# MSH-91 independent restricted reference/harness review

Verdict: PASS for the explicitly restricted board comparator; no outcome, search or strength admission.

Reviewer `/root/xiangqi_boundary_reviewer`, independent of fixture author, reference author and native builder. Date2026-10-27. Reviewed tools/xiangqi/{build_reference.py,pikafish_board_probe.cpp,reference.py,compare.py,test_reference.py}, retained patch, build/compiler/source/tree/license/binary manifest, failure and perft artifacts. Working baseline53e90a9 with task-scoped additions. Exact reviewed artifact hashes are in `harness-reviewed-sha256.json` beside this report.

## Direct checks

- Pinned Pikafish9d7833aa113626b3ea63ad83f49fb9b456bd5839/tree93fb0853d2d60dcfbb455b033b94907f4d572f61, clean isolated add-only harness patch. Build records Apple clang21.0.0/arm64/macOS27.0.1, exact flags/translation units, source/header hashes, GPL license hash, patch/harness/binary identity. Independently verified admitted binary hash `b60ae49a031272032aeb9263cc3a47307b0b81ccff77633c87eb85de9291c7b9`, harness37a8f59b4029f2019c082e80b6027ee09faa8bff5b5b2b5cb4feb9656506831e, patchba3c1d833efad6dc73a3ddf8f03a44a03e5a3e5b58dfda8a654a6e1afc4d9d8f.
- Read source operation and normalization: MoveList<ALL> (EVASIONS under check) is entire admitted legal set; do/undo compares original board, state bytes/pointer, FEN, keys and complete choices. No source history adjudicator/search/NNUE evaluator called. `nm -g` query found no exported rule_judge/Search/evaluate/Network/load_network symbols; supporting source/link inspection only, not sole evidence.
- Strict duplicate-aware JSON, shape/domain checks, exact 0–9 coordinate convention, bounded requests/responses/diagnostics/depth/walks and wall deadline; explicit reference-domain refusals. No product dependency or shipped binary.
- Independently executed `python3 tools/xiangqi/test_reference.py`:14/14 PASS, including malformed/duplicate/unsupported/invalid action, tampered binary, deliberate expected mismatch, timeout, output/diagnostic limits, process failure and descendant cleanup.
- Independently executed `python3 tools/xiangqi/compare.py --fixtures tests/fixtures/xiangqi/board.json --depth 2 --report /private/tmp/msh91-93-review/independent-reference-check.json`:105 fixtures,4 explicitly excluded structural cases,101 admitted cases,0 failures. No expected file mutation. Corrected reviewed corpus SHAa0724cf66703a63256efd77fe67313259ca8ce0062095155d85286d1f1b4ed4d. Inspected P001 independent depth3 result7/divide5+2 as well.
- Retained actual initial B38 mismatch, original hash, source-based correction and renewed independent freeze. Reference/native observations remain separate from independently expected facts. Mismatch demonstration does not rewrite inputs.

## Material findings resolved

1. Cleanup originally killed a process group only while its leader remained running. A retired leader could leave a descendant holding pipes. Reported to author; cleanup now attempts known-group kill even after leader exit, catching absent group. Executed failure sentinel confirms timeout and no delayed descendant marker.
2. Pinned `Position::do_move` lines562–568 implements foreign `rule60/check10` counter adjustment; `fen()` exposes that clock. This differs from native declared/raw no-capture plies and is not WXF authority. Direct query of preserved F002 checking cycle repeated6times confirms24noncaptureplies but foreign FEN counter20/fullmove13. Reported to author; comparison now excludes only this policy-specific counter field while retaining exact placement/side/fullmove comparison and both complete original FEN observations. Added executable sentinel passes. Native counter correctness remains a native-test obligation, not differential authority.

The two material findings are resolved without changing accepted native rules. Four structural General-threat/capture cases remain required native rejection tests, not silently absent conformance coverage. Neither comparator agreement nor bare-General perft proves WXF history/adjudication or real playing strength. Runtime board admission applies to the documented constrained domain and encoded bounds only.

## Final-artifact provenance finding — review held pending correction

Final board-admission artifact inspection found mutable-prefix aliasing in `compare.run_case`: `request['moves'] = moves or []` retains the caller's nonempty list, while the sequence loop appends further actions. Previously captured requests therefore change after their observations were obtained. The final length64 summary contained65actions while its FEN represented64plies; all nonempty earlier prefix requests were likewise rewritten. This affects recorded request provenance, not the actual queried board observations or independent fixture facts.

Reported to author/root. Required correction: copy the prefix into the retained request, add a focused immutability/prefix-length regression, regenerate final evidence with stable corrected tool hashes, and verify each request length/observed fullmove relationship. Initial fixture-only independent rerun and14sentinels remain applicable. Overall harness review PASS above is held until this finding is resolved; do not reuse the aliased sequence report as exact-context evidence.

## Renewed final evidence review — PASS

Author corrected request capture to `list(moves or [])`, omitted unused append after the final observation, added the actual-query immutability regression, preserved superseded report hash in `sequence-provenance-correction.json`, and regenerated stable evidence. Independently reran all15admission/failure sentinels: PASS. Independently checked all520sequence records: recorded prefix length equals observation ply; native-side token and fullmove number agree with that exact length; all8final prefixes are64plies. No fixture/sequence errors; full raw report SHA-256 `d87aa5db6663bcfb840e1f3f26c36236ac5a5e2451dec9838b73d6466adc31d5` matches retained compact manifest. Updated compare.py SHA256 `2a1e5910250fd19ed7424b07eb78c15f69e9a3f72f50aa01052b534a616a692c`, test_reference.py `f161ade713a85ec81639cf03b01e4b98f378934d93d45f46bafe2108fee39ee0`; adapter unchanged `843a804d64aa07df9756c107bb6a569a114be5ec4830b294747fc43843f4dece`.

All three material harness findings are resolved with focused regression or provenance checks. Final restricted-reference/harness verdict PASS reinstated. Earlier aliased report is explicitly superseded, not exact-context evidence.
