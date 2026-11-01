# Acquired source evidence and aliases

`historical-images.json` is the exact copy of the research cache's
`acquisition-images.json`; `historical-cache.json` preserves the researcher's
original cache names/hashes. `historical-iiif.json` is the original public IIIF
manifest. Absolute `/private/tmp` filenames describe the actual acquisition
cache, not portable repository file paths. Public URLs and SHA256 permit lawful
reacquisition; do not treat a vanished cache as an unread-source success.

Three unaltered selected images4/7/9 are retained beside these manifests under
shorter filenames. Attribution: **『禽象戲圖解』（東北大学附属図書館所蔵）**,
狩野文庫5/17701/1, record10010000019229. [Original institutional record](https://touda.tohoku.ac.jp/portal/item/10010000019229),
[CC BY4.0](https://creativecommons.org/licenses/by/4.0/),
[institutional conditions](https://touda.tohoku.ac.jp/collection/database/library/use_of_materials).
Images are unchanged institution-served JPEG derivatives, not TIFF masters.

`modern-manifest.json` distinguishes actual public source files from rendered/
extracted derivatives. `kanare-manifest.json` preserves actual HTTP status,
bytes/hash and retrieval time for three publisher PDFs. These modern copyrighted
PDFs/HTML are not reproduced here. Their original URLs/locators and read limits
are in the source ledger and researchers' reports. Neither report is a complete
scholarly translation or authority to adopt unresolved rules.

## Current modern planning evidence

The earlier manifests/reports and `verify-research.py` retain their historical
scope (the old checker targets the old unratified working tree, not current task
completion). Current static verification is `verify-modern.py` with
`modern-validation.json`; it verifies selected-profile fixture/task integrity,
not native conformance. The immutable2026-10-31 checkpoint lives in
`../archive/2026-10-31`, including its original validation result. Contemporary
source/cross-check and comparator limits are in current sources/profile/review
and comparator-audit documents. No current rule depends on unread historical
books. No modern copyright PDF was added to Git.

MSH-85's later planning refinement is checked by `verify-native-plan.py` and
`native-plan-validation.json`. The earlier modern checker intentionally protects
its earlier task-dependency snapshot; its historical success is not a claim that
it validates later component additions. Current native planning verifies the
profile and all103 fixture bytes remain unchanged while85 gains the four actual
component dependencies. Neither checker executes a native Tori rules kernel.
