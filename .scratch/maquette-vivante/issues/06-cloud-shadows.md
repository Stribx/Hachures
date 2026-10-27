# 06: Cloud shadows

**What to build:** Soft shadows of clouds drift across the board and the Grand Plan with the wind, the light dimming and warming back as they pass, like sunlight over a real landscape.

**Blocked by:** 02 (the enluminure palette).

**Status:** ready-for-agent

- [ ] Cloud shadows are drawn in the ink shader, with no new entity, on desktop and web.
- [ ] They move with the shared wind direction and are large and soft: never dark enough to hide a tile, a marker or the pencil line.
- [ ] A setting or the Low preset can turn them off.
- [ ] Two shots of the same view at different times (`HACHURES_AT=t1,t2`) show the motion, and nothing else differs.
- [ ] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [ ] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [ ] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.
