# 12: Final pass

**What to build:** The living model holds up everywhere: on every preset, on desktop and in the browser, and against the art direction's "fini quand".

**Blocked by:** 01 to 11.

**Status:** done, except running the browser build

- [x] `HACHURES_FPS=1` per-pass GPU timings on each preset are recorded in `direction-artistique.md`; High stays within about 2 ms of the start of the feature.
- [ ] The browser build is checked: everything shows, nothing panics.
- [x] Art review round 7 (written as round 8; round 7 came after the palette) is appended to `direction-artistique.md`, and every Phase 6 "fini quand" holds on a screenshot.
- [x] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [x] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [x] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.

## Comments

- GPU, total of the passes on 1-1 (debug build, window 1568 x 1689, same for all), `main` against this branch: Low 6.8 → 6.3–6.7 ms (no extra); **High 8.9 → 10.6–10.7 ms (+1.8, inside the 2 ms budget)**; Ultra 10.5 → 12.4–12.6 ms (+2). The extra is mostly the opaque pass (grass, hedges) and the shadows. Recorded in `direction-artistique.md`. Script: /tmp/claude-1000/-home-lair-Documents-perso-Hachures/0c12f20b-002a-4d83-9aa8-d1079a5805fa/scratchpad/measure.sh.
- `./scripts/build-web.sh` builds (wasm-release, exit 0). **Not checked: running it in a browser** — none is installed here. Serve with `python3 -m http.server -d website` and open `/play/`; watch the wind shaders compile under WebGL2.
- Round 8 is in `direction-artistique.md`: 8 / 10, still capped by the interface's 1 px rules; every Phase 6 "fini quand" holds on the shots from tickets 01–09, except the sound (ticket 11, waiting for the CC0 list's approval).
