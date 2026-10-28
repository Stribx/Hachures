# 02: The parade

**What to build:** A dev switch, `HACHURES_PARADE=1`, that stands one still figure of every species of the level in a row on the board, for art review. Baseline shots of today's enemies.

**Blocked by:** none.

**Status:** done

- [x] `combat::spawn_figure` spawns only the visual of a species; `spawn_enemies` uses it and behaves exactly as before.
- [x] `shot/parade.rs`: pure `parade_cells(grid, n)` (tested: n distinct plain Sol cells, consecutive in one row when possible, deterministic) and the system that stands the figures there (`LevelEntity`, never `Enemy`).
- [x] `HACHURES_PARADE=1` is in the `shot` doc line and in CLAUDE.md's "Dev tools".
- [x] Baseline shots: the parade on 1-1 and on a world 2 level, and a 1-1 wave (`HACHURES_WAVE=1 HACHURES_AT=6,7`).
- [x] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [x] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [x] No "after" shot: this ticket only adds the harness. The baselines below are the "before" for 03 and 04.

## Comments

- `combat::enemies::spawn_figure(commands, look, species, index, at) -> Entity` and `BOSS_SCALE`. `spawn_enemies` inserts `LevelEntity` and `Enemy` on top; the balance table is unchanged. `shot.rs` became `shot/mod.rs` and `shot/parade.rs`. `parade_cells` is tested: distinct free Sol cells, deterministic, a run in the row nearest the middle, and a reading-order fallback. The parade shows every species in `species.ron`, not only the level's own, so one shot reviews them all.
- Baseline shots (today's primitives): /tmp/claude-1000/-home-lair-Documents-perso-Hachures/b2d5565c-b945-4017-af1f-42e30343b0ac/scratchpad/shots/base-parade-1-1.png, /tmp/claude-1000/-home-lair-Documents-perso-Hachures/b2d5565c-b945-4017-af1f-42e30343b0ac/scratchpad/shots/base-parade-2-1.png, /tmp/claude-1000/-home-lair-Documents-perso-Hachures/b2d5565c-b945-4017-af1f-42e30343b0ac/scratchpad/shots/base-wave-1-1-1.png, /tmp/claude-1000/-home-lair-Documents-perso-Hachures/b2d5565c-b945-4017-af1f-42e30343b0ac/scratchpad/shots/base-wave-1-1-2.png. On the default camera the row's right end, with the boss, is cut by the frame: 03 frames it with `HACHURES_CAM`.
