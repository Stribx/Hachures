# 08: Rain and snow with the world events

**What to build:** The weather follows the world: rain slants in the wind and puddles ripple while la Crue is under way, and snow falls during le Gel and in world 2. The weather starts when the event does and stops when it withdraws, read from the existing event state.

**Blocked by:** 01 (wind in the ink).

**Status:** ready-for-agent

- [ ] Rain while la Crue is on, snow while le Gel is on and lightly throughout world 2; nothing otherwise.
- [ ] It slants with the shared wind direction.
- [ ] It's light enough that every tile and marker stays readable.
- [ ] Density follows the preset; off on Low.
- [ ] Two shots of the same view at different times (`HACHURES_AT=t1,t2`) show the motion, and nothing else differs.
- [ ] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [ ] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [ ] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.
