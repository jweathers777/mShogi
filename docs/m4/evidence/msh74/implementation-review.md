# MSH-74 independent implementation review

2026-10-26; fresh delegated reviewer `msh74_implementation_review`, reviewing
baseline `570ef0e` through the working MSH-74 boundary. Method: source/diff review,
trace normal ProductApp → hook → strict Tauri entry points → native controller,
file admission/archive and observation projection; run focused tests. This is
separate from visual review and packaged platform interaction.

Material findings resolved before the review checkpoint:

- Effect-local teardown retires StrictMode's first polling/listener promises.
- A delayed clean Save receipt cannot authorize discarding newer dirty state;
  the caller receives the latest accepted same-document snapshot, or no authority.
- Confirmation dialogs make the surrounding product inert and check the complete
  native document/version/revision/selection token.
- Selecting a hand piece preserves a board Tab entry; arrow navigation skips
  hand pieces without native choices.
- Generic cancelled work says Search stopped; the UI does not infer that an
  engine-turn cancellation was analysis from turn or paused state.
- Dialog focus capture precedes inert application and restores after removal.

Independent execution: product board/app/hook tests 22 passed at initial review;
updated hook suite 7 passed, including deferred StrictMode teardown; native
session `product_` tests 5 passed; desktop host tests 7 passed with desktop Rust
1.90; diff whitespace check passed. Initial host invocation from repository root
failed because Rust1.89 is below the host requirement; rerun from
`desktop/src-tauri` correctly selected the recorded1.90 toolchain and passed.

Reviewer reports no unresolved material implementation finding in these reviewed
boundaries. This does not establish owner visual approval, GUI interaction or
Windows/Linux success. Those are separate evidence gates.

Follow-up independent review of the delta against `41a3009`: disappearance of
Resume/Cancel/Retry/Return to live now restores focus to the native status after
snapshot commit, and Dialog explicitly prioritizes Cancel. The Ubuntu CI run
found a racy test precondition: public `command_with_receipt` can legitimately
accept its worker completion while constructing its returned snapshot. The test
now queues the same real immutable candidate without polling and explicitly
asserts its receipt is pending before replacement. It still requires retirement
with an error and rejects resurrection through later legacy/shared polls. No
product rule or admission behavior changed. Reviewer independently reran this
native test (passed), ShogiApp/ProductApp17 tests (passed), and diff check (passed).
