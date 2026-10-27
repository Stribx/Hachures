# 07: Birds and butterflies

**What to build:** Now and then a little flock of paper-and-ink birds crosses the sky over the board or the Grand Plan, and butterflies flit over the flower patches. Built in code, in the model's materials.

**Blocked by:** 04 (grass, flowers and wheat).

**Status:** done

- [x] Flocks cross at random intervals, from one edge to the other, above the pieces, and leave.
- [x] Butterflies stay over flower cells and flit; they never block a click (no picking).
- [x] None on the Low preset; a handful at most on High.
- [x] They stop with the game when it's paused.
- [x] Two shots of the same view at different times (`HACHURES_AT=t1,t2`) show the motion, and nothing else differs.
- [x] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [x] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [x] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.

## Comments

- `map::fauna`: `butterfly_homes(grid)` picks up to six cells in flower (tested: flowers only, capped, deterministic, none without flowers); `settle` sets them out again whenever the board is redrawn, `flutter` has them wander and beat their wings. `launch` sends a flock of three to seven ink birds in a V across the sky every 20 to 40 s, with the wind; `cross` flies it on and lets it go. All built in code, unpickable, none on Low, still while paused.
- Shots: /tmp/claude-1000/-home-lair-Documents-perso-Hachures/0c12f20b-002a-4d83-9aa8-d1079a5805fa/scratchpad/fa-1.png (butterflies over the meadow; the flock leaving at the top edge) and fa-diff.png.
