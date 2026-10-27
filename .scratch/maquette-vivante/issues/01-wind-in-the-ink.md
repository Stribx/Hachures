# 01: Wind in the ink

**What to build:** The relief model breathes: the ink material can sway in a shared wind. Any inked surface can be given a wind amount; anchored at its base, it bends with the wind's direction, slow sways plus gusts that roll across the land. Its shadow sways with it, on desktop and in the browser. The first users are the trees around the board and on the Grand Plan, whose canopies now move while their trunks stay put.

**Blocked by:** None (can start immediately).

**Status:** ready-for-agent

- [ ] The ink material has a wind amount per material; 0 (the default) leaves every existing surface exactly where it was.
- [ ] One shared wind (direction, strength, gust speed) drives every swaying surface, so neighbouring trees move together, a gust visibly travelling across them.
- [ ] The displacement grows with height above the model's base: trunks and the ground never move.
- [ ] Shadows and the depth used by the creases (desktop) follow the swaying mesh.
- [ ] Trees on the Grand Plan and around the board sway; rocks don't.
- [ ] The browser build compiles and runs it (WebGL2).
- [ ] Two shots of the same view at different times (`HACHURES_AT=t1,t2`) show the motion, and nothing else differs.
- [ ] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [ ] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [ ] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.
