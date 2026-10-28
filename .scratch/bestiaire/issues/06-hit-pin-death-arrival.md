# 06: Hit, pin, death, arrival

**What to build:** The figurines answer the towers and leave the board as pieces of the model:
- a hit makes them jolt (tilt back and squash, gone in 0.2 s);
- the Harpon makes them stagger in place;
- on death they tip over, the seal cracks into sépia droplets and the figure breaks into shards of its coat, with a sépia blot that fades on the ground;
- at the Cible they sink into the board like a pin pulled out;
- a Fuyard at the Refuge gives a little hop and fades.

**Blocked by:** 05.

**Status:** done

Tests first:
- [x] `jolt(t)` is at its peak at 0 and zero from 0.2 s.
- [x] ECS: a defeated enemy despawns on the same frame as before, and visual-only shards and a blot exist.
- [x] ECS: an enemy reaching its goal despawns on the same frame as before, and an exit ghost plays, then goes.

Build:
- [x] The jolt and the stagger in `animate` (`Stride` keeps the hit timer).
- [x] `shatter` takes the seal and coat colours; the blot fades.
- [x] Exit ghosts: a copy of the parts with no `Enemy`, which sinks or hops away.
- [ ] Shots at the moment of a death and of an arrival (`HACHURES_AT`): not caught yet (see comments), moved to 08.
- [x] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [x] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [x] Parade shots: unchanged by this ticket (only hits, pins, deaths and arrivals changed).

## Comments

- `figure/jolt.rs` (`jolt`, `stagger`), `figure/exit.rs` (`Exit` ghosts: `Fall`, `Sink`, `Hop`; `exit_pose`, `exit_life`, `leave`, `exits`). `Stride` notices the HP drop itself, so `strike` is untouched. `defeat` leaves a `Fall` ghost that breaks into 6 coat shards, 4 sépia drops and a blot that shrinks away; bornes keep the dark shards. `walk` leaves a `Sink` ghost for a hostile at its goal and a `Hop` ghost for an ally. The enemy entity is still despawned on the same frame, and the balance table is unchanged. Red run: all four new tests failed on stubs; the old 8 dark cubes showed in place of a ghost. 126 tests green.
- Shots: four `HACHURES_DEMO=1 HACHURES_WAVE=1` runs did not catch the first kill. The spawn is at the board's left edge, under the demo's barricade wall, and the arbalète kills before t = 2 s. The ECS tests cover the order: despawn, then ghost, then shards and blot. The visual check goes to 08, on the boss level and an open board.
