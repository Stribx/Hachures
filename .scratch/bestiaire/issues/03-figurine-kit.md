# 03: The figurine kit

**What to build:** One kit of primitives for every figurine. Each species lists its props in `species.ron` (`carries`). A pure function turns a species into parts, and `EnemyLook` holds one parts list per species. The Chaîneur is dressed first: a slate peg-doll on a felt base with a bicorne, a chain coiled on its shoulder and the seal on its chest.

**Blocked by:** 01 (the owner's approval).

**Status:** done

Tests first (red, then green):
- [x] `kit::parts`: every hostile has exactly one Seal part; a non-hostile has none.
- [x] Every part lies inside a 0.9-cell footprint at the enemy scale, and above the base.
- [x] A species without `carries` in its RON still parses (serde default), and its parts are the bare body.

Build:
- [x] `map/species.rs`: the `Carry` enum and `carries: Vec<Carry>` (data only).
- [x] `combat/figure/kit.rs`: `Part` (mesh kind, transform, paint: coat, wood, felt, seal, metal) and `parts(shape, carries, hostile)`.
- [x] `combat/figure/mod.rs`: `EnemyLook` built from the kit, one parts list and one coat material per species; parts use `Hachure::fine()`; `shards.rs` keeps only the shards.
- [x] `spawn_figure` (from 02) spawns from the new parts.
- [x] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [x] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [x] Before/after parade shots (`HACHURES_PARADE=1`) are shown in the ticket's comments.

## Comments

- `combat/figure/`: `kit.rs` (pure: `Form`, `Paint`, `Role`, `Part`, `parts`), `look.rs` (`EnemyLook`: per species, mesh, transform, fine-hachure material, role), `spawn.rs` (`Body` pivot, `spawn_figure(.., scale)`). Root, then `Body` at the ground, then the parts; the boss passes `BOSS_SCALE` so it no longer sinks into the board. `FELT`, `LINEN`, `IRON` in `theme.rs`. Red run: the four kit tests failed on `todo!()`, then went green. `carries_default_to_none` parses a species without `carries`. 111 tests pass, and the balance table is unchanged.
- Shots: /tmp/claude-1000/-home-lair-Documents-perso-Hachures/b2d5565c-b945-4017-af1f-42e30343b0ac/scratchpad/shots/k03-parade-1-1.png (game camera), /tmp/claude-1000/-home-lair-Documents-perso-Hachures/b2d5565c-b945-4017-af1f-42e30343b0ac/scratchpad/shots/k03-row.png (`HACHURES_CAM=12.5,4,11,12.5,0.3,6`). The peg-dolls read: felt base, turned wooden head, coat in the people's colour, the seal on the chest. The boss box is too large for its cell (07 rebuilds it).
