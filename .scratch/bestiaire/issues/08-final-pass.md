# 08: Final pass

**What to build:** The checks that close the bestiaire.

**Blocked by:** 01 to 07.

**Status:** done, except the browser run (ready-for-human)

- [x] GPU timings (`HACHURES_FPS=1`, fullscreen 2880×1800, High): 1-4 with no wave 14.95 ms, the boss wave 15.52 ms (+0.57 ms, an upper bound since the old enemies were not free). After small parts stopped casting shadows, the shadow pass is the same in both (1.60 ms). Low was not measured.
- [x] `./scripts/build-web.sh` compiles (wasm-release, 10 min). [ ] Running it in a browser is still to do: this machine has no Chrome.
- [x] Art review round 10 in the DA, from the parade, a world 1 wave, a world 2 wave (/tmp/claude-1000/-home-lair-Documents-perso-Hachures/b2d5565c-b945-4017-af1f-42e30343b0ac/scratchpad/shots/k08-w2.png) and the boss wave (/tmp/claude-1000/-home-lair-Documents-perso-Hachures/b2d5565c-b945-4017-af1f-42e30343b0ac/scratchpad/shots/k08-boss-2.png).
- [x] Arrival shots at the Moulin, and a death: /tmp/claude-1000/-home-lair-Documents-perso-Hachures/b2d5565c-b945-4017-af1f-42e30343b0ac/scratchpad/shots/k08-death-close.png (the health bar gone and the ghost still standing, then sépia drops flying).
- [x] Every figure reads on the fields (tested contrast), in greyscale (/tmp/claude-1000/-home-lair-Documents-perso-Hachures/b2d5565c-b945-4017-af1f-42e30343b0ac/scratchpad/shots/k08-grey.png) and at the Calque (/tmp/claude-1000/-home-lair-Documents-perso-Hachures/b2d5565c-b945-4017-af1f-42e30343b0ac/scratchpad/shots/k08-calque.png, `HACHURES_CALQUE=1`).
- [x] DA phase 7 is marked done and its "Tri" row says so. Its "fini quand" items are met on the parade, wave, greyscale and Calque shots.
- [x] CLAUDE.md's module table names `combat/figure/` and the dev tools list `HACHURES_PARADE`. No new glossary term.
- [x] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green; the balance table is unchanged.

## Comments

- Tests: 132 pass. fmt and clippy are clean, the structure test passes, and the balance table is unchanged through every ticket.
- GPU (`HACHURES_FPS=1`, debug build, 1-1, default preset, tiled window): an empty board totals 9.57 ms of passes, a wave 10.22 ms (+0.65 ms). The main opaque pass is +0.9 ms and the shadow pass +1.0 ms; the SSAO difference is noise. The old enemies cost something too, so the true cost over the old look is lower, but the +0.5 ms High budget is not proven. Cheapest cut if needed: no shadows from the small parts (seal, horns, bands, links).
- Arrivals: /tmp/claude-1000/-home-lair-Documents-perso-Hachures/b2d5565c-b945-4017-af1f-42e30343b0ac/scratchpad/shots/k08-arrive-strip.png (Chaîneurs walk onto the Moulin's cell and the Moulin goes 20, 18, 16).
- Follow-up: small parts (`casts_shadow`, under 0.065 kit units: seals, bands, horns, small heads), the chain links and the shards cast no shadow. Harness: `HACHURES_WAVE=n` starts wave n, and `HACHURES_CALQUE=1` turns le Calque on. 136 tests pass and the balance table is unchanged.
- Still open, for the owner: run the web build in a browser (no Chrome on this machine).
- Seen in passing, out of scope: le Calque's label shows "Chaîneur inf (but à inf)" on a water cell, so a raw `inf` reaches the player.
