# Shogi mutual-check invariant

The [mutual-check derivation](mutual-check-derivation.md) excludes a closed
repetition interval in which both sides check on every turn. It uses the
ordinary Shogi stock bounds, one king per side, legal complete actions and an
unattacked nonmoving king. Those conditions apply to admitted live roots,
including reduced diagrams and unknown prehistory. The broader structural
position API also supports analysis outside this live-root domain.

The argument reduces both sides to persistent bishop discovery blockers and
then excludes the possible geometry. The [supplement](mutual-check-supplement/README.md)
contains the finite calculation and the mixed-radius proof. The calculation
covers necessary geometry under the reduction assumptions.

If the rules library computes both continuous-check flags true in an admitted
history, it returns `GameError::MutualCheckInvariant` with candidate context.
The rules library rejects the candidate before commit and leaves the game
unscored. Canonical replay preserves the invariant diagnostic. The
[selected Shogi profile](shogi-profile.md) defines the ordinary one-sided
continuous-check result.
