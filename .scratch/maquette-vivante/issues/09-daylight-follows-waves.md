# 09: Daylight follows the waves

**What to build:** Each wave moves the sun a little later into the afternoon: the light warms and the shadows lengthen toward the last wave, then a new level starts back in the early afternoon. The change is slow and gentle, eased between waves.

**Blocked by:** 02 (the enluminure palette).

**Status:** ready-for-agent

- [ ] A pure function gives the sun's direction and colour for a wave out of the level's total; tests cover the first and last wave.
- [ ] The change eases over a few seconds when a wave starts, never a jump.
- [ ] At the last wave the shadows are still short enough that no piece hides its neighbour.
- [ ] Water glints follow the moved sun.
- [ ] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [ ] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [ ] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.
