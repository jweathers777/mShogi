# Functional Tenjiku search and supervised results

MSH-44 working checkpoint, built from84d7aa8. Both immutable whole profiles use
`mshogi.tenjiku.material.v1` and `mshogi.tenjiku.full-width.v1`. The native Game
remains the sole rules, effect, history and outcome authority. There is no
transposition cache or separate legality implementation. Each search reconstructs
its private full-history owner, submits complete actions and uses native undo;
restoration checks compare semantic identity and exact snapshot while runtime
authority advances. Cancellation unwinds admitted work. NoBoardActions is
unscored even at an evaluation cutoff; no mate/stalemate rule is invented.

The transparent untrained evaluator totals all44 native movement faces, preserving
original/promoted identity in Game. Only native conclusions enter the1,000,000
terminal band, adjusted by board-action distance. Scores always favor the captured
root side. This is functional play, without a competitive-strength claim.

Interactive limits are depth1/1024 total visits. Ordinary analysis permits depth1–2
and1–2000 visits. The protocol additionally accepts zero visits as honest incomplete
work. Full-width iterations publish only completed results. A later interrupted
iteration can retain a previous completed result, including its actual depth/PV;
without a completed iteration there is no fallback action, score or outcome.
Native admission/resource errors are separate unscored failures.

The existing version4 supervised family now carries TenjikuResult with native
complete-action PV, evaluator/search identity, perspective, completed depth,
all attempted visits and stop reason. Exact context equality binds request,
session, purpose, selection, process generation, limits, profile and complete
history. Chess Full remains unsupported. Polling performs shape/context checks;
the existing bounded admission worker reconstructs the exact prefix, re-admits
all PV actions and checks its actual leaf score/conclusion before publication.
Returned reconstruction/PV work is charged to the existing160M document budget.
EngineTurn observations precede their exact native Play in the journal; analysis
adds only an observation. Native and desktop capacities/deadlines are unchanged.

## Initial release measurements

macOS27.0.1/26A434 arm64, Rust/Cargo1.99.1. These are local timing observations,
not portable latency guarantees. Commands:

```
cargo run --release --locked -p mshogi-search --example tenjiku_work -- 1 1024
cargo run --release --locked -p mshogi-search --example tenjiku_work -- 1 1024 extended
cargo run --release --locked -p mshogi-search --example tenjiku_work -- 2 2000
```

| Root | Profile | Native choices | Visits | Completed depth | Milliseconds |
| --- | --- | ---: | ---: | ---: | ---: |
| Standard | Adams |70|71|1|265.057|
| Standard | Muller |74|75|1|278.007|
| Demon tactical diagram | Adams |616|617|1|624.857|
| Demon tactical diagram | Muller |616|617|1|630.506|
| Area/Hawk diagram | Adams |716|717|1|323.201|
| Area/Hawk diagram | Muller |716|717|1|320.698|
| Retained128 native history | Adams |212|213|1|1319.073|
| Retained128 native history | Muller |215|216|1|1486.816|

Depth2/2000 exhausted the node budget after completing depth1: standard
Adams7508.417ms/Muller7501.347ms; Demon tactical1700.997/1734.935ms.
The example asserts exact caller record/context preservation. These measurements
justify different limits from Chu/Tori without bypassing native transactions.

Native search10 focused tests cover independently solved minimax, complete/partial
iterations, native stop/unwind, no-action roots/cutoffs, same-board history,
profile-sensitive Demon leaves, ordered multi-royal capture and capacity failure.
An independent review of the engine result boundary found no correctness defect;
the reviewer separately ran6 protocol/routing tests. Further checkpoint evidence
and exact source receipts belong in the single MSH-44 verification record.
