# Independent review and verification

Research date:4November2026. Integration target:`codex/rewrite`.
Base:`8676cbb774c5837b839772a731868a720aaab43f`.

## Independent read-only assessment

Reviewer:`/root/access_review`, separate agent, **ACCEPT: no remaining material factual findings**.
Read the complete report and bilingual requests; independently inspected live National Archives metadata and reuse policy, Kyoto integrated catalogue, Harvard NIJL metadata, official institutional procedures/contacts/rates, Adams Appendix A and both administrative PDFs. Kyoto dedicated OPAC/archive blocks were observed directly. No repository edits, institutional contact, submissions or costs by the reviewer.

Resolved findings:

- Correct Adams's journal title to **Kapitan**; preserve the older handoff as dated evidence.
- Distinguish TNM onsite ¥30/¥100 from library-mediated remote ¥60/¥200; retain item-specific eligibility/fee uncertainty.

The reviewer re-read both corrections and accepted the complete request drafts. No Japanese Tenjiku pages, historical rule resolution, independently dated manuscript or copy identity for Banaschak is claimed.

## Reviewed artifact identity

These hashes identify the reviewed report/drafts/manifest without inventing a future commit hash. `report.md` below is delivered as `../tenjiku-historical-access.md`; the other filenames are unchanged.

| Reviewed artifact | SHA-256 |
| --- | --- |
| `report.md` | `721845f8ba94a8619e6332079a70d55031cdf8db3141a77e66a42fc6c691367d` |
| `requests.md` | `158aaa1e50b67cbeb998062883f4c8ec86d371a5ad25badc0edd1bd38088ed80` |
| `provenance.json` | `09c26d7f818b0f9b3b878692e306227d2241d5cd8ce103760963a52a43bec26d` |

## Verification

Environment:macOS host; bundled runtime26.1103.10131, system Python3 for JSON/hash/path checks; bundled Poppler for rendering. No platform software behavior is being certified.

- `python3 /private/tmp/tenjiku-historical-access/verify.py`: PASS, checks JSON parsing, downloaded-original SHA-256 and byte sizes, repository-relative links, five fee calculations, and empty historical-Tenjiku acquisition list. Initial path check ran before this review file existed and flagged its pending link; final check includes this file.
- `git diff --cached --check`: PASS for staged research artifacts; final committed diff checked again before push.
- Downloaded Adams PDF hash matches earlier source acquisition. Entire PDF177–179 rendered with `pdftoppm -f 177 -l 179 -scale-to 1700 -png ...` and inspected. Fee PDFs each rendered/read in full; reviewer independently verified hashes.
- Original PDFs/HTML are retained unchanged in the primary checkout's Git-ignored `originals/` directory. Manifest distinguishes raw-file hashes from browser-only observations and failed fetches.
- Scope review: only new historical-access report/directory; no MSH-83 record/profile/fixtures, dependencies, other tasks or production files.

Required CI jobs:**none**. Native game tests, CI and GUI campaigns:**not run, not required** for this investigation. No emails/forms/orders/account registrations/payments. The source tree and originals are rechecked at integration; actual delivery commit is available in Git history for these paths.

## Recorded integration

Reviewed research integrated as `ee9aaa8172bf26f0f2b7d08fc62040a5dbbe911c` on `codex/rewrite`, preserving the other worker's preceding modern-profile commit `1136886`. `git diff be93699 ee9aaa8 -- docs/m4/research/tenjiku-historical-access.md docs/m4/research/tenjiku-historical-access` is empty: integrated research bytes match the reviewed isolated source. At `ee9aaa8`, the validation script run against the primary checkout passed7original-file hashes/sizes,9relative links and5fee illustrations; committed diff whitespace check passed. Working tree clean, originals ignored and retained. This final integration note changes no research finding or reviewed artifact hash.

Push of research commits `ee9aaa8` and `ccef16e` to `origin/codex/rewrite` was confirmed. Cleanup limitation: the app-managed archive operation refused `/Users/johnweathers/.codex/worktrees/tenjiku-historical-access/mShogi` with “The task's primary checkout cannot be removed”; attaching the normal integration checkout was also refused because it is not managed. The isolated worktree is clean, contains no needed untracked/ignored originals, and its research changes are integrated. It remains solely as this chat's app-bound primary/recoverable checkout until the app permits detachment/archive; no research remains active there. Do not remove another worker's checkout or bypass the app protection. Original source bytes remain in the primary integration checkout, independently of this retained worktree.
