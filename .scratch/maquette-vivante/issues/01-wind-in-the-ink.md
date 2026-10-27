# 01: Wind in the ink

**What to build:** The relief model breathes: the ink material can sway in a shared wind. Any inked surface can be given a wind amount; anchored at its base, it bends with the wind's direction, slow sways plus gusts that roll across the land. Its shadow sways with it, on desktop and in the browser. The first users are the trees around the board and on the Grand Plan, whose canopies now move while their trunks stay put.

**Blocked by:** None (can start immediately).

**Status:** done

- [x] The ink material has a wind amount per material; 0 (the default) leaves every existing surface exactly where it was.
- [x] One shared wind (direction, strength, gust speed) drives every swaying surface, so neighbouring trees move together, a gust visibly travelling across them.
- [x] The displacement grows with height above the model's base: trunks and the ground never move.
- [x] Shadows and the depth used by the creases (desktop) follow the swaying mesh.
- [x] Trees on the Grand Plan and around the board sway; rocks don't.
- [ ] The browser build compiles and runs it (WebGL2).
- [x] Two shots of the same view at different times (`HACHURES_AT=t1,t2`) show the motion, and nothing else differs.
- [x] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [x] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [x] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.

## Comments

- Every ink material now runs our own vertex stage (main pass and prepass, so shadows and the creases' depth follow); with wind 0 it puts every vertex where Bevy's would. `Swaying(amount)` on a model, like `Carved`, gives its ink some wind; the trees around the board and on the Grand Plan get it, rocks don't.
- The wind blows from the west-south-west; a gust rolls downwind across the land and each tree sways in its own time on top of it.
- Shots: /tmp/claude-1000/-home-lair-Documents-perso-Hachures/0c12f20b-002a-4d83-9aa8-d1079a5805fa/scratchpad/wind-1.png and wind-2.png (Grand Plan, 1.3 s apart; wind-diff.png shows only the trees and the water lines change), wb-pair.png (1-1, canopy and its shadow lean between 5 s and 6.5 s).
- **Open:** the browser build compiles, but no browser here can run the WebGL2 shader; check it in the final pass (ticket 12).
