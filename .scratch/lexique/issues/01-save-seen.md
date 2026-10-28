# 01: Save remembers what was seen

**Status:** done

Tests first (`campaign/save.rs` tests):
- [x] `discover("species.chaineur")` returns true once, then false.
- [x] A save round-trips `seen`.
- [x] A version-2 save with 1-1 won upgrades to version 3 with `seen` holding 1-1's tools and species.

Build: `seen: BTreeSet<String>` (serde default), `discover`, `VERSION = 3`, backfill in `upgrade`.
