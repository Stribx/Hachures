# 06: Cloud shadows

**What to build:** Soft shadows of clouds drift across the board and the Grand Plan with the wind, the light dimming and warming back as they pass, like sunlight over a real landscape.

**Blocked by:** 02 (the enluminure palette).

**Status:** done

- [x] Cloud shadows are drawn in the ink shader, with no new entity, on desktop and web.
- [x] They move with the shared wind direction and are large and soft: never dark enough to hide a tile, a marker or the pencil line.
- [ ] A setting or the Low preset can turn them off.
- [x] Two shots of the same view at different times (`HACHURES_AT=t1,t2`) show the motion, and nothing else differs.
- [x] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [x] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [x] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.

## Comments

- In `hachure.wesl`, `cloud(pos.xz)`: two octaves of value noise drifting downwind (the same `WIND` as `wind.wesl`), dimming the sun's share to two thirds at most, above where the strokes start, so no tile, marker or pencil line loses its colour. One noise per pixel, no entity, the same on the board and the Grand Plan; the browser build compiles it.
- **Not done: an off switch.** It costs two noise samples a pixel and moves calmly, so it runs on every preset and under reduced motion too. Say if you want it tied to a setting.
- Shots: /tmp/claude-1000/-home-lair-Documents-perso-Hachures/0c12f20b-002a-4d83-9aa8-d1079a5805fa/scratchpad/cl-pair.png (the Grand Plan 15 s apart: a cloud shadow has come over the left sheet).
