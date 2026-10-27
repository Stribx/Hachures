# 12: Final pass

**What to build:** The living model holds up everywhere: on every preset, on desktop and in the browser, and against the art direction's "fini quand".

**Blocked by:** 01 to 11.

**Status:** ready-for-agent

- [ ] `HACHURES_FPS=1` per-pass GPU timings on each preset are recorded in `direction-artistique.md`; High stays within about 2 ms of the start of the feature.
- [ ] The browser build is checked: everything shows, nothing panics.
- [ ] Art review round 7 is appended to `direction-artistique.md`, and every Phase 6 "fini quand" holds on a screenshot.
- [ ] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [ ] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [ ] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.
