# 11: Ambient sounds in the game

**What to build:** The land sounds alive: a wind bed that swells with the gusts, birds over the fields, rain during la Crue, snow wind in world 2, water near the river and the Moulin, all under an ambient volume setting.

**Blocked by:** 10 (the CC0 shortlist).

**Status:** ready-for-agent

- [ ] Only the approved files are added, each with a row in `CREDITS.md`, and `build-web.sh` copies their folder.
- [ ] Each loop starts and stops with its condition (event, world, place), with fades, never a cut.
- [ ] An ambient volume setting is saved with the other settings and offered in every language.
- [ ] The browser build plays them after the first click.
- [ ] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [ ] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [ ] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.
