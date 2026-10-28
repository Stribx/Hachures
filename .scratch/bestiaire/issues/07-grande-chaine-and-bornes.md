# 07: La Grande Chaîne and the bornes

**What to build:** World 1's boss, assembled from the kit (bible §9 and its card): a walnut cart, two oxen (the Harde body with horns, `Trot`), the yoke, Gast standing on the box, the sighting lantern up front, and a 20-link surveying chain dragging along the path it has driven. In phase 3 (*rompue*) the oxen panic and the yoke knocks about. The bornes become small cream limestone markers with a seal-red cap.

**Blocked by:** 04, 05.

**Status:** done

Tests first:
- [x] `chain::trail(history, links, spacing)`: 20 links, each `spacing` from the next along the path, the first at the cart's rear; with a history shorter than the chain, the links bunch up at the start.
- [x] ECS: the switch to `grande_chaine_rompue` changes the oxen's gait.

Build:
- [x] The boss parts, with the lantern and the yoke as separate parts.
- [x] The chain follows the cart's recorded path.
- [x] The borne markers.
- [x] Shots of the boss: on the parade (phase 1 and phase 3 look the same; phase 3 shows in its gait). The chain is only spawned with a real boss, so it is shot in 08 on 1-4.
- [x] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [x] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [x] Before/after parade shots (`HACHURES_PARADE=1`) are shown in the ticket's comments.

## Comments

- The `Carry` props `Boeufs`, `Joug`, `Gast` and `Visee`, new colours `HIDE`, `SLATE`, `LAMP` and `LIMESTONE`, `figure/chain.rs` (pure `trail`, the `Rope` history, 20 `Link`s, `drag_chain`), `Gait::Panique` from (Chariot, Ombre), and bornes as a limestone block with a seal-red cap. Red run: six new tests failed on stubs. 132 tests are green and the balance table is unchanged.
- Review on the first shot: a one-cell black slab dwarfed small oxen. The boss cart now has its own lower box (`BOSS_BOX` 0.14; the convoy keeps 0.22), a warmer walnut (0.36, 0.24, 0.14), and Gast and the lantern stand on the box's real top (`crown`).
- Follow-up: the legs now pair diagonally for a trot (`Part.side` set where each leg is made, so it survives the shift that put each ox's legs on one side), and the yoke (`Role::Yoke`) knocks under `Gait::Panique` only. Tests: `legs_alternate_per_animal` and `the_yoke_knocks_only_in_panic`, both seen red first.
- The boss wave on 1-4 (`HACHURES_WAVE=8 HACHURES_LEVEL=4`): /tmp/claude-1000/-home-lair-Documents-perso-Hachures/b2d5565c-b945-4017-af1f-42e30343b0ac/scratchpad/shots/k08-boss-2.png (game camera) and /tmp/claude-1000/-home-lair-Documents-perso-Hachures/b2d5565c-b945-4017-af1f-42e30343b0ac/scratchpad/shots/k08-boss-3-crop.png (the chain trailing over the Chaussée it lays, the cart crossing the ford).
- Shots: /tmp/claude-1000/-home-lair-Documents-perso-Hachures/b2d5565c-b945-4017-af1f-42e30343b0ac/scratchpad/shots/k07-boss.png (before), /tmp/claude-1000/-home-lair-Documents-perso-Hachures/b2d5565c-b945-4017-af1f-42e30343b0ac/scratchpad/shots/k07-boss-2.png (after). The parade stands both boss species side by side, so their two-cell carts overlap; a level only ever has one.
