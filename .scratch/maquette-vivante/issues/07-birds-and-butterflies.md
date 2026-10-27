# 07: Birds and butterflies

**What to build:** Now and then a little flock of paper-and-ink birds crosses the sky over the board or the Grand Plan, and butterflies flit over the flower patches. Built in code, in the model's materials.

**Blocked by:** 04 (grass, flowers and wheat).

**Status:** ready-for-agent

- [ ] Flocks cross at random intervals, from one edge to the other, above the pieces, and leave.
- [ ] Butterflies stay over flower cells and flit; they never block a click (no picking).
- [ ] None on the Low preset; a handful at most on High.
- [ ] They stop with the game when it's paused.
- [ ] Two shots of the same view at different times (`HACHURES_AT=t1,t2`) show the motion, and nothing else differs.
- [ ] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [ ] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [ ] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.
