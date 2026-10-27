# 09: Daylight follows the waves

**What to build:** Each wave moves the sun a little later into the afternoon: the light warms and the shadows lengthen toward the last wave, then a new level starts back in the early afternoon. The change is slow and gentle, eased between waves.

**Blocked by:** 02 (the enluminure palette).

**Status:** done

- [x] A pure function gives the sun's direction and colour for a wave out of the level's total; tests cover the first and last wave.
- [x] The change eases over a few seconds when a wave starts, never a jump.
- [x] At the last wave the shadows are still short enough that no piece hides its neighbour.
- [x] Water glints follow the moved sun.
- [x] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [x] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [x] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.

## Comments

- `map::daylight::daylight(wave, waves)`: from the early-afternoon sun (61°) before the first wave down to 43° by the last, swinging a little west and warming toward `EVENING`. Tests: the first wave keeps the old sun, the last is lower (but over 35°) and warmer, every wave moves it on, also on a 3-wave level.
- `follow_the_waves` eases the `Sun` toward it with a 2 s time constant, so a wave start never jumps the light; it stops touching the light once there. Water glints follow, as the shader reads the sun.
- Seen in play at wave 1: /tmp/claude-1000/-home-lair-Documents-perso-Hachures/0c12f20b-002a-4d83-9aa8-d1079a5805fa/scratchpad/dl.png. The last waves' light is checked by the tests only: the harness can't jump to wave 12.
